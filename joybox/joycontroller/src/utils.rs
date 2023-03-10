// Some util functions

//! Liberated from <https://github.com/kchmck/moving_avg.rs>
//!
//! Moving average filter.

use avr_device::interrupt::Mutex;
//use bare_metal::Mutex;
use core::cell::RefCell;
/// Computes a moving average over a ring buffer of numbers.
#[derive(Clone, Copy)]
pub struct MovingAverage16 {
    /// Current history of numbers.
    hist: [i16; 16],
    /// Size of the history, as T.
    size: usize,
    /// Index in the history vector to replace next.
    pos: usize,
}

impl MovingAverage16 {
    /// Create a new `MovingAverage` that averages over the given amount of numbers.
    pub fn new() -> Self {
        let hist: [i16; 16] = [0; 16];
        MovingAverage16 {
            hist: hist,
            size: 16,
            pos: 0,
        }
    }

    /// Add the given number to the history, overwriting the oldest number, and return the
    /// resulting moving average.
    pub fn feed(&mut self, num: i16) -> i16 {
        self.hist[self.pos] = num;
        self.pos += 1;
        self.pos %= 16;
        self.avg()
    }

    /// Calculate moving average based on the current history.
    fn avg(&self) -> i16 {
        self.hist.iter().fold(0, |s, &x| s + x) / (self.size as i16)
    }
}

pub type Usart = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
pub static GLOBAL_SERIAL: Mutex<RefCell<Option<Usart>>> = Mutex::new(RefCell::new(None));

pub fn serial_init(serial: Usart) {
    avr_device::interrupt::free(|cs| {
        GLOBAL_SERIAL.borrow(cs).replace(Some(serial));
    });
}

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
