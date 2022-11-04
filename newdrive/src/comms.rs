//! Comms interface
//! SPI slave interface that makes packets
//! lifted from https://medium.com/embedism/the-definitive-guide-on-writing-a-spi-communications-protocol-for-stm32-73594add4c09
//! and rewritten again in rust ( after c++ )
//!

use arduino_hal::{pac::SPI, prelude::*};
use avr_device::generic::{Reg, RegisterSpec};
use avr_device::interrupt::Mutex;
use core::cell::RefCell;
use core::u8;

const FRAME_SIZE: usize = 16;

// Current status of the frame
pub enum FrameStatus {
    first,
}

pub struct PacketBuffer {
    data: [u8; FRAME_SIZE],
    pos: u8,
}

use avr_device;

pub struct SlaveSPI;

static SPI_INT: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));

impl SlaveSPI {
    pub fn init(s: SPI) {

        // activate spi interrupt ( spie )
        // activate spi ( spe )
        // set slave (mstr = 0)
        s.spcr
            .write(|w| w.spie().set_bit().spe().set_bit().mstr().clear_bit());
        //serial_println!("reg2 {:#?}", s.spcr.read().bits()).void_unwrap();
        // put the spi into a protected global
        avr_device::interrupt::free(|cs| {
            SPI_INT.borrow(&cs).replace(Some(s));
        })
    }
}

use crate::serial_println;

#[avr_device::interrupt(atmega328p)]
fn SPI_STC() {
    avr_device::interrupt::free(|cs| {
        // spi slave stuff
        let mut data: u8 = 0;
        if let Some(s) = &mut *SPI_INT.borrow(&cs).borrow_mut() {
            data = s.spdr.read().bits();
        }
        //let data: u8 = arduino_hal::pac::spi::SPDR::read().bits();
        serial_println!("{}", data).void_unwrap();
    });
}
