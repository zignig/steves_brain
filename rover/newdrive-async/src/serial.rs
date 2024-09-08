use arduino_hal::hal::usart::Event;
use avr_device::interrupt::Mutex;
use heapless::mpmc::Q4;
use core::{cell::RefCell, future::poll_fn, task::Poll};
use portable_atomic::AtomicU8;

use crate::{channel::Sender, executor::ExtWaker};

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
static NEXT_CHAR: Q4<u8> = Q4::new();

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
            task_id: 0xFFFF
        }
    }
    pub async fn char_available(&mut self) {
        poll_fn(|cx| {
            match self.state{
                SerialState::Init => {
                    print!("Setup serial incoming");
                    // Set own task id
                    self.task_id = cx.waker().task_id();
                    self.state = SerialState::Wait;
                    Poll::Pending
                },
                SerialState::Wait => {
                    Poll::Pending
                }
            }
        })
        .await
    }

    pub async fn task(&mut self,mut outgoing: Sender<'_,u8>){
        loop {
            self.char_available().await;
        }
    }
}

#[avr_device::interrupt(atmega328p)]
fn USART_RX() {
    avr_device::interrupt::free(|cs| {
        if let Some(serial) = &mut *GLOBAL_SERIAL.borrow(cs).borrow_mut() {
            let c = serial.read_byte();
            serial.write_byte(c);
            serial.write_byte(44);
            if NEXT_CHAR.enqueue(c).is_err(){
                print!("no space left in char buffer");
            }
        }
    });
}
