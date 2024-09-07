use core::{
    cell::{RefCell, RefMut},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use arduino_hal::pac::TC0;
use fugit::{Duration, Instant};
use heapless::{binary_heap::Min, BinaryHeap};
use portable_atomic::{AtomicU32, Ordering};

use avr_device::interrupt::Mutex;

//use crate::executor::{wake_task, ExtWaker};

type TickInstant = Instant<u32, 1, 32768>;
type TickDuration = Duration<u32, 1, 32768>;

const MAX_DEADLINES: usize = 8;
static WAKE_DEADLINES: Mutex<RefCell<BinaryHeap<(u64, usize), Min, MAX_DEADLINES>>> =
    Mutex::new(RefCell::new(BinaryHeap::new()));

/// Deadlines can only be scheduled in a COMPARE register if they fall within
/// the current overflow-cycle/epoch, and also are not too close to the current
/// counter value. (see nRF52833 Product Specification section 6.20.7)
// fn schedule_wakeup(
//     mut rm_deadlines: RefMut<BinaryHeap<(u64, usize), Min, MAX_DEADLINES>>,
//     mut rm_rtc: RefMut<Option<TC0>>,
// ) {
//     let rtc = rm_rtc.as_mut().unwrap();
//     while let Some((deadline, task_id)) = rm_deadlines.peek() {
//         let ovf_count = (*deadline >> 24) as u32;
//         if ovf_count == TICKER.ovf_count.load(Ordering::Relaxed) {
//             let counter = (*deadline & 0xFF_FF_FF) as u32;
//             if counter > (rtc.get_counter() + 1) {
//                 rtc.set_compare(RtcCompareReg::Compare0, counter).ok();
//                 rtc.enable_event(RtcInterrupt::Compare0);
//             } else {
//                 // Wake now if it's too close or already past,
//                 // then try again with the next available deadline
//                 wake_task(*task_id);
//                 rm_deadlines.pop();
//                 continue;
//             }
//         }
//         break;
//     }
//     if rm_deadlines.is_empty() {
//         rtc.disable_event(RtcInterrupt::Compare0);
//     }
// }

// enum TimerState {
//     Init,
//     Wait,
// }

// pub struct Timer {
//     end_time: TickInstant,
//     state: TimerState,
// }

// impl Timer {
//     pub fn new(duration: TickDuration) -> Self {
//         Self {
//             end_time: Ticker::now() + duration,
//             state: TimerState::Init,
//         }
//     }

//     /// Registration places the deadline & its task_id onto a `BinaryHeap`, and
//     /// then will attempt to schedule it (via COMPARE0) if it's earlier than
//     /// the current deadline.
//     fn register(&self, task_id: usize) {
//         let new_deadline = self.end_time.ticks();
//         critical_section::with(|cs| {
//             let mut rm_deadlines = WAKE_DEADLINES.borrow_ref_mut(cs);
//             let is_earliest = if let Some((next_deadline, _)) = rm_deadlines.peek() {
//                 new_deadline < *next_deadline
//             } else {
//                 true
//             };
//             if rm_deadlines.push((new_deadline, task_id)).is_err() {
//                 // Dropping a deadline in this system can be Very Bad:
//                 //  - In the LED task, the LED will stop updating, but may come
//                 //    back to life on a button press...
//                 //  - In a button task, it will never wake again
//                 // `panic` to raise awareness of the issue during development
//                 panic!("Deadline dropped for task {}!", task_id);
//             }
//             // schedule now if its the earliest
//             if is_earliest {
//                 schedule_wakeup(rm_deadlines, TICKER.rtc.borrow_ref_mut(cs));
//             }
//         });
//     }
// }

// impl Future for Timer {
//     type Output = ();
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         match self.state {
//             TimerState::Init => {
//                 self.register(cx.waker().task_id());
//                 self.state = TimerState::Wait;
//                 Poll::Pending
//             }
//             TimerState::Wait => {
//                 if Ticker::now() >= self.end_time {
//                     Poll::Ready(())
//                 } else {
//                     Poll::Pending
//                 }
//             }
//         }
//     }
// }

// pub async fn delay(duration: TickDuration) {
//     Timer::new(duration).await;
// }

static TICKER: Ticker = Ticker {
    ovf_count: AtomicU32::new(0),
    timer: Mutex::new(RefCell::new(None)),
};

/// Keeps track of time for the system using RTC0, which ticks away at a rate
/// of 32,768/sec using a low-power oscillator that runs even when the core is
/// powered down.
pub struct Ticker {
    ovf_count: AtomicU32,
    timer: Mutex<RefCell<Option<TC0>>>,
}

impl Ticker {
    /// Called on startup to get RTC0 going, then hoists the HAL representation
    /// of RTC0 into the `static TICKER`, where it can be accessed by the
    /// interrupt handler function or any `Timer` instance.
    pub fn init(timer: TC0) {
        // overflow interrupt enable
        timer.timsk0.write(|w| w.toie0().set_bit());
        timer.tccr0b.write(|w| w.cs0().direct());
        // Put the  timer into place
        avr_device::interrupt::free(|cs| {
            TICKER.timer.borrow(cs).replace(Some(timer));
        });
    }
    // Get the current time, which is a combination of:
    // /// value are collected during the same overflow-cycle.
    pub fn now() -> TickInstant {
        let ticks = TICKER.ovf_count.load(Ordering::SeqCst);
        TickInstant::from_ticks(ticks)
    }
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_OVF() {
    avr_device::interrupt::free(|cs| {
        if let Some(rm_timer) = &mut *TICKER.timer.borrow(cs).borrow_mut() {
            TICKER.ovf_count.fetch_add(1, Ordering::Relaxed);
        }
    });
}

// #[interrupt]
// fn RTC0() {
//     critical_section::with(|cs| {
//         let mut rm_rtc = TICKER.rtc.borrow_ref_mut(cs);
//         let rtc = rm_rtc.as_mut().unwrap();
//         if rtc.is_event_triggered(RtcInterrupt::Overflow) {
//             rtc.reset_event(RtcInterrupt::Overflow);
//             TICKER.ovf_count.fetch_add(1, Ordering::Relaxed);
//         }
//         if rtc.is_event_triggered(RtcInterrupt::Compare0) {
//             rtc.reset_event(RtcInterrupt::Compare0);
//         }

//         // For OVF & COMPARE0 events, schedule the next wakeup. This should also
//         // kill enough clock cycles to allow the event flags to clear.
//         // (see nRF52833 Product Specification section 6.1.8)
//         schedule_wakeup(WAKE_DEADLINES.borrow_ref_mut(cs), rm_rtc);
//     });
// }
