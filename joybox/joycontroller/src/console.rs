// This is using timer0 (8Bit) as a systick timer

//use arduino_hal::pac::*;
//use arduino_hal::prelude::*;
use core::cell;
use core::cell::RefCell;

struct SerialBuffer { 
    pos: usize,
    data: [u8;32]
}

// Wrapped globals ( so ugly )
static BUFFER: avr_device::interrupt::Mutex<RefCell<Option<SerialBuffer>>> =
    avr_device::interrupt::Mutex::new(Mutex::new(RefCell::new(None)));

impl SerialBuffer { 
    pub fn init(){
        avr_device::interrupt::free(|cs| {
            BUFFER.borrow(&cs).replace(Some());
        });
    }
}

// interrupts take no arguments , have to yoink globals (bleck!)
#[avr_device::interrupt(atmega328p)]
fn USART_RX() {
    avr_device::interrupt::free(|cs| {

    })
}


