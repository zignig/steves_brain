// Some util functions

//! Liberated from <https://github.com/kchmck/moving_avg.rs>
//!
//! Moving average filter.

use avr_device::interrupt::Mutex;
//use bare_metal::Mutex;
use core::cell::RefCell;
/// Computes a moving average over a ring buffer of numbers.
#[derive(Clone, Copy)]
pub struct SerialBuffer {
    cursor: usize,
    pos: usize,
    flag: bool,
    data: [u8;32]
}   

// Wrapped globals ( so ugly )
static BUFFER: avr_device::interrupt::Mutex<RefCell<Option<SerialBuffer>>> =
    avr_device::interrupt::Mutex::new(RefCell::new(None));

impl SerialBuffer { 
    pub fn new()-> Self {
        Self { 
            cursor: 0,
            pos: 0,
            flag: false,
            data: [0;32]
        }
    }
}

pub type Usart = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
pub static GLOBAL_SERIAL: Mutex<RefCell<Option<Usart>>> = Mutex::new(RefCell::new(None));

pub fn serial_init(serial: Usart) {
    avr_device::interrupt::free(|cs| {
        GLOBAL_SERIAL.borrow(cs).replace(Some(serial));
        BUFFER.borrow(cs).replace(Some(SerialBuffer::new()));
    });
}


// interrupts take no arguments , have to yoink globals (bleck!)
// #[avr_device::interrupt(atmega328p)]
// fn USART_RX() {
//     let data: u8 =0;
//     avr_device::interrupt::free(|cs| {
//         if let Some(uart) = &mut *GLOBAL_SERIAL.borrow(cs).borrow_mut(){
//         }
//         if let Some(sb) = &mut *BUFFER.borrow(cs).borrow_mut(){
//             sb.data[sb.pos] = data;
//             sb.pos += 1;
//             sb.flag = true;
//         }
//     })
// }

#[macro_export]
macro_rules! serial_println {
        ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
                if let Some(serial) = &mut *crate::utils::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                    let _ = ::ufmt::uwriteln!(serial, $($arg)*);
                }
            })
        }
    }
