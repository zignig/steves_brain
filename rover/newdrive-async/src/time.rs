/// Async executor for avr , timer queue
/// Stolen from https://github.com/therustybits/zero-to-async
/// chapter 6 and converted. 


//TODO Should convert to timer1 (16bit)
// and use the overflow and value interuppt.

use core::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use arduino_hal::pac::TC0;
use fugit::{Duration, Instant};
use heapless::{binary_heap::Min, BinaryHeap};
use portable_atomic::{AtomicU32, Ordering};
use avr_device::interrupt::Mutex;

use crate::executor::{wake_task, ExtWaker};


//use crate::executor::{wake_task, ExtWaker};
// fugit does ticks -> millis in compile time
// for prescale_64
pub type TickInstant = Instant<u32, 1, 984>;
pub type TickDuration = Duration<u32, 1, 984>;

const MAX_DEADLINES: usize = 15;
static WAKE_DEADLINES: Mutex<RefCell<BinaryHeap<(u32, usize), Min, MAX_DEADLINES>>> =
    Mutex::new(RefCell::new(BinaryHeap::new()));

enum TimerState {
    Init,
    Wait,
}

pub struct Timer {
    end_time: TickInstant,
    state: TimerState,
}

impl Timer {
    pub fn new(duration: TickDuration) -> Self {
        Self {
            end_time: Ticker::now() + duration,
            state: TimerState::Init,
        }
    }

    // Insert the timer into the heap
    pub fn register(&self, task_id: usize) {
        let new_deadline = self.end_time.ticks();
        // crack open the deadlines
        avr_device::interrupt::free(|cs| {
            let deadlines = &mut *WAKE_DEADLINES.borrow(cs).borrow_mut();
            if deadlines.push((new_deadline, task_id)).is_err() {
                panic!("Deadline dropped for task {}!", task_id);
            }
        });
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            TimerState::Init => {
                self.register(cx.waker().task_id());
                self.state = TimerState::Wait;
                Poll::Pending
            }
            TimerState::Wait => {
                if Ticker::now() >= self.end_time {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

pub async fn delay(duration: TickDuration) {
    Timer::new(duration).await;
}

static TICKER: Ticker = Ticker {
    ovf_count: AtomicU32::new(0),
    timer: Mutex::new(RefCell::new(None)),
};

pub struct Ticker {
    ovf_count: AtomicU32,
    timer: Mutex<RefCell<Option<TC0>>>,
}

impl Ticker {
    pub fn init(timer: TC0) {
        // overflow interrupt enable
        timer.timsk0.write(|w| w.toie0().set_bit());
        // prescaler
        //timer.tccr0b.write(|w| w.cs0().direct());
        timer.tccr0b.write(|w| w.cs0().prescale_64());
        // Put the  timer into place
        avr_device::interrupt::free(|cs| {
            TICKER.timer.borrow(cs).replace(Some(timer));
        });
    }
    // Get the current time, which is a combination of:
    pub fn now() -> TickInstant {
        let ticks = TICKER.ovf_count.load(Ordering::SeqCst);
        TickInstant::from_ticks(ticks)
    }

    pub fn ticks() -> u32 {
        TICKER.ovf_count.load(Ordering::SeqCst)
    }

    pub fn show_timers() {
        avr_device::interrupt::free(|cs| {
            let ticks = Ticker::ticks();
            let deadlines = &mut *WAKE_DEADLINES.borrow(cs).borrow_mut();
            for i in deadlines.iter() {
                // this might just be negative ( panic causing)
                let until: i32 = (i.0 as i32) - (ticks as i32);
                crate::print!("task {} in {} ticks ",i.1, until);
            }
        });
    }
}


#[avr_device::interrupt(atmega328p)]
fn TIMER0_OVF() {
    let ticks = TICKER.ovf_count.fetch_add(1, Ordering::Relaxed);
    avr_device::interrupt::free(|cs| {
        let deadlines = &mut *WAKE_DEADLINES.borrow(cs).borrow_mut();
        if let Some((next_deadline, task_id)) = deadlines.peek() {
            if ticks > *next_deadline {
                //crate::print!("Finished -- {} at {}", task_id,ticks);
                // Wake up the task in the executor
                wake_task(*task_id);
                deadlines.pop();
            }
        }
    });
}