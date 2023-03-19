// USART BUFFER 

//use arduino_hal::pac::*;
//use arduino_hal::prelude::*;
use core::cell::RefCell;

use avr_device::interrupt::Mutex;

use arduino_hal::pac::usart0::udr0::UDR0_SPEC;
use avr_device::generic::Reg;

pub struct SerialBuffer { 
    reg: &mut Reg<UDR0_SPEC>,
    pos: usize,
    flag: bool,
    data: [u8;32]
}   


// Wrapped globals ( so ugly )
static BUFFER: avr_device::interrupt::Mutex<RefCell<Option<SerialBuffer>>> =
    avr_device::interrupt::Mutex::new(RefCell::new(None));

impl SerialBuffer { 
    pub fn new(data_reg: &mut Reg<UDR0_SPEC>)-> Self {
        Self { 
            reg: data_reg,
            pos: 0,
            flag: false,
            data: [0;32]
        }
    }

    pub fn init(data_reg: &mut Reg<UDR0_SPEC>){
        avr_device::interrupt::free(|cs| {
            BUFFER.borrow(cs).replace(Some(SerialBuffer::new(data_reg)));
        });
    }
}

// interrupts take no arguments , have to yoink globals (bleck!)
#[avr_device::interrupt(atmega328p)]
fn USART_RX() {
    avr_device::interrupt::free(|cs| {
        if let Some(sb) = &mut *BUFFER.borrow(cs).borrow_mut(){
            let c = sb.reg.read().bits();
            sb.data[sb.pos] = c;
            sb.pos += 1;
            sb.flag = true;
        }
    })
}


