//! Comms interface
//! SPI slave interface that makes packets
//! lifted from <https://medium.com/embedism/the-definitive-guide-on-writing-a-spi-communications-protocol-for-stm32-73594add4c09>
//! and rewritten again in rust ( after c++ )
//!

use crate::serial_println;
use arduino_hal::prelude::*;

use arduino_hal::pac::SPI;
use avr_device;
use store::Load;
//use avr_device::generic::{Reg, RegisterSpec};
use crate::commands::Command;
use crate::ring_buffer::Ring;

use avr_device::interrupt::Mutex;
use core::cell::RefCell;
use core::u8;

pub const FRAME_SIZE: usize = 8;
pub const SYNC1: u8 = 0xF;
pub const SYNC2: u8 = 0xE;
pub const RING_SIZE: usize = 4;
// use serde_cbor::{Deserializer, Serializer};
// use serde_derive::{Deserialize, Serialize};

#[derive(Clone,Copy)]
pub struct PacketBuffer {
    pub data: [u8; FRAME_SIZE],
    pos: usize,
}

impl PacketBuffer {
    pub fn new() -> Self {
        Self {
            data: [0; FRAME_SIZE],
            pos: 0,
        }
    }
}

impl Default for PacketBuffer {
    fn default() -> Self {
        Self {
            data: [0; FRAME_SIZE],
            pos: 0,
        }
    }
}

pub struct SlaveSPI;

// guarded access to the SPI object
static SPI_INT: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));

// guarded incoming data PacketBuffer
static DATA_FRAME: Mutex<RefCell<Option<PacketBuffer>>> = Mutex::new(RefCell::new(None));

// guarded outgoing data PacketBuffer 
//static OUT_FRAME: Mutex<RefCell<Option<PacketBuffer>>> = Mutex::new(RefCell::new(None));

// ring buffer for the created commands
static COMMAND_RING: Mutex<RefCell<Option<Ring<Command, RING_SIZE>>>> =
    Mutex::new(RefCell::new(None));

// ring buffer for outgoing packets
// static OUT_RING: Mutex<RefCell<Option<Ring<PacketBuffer,RING_SIZE>>>> = 
//     Mutex::new(RefCell::new(None));

impl SlaveSPI {
    pub fn init(s: SPI) {
        // activate spi interrupt ( spie )
        // activate spi ( spe )
        // set slave (mstr = 0)
        s.spcr
            .write(|w| w.spie().set_bit().spe().set_bit().mstr().clear_bit());
        // put the spi , ring buffer and packetbuffer  into a protected globals
        avr_device::interrupt::free(|cs| {
            SPI_INT.borrow(&cs).replace(Some(s));
            DATA_FRAME.borrow(&cs).replace(Some(PacketBuffer::new()));
            // OUT_FRAME.borrow(&cs).replace(Some(PacketBuffer::new()));
            COMMAND_RING
                .borrow(&cs)
                .replace(Some(Ring::<Command, RING_SIZE>::new()));
            // OUT_RING.borrow(&cs).replace(Some(Ring::<PacketBuffer,RING_SIZE>::new()));
        });
    }
}

#[avr_device::interrupt(atmega328p)]
fn SPI_STC() {
    avr_device::interrupt::free(|cs| {
        // get the data byte from the SPI bus
        let mut data: u8 = 0;
        if let Some(s) = &mut *SPI_INT.borrow(&cs).borrow_mut() {
            data = s.spdr.read().bits();

        }
        // put the data into the buffer
        if let Some(pb) = &mut *DATA_FRAME.borrow(&cs).borrow_mut() {
            // push the byte into the packet checker
            if let Some(the_packet) = process_packet(data, pb) {
                // the packet is well formed
                serial_println!("{:#?}", the_packet.data[..]).void_unwrap();
                // deserialize the command part of the packet
                let comm = Command::load_from_bytes(&the_packet.data[3..]).unwrap_or_default();
                // chuck the command into a ring buffer
                if let Some(cr) = &mut *COMMAND_RING.borrow(&cs).borrow_mut() {
                    cr.append(comm);
                }
            }
        }
    });
}

pub fn process_packet(data: u8, pb: &mut PacketBuffer) -> Option<PacketBuffer> {
    // match up the packet data
    let val = match pb.pos {
        0 => {
            if data == SYNC1 {
                Ok(())
            } else {
                Err(())
            }
        } //Sync1
        1 => {
            if data == SYNC2 {
                Ok(())
            } else {
                Err(())
            }
        } //Sync2
        2 => Ok(()),  //Checksum
        3 => Ok(()),  //Command
        4 => Ok(()),  //data1
        5 => Ok(()),  //data2
        6 => Ok(()),  //data3
        7 => Ok(()),  //data4
        _ => Err(()), // everthing else
    };

    if val == Ok(()) {
        //  If the packet is good so far
        //serial_println!("{:?}",data).void_unwrap();
        pb.data[pb.pos] = data;
        pb.pos += 1;
        // end of the frame
        if pb.pos == FRAME_SIZE {
            pb.pos = 0;
            //Packet Buffer Full ready to go
            return Some(pb.clone());
        }
        // Not Yet
        None
    } else {
        // bad packet data
        // reset
        pb.pos = 0;
        // not ready
        None
    }
}

//Fetch a command out of the ring buffer
pub fn fetch_command() -> Option<Command> {
    let mut comm = None;
    avr_device::interrupt::free(|cs| {
        if let Some(cr) = &mut *COMMAND_RING.borrow(&cs).borrow_mut() {
            //serial_println!("len: {} ",cr.len()).void_unwrap();
            if let Some(val) = cr.pop() {
                //serial_println!("value: {:#?} ",val).void_unwrap();
                comm = Some(val);
            }
        }
    });
    comm
}
