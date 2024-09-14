use arduino_hal::hal::usart::Event;
use avr_device::interrupt::Mutex;

use core::{
    cell::{Cell, RefCell},
    future::poll_fn,
    task::Poll,
};

use crate::{
    executor::{wake_task, ExtWaker},
    isrqueue::{self, ISRQueue}
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

pub static INCOMING_QUEUE: ISRQueue<u8, 32> = ISRQueue::new();
static SERIAL_TASK_ID: Mutex<Cell<usize>> = Mutex::new(Cell::new(0xFFFF));

enum SerialState {
    Init,
    Wait,
}

pub struct SerialIncoming<'a> {
    state: SerialState,
    task_id: usize,
    incoming: isrqueue::Receiver<'a, u8, 32>,
}

impl<'a> SerialIncoming<'a> {
    pub fn new() -> Self {
        Self {
            state: SerialState::Init,
            task_id: 0xFFFF,
            incoming: INCOMING_QUEUE.get_receiver(),
        }
    }

    pub async fn setup(&mut self){
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
                    print!("Finished Setup");
                    Poll::Ready(())
                }
                SerialState::Wait => {
                    Poll::Ready(())
                }
            }
        })
        .await
    }

    pub async fn task(&mut self,outgoing: isrqueue::Sender<'_,u8,16>) {
        self.setup().await;
        loop {
            let val = self.incoming.receive().await;
            outgoing.send(val);
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
        INCOMING_QUEUE.send(ch);
        // crate::print!("{}", INCOMING_QUEUE.len());
        let task_id = SERIAL_TASK_ID.borrow(cs).get();
        wake_task(task_id);
    });
}
