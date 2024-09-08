use arduino_hal::hal::usart::Event;
use avr_device::interrupt::Mutex;

use core::{
    cell::{Cell, RefCell},
    future::poll_fn,
    task::Poll,
};

use heapless::mpmc::Q8;

use crate::{
    channel::Sender,
    executor::{wake_task, ExtWaker},
};

pub type Usart = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
pub static GLOBAL_SERIAL: Mutex<RefCell<Option<Usart>>> = Mutex::new(RefCell::new(None));

// Serial for print
pub fn serial_init(mut serial: Usart) {
    // Enable the interrupt to uart rcx
    serial.listen(Event::RxComplete);
    avr_device::interrupt::free(|cs| {
        GLOBAL_SERIAL.borrow(cs).replace(Some(serial));
    });
}

#[macro_export]
macro_rules! print{
        ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
                if let Some(serial) = &mut *crate::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                    let _ = ::ufmt::uwriteln!(serial, $($arg)*);
                }
            })
        }
    }

// Serial incoming stuff
// Don't know what i'm doing with this , making it up

// A queue for the chars
static NEXT_CHAR: Q8<u8> = Q8::new();
static SERIAL_TASK_ID: Mutex<Cell<usize>> = Mutex::new(Cell::new(0xFFFF));

enum SerialState {
    Init,
    Wait,
}

pub struct SerialIncoming {
    state: SerialState,
    task_id: usize,
}

impl SerialIncoming {
    pub fn new() -> Self {
        Self {
            state: SerialState::Init,
            task_id: 0xFFFF,
        }
    }

    pub async fn char_available(&mut self) -> u8 {
        poll_fn(|cx| {
            match self.state {
                SerialState::Init => {
                    print!("Setup serial incoming");
                    // Set own task id
                    self.task_id = cx.waker().task_id();
                    self.state = SerialState::Wait;
                    // Put the serial task id into the static
                    // I Suspect this is a bad plan , but let's see if it works
                    avr_device::interrupt::free(|cs| {
                        SERIAL_TASK_ID.borrow(cs).replace(self.task_id);
                    });
                    Poll::Pending
                }
                SerialState::Wait => {
                    if let Some(c) = NEXT_CHAR.dequeue() {
                        // print!("|{}|", c as char);
                        Poll::Ready(c)
                    } else {
                        Poll::Pending
                    }
                }
            }
        })
        .await
    }

    pub async fn task(&mut self, outgoing: Sender<'_, u8>) {
        loop {
            let c = self.char_available().await;
            outgoing.send(c);
        }
    }
}

#[avr_device::interrupt(atmega328p)]
fn USART_RX() {
    avr_device::interrupt::free(|cs| {
        // This is a bit cheeky , but this is the only place that uses rx
        // it is in a critical section , could borrow but don't want to
        // waste cycles in a interrupt.
        let serial_port = unsafe { &*arduino_hal::pac::USART0::ptr() };
        let ch = serial_port.udr0.read().bits();
        if NEXT_CHAR.enqueue(ch).is_ok() {
            let task_id = SERIAL_TASK_ID.borrow(cs).get();
            wake_task(task_id);
        } else {
            print!("no space left in char buffer");
        }
    });
}
