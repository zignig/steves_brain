//! Comms interface
//! SPI slave interface that makes packets
//! lifted from https://medium.com/embedism/the-definitive-guide-on-writing-a-spi-communications-protocol-for-stm32-73594add4c09
//! and rewritten again in rust ( after c++ )
//!

use crate::serial_println;
use arduino_hal::{pac::SPI, prelude::*};
use avr_device;
//use avr_device::generic::{Reg, RegisterSpec};
use avr_device::interrupt::Mutex;
use core::cell::RefCell;
use core::u8;
use crate::ring_buffer::Ring;


const FRAME_SIZE: usize = 8;
const SYNC1: u8 = 0xF;
const SYNC2: u8 = 0xE;
const RING_SIZE: usize = 16;
// use serde_cbor::{Deserializer, Serializer};
// use serde_derive::{Deserialize, Serialize};

use ufmt::{derive::uDebug, uwrite};

#[derive(uDebug, Clone, Copy)]
pub enum Command {
    Hello,
    Stop,
    Run(i16, i16),
    SetAcc(u8),
    SetJoy(i16, i16),
    SetTimeout(i16),
    SetTrigger(i16),
    SetMinspeed(u8),
    Sensor,
    Config,
    Count,
    Empty,
}

// data byte and sign bit
fn toi16(a: u8, b: u8) -> i16 {
    let mut val: i16 = a as i16;
    if b == 1 {
        val = -val;
    }
    val
    //(a as i16) << 8 | (b as i16)
}

impl Default for Command {
    fn default() -> Self {
        Command::Empty
    }
}

impl Command {
    fn generate(pb: &PacketBuffer) -> Self {
        // match on the third byte , command type
        let ctype_u8 = pb.data[2];
        let comm: Command = match ctype_u8 {
            0 => Command::Hello,
            1 => Command::Stop,
            2 => Command::Run(toi16(pb.data[4], pb.data[6]), toi16(pb.data[5], pb.data[7])),
            3 => Command::SetAcc(pb.data[4]),
            4 => Command::SetJoy(toi16(pb.data[4], pb.data[6]), toi16(pb.data[5], pb.data[7])),
            5 => Command::SetTimeout(toi16(pb.data[4], pb.data[6])),
            6 => Command::SetTrigger(toi16(pb.data[4], pb.data[6])),
            7 => Command::SetMinspeed(pb.data[4]),
            8 => Command::Sensor,
            9 => Command::Config,
            10 => Command::Count,
            _ => Command::Empty,
        };
        comm
    }
}
#[derive(Clone)]
pub struct PacketBuffer {
    data: [u8; FRAME_SIZE],
    pos: usize,
}

impl PacketBuffer {
    fn new() -> Self {
        Self {
            data: [0; FRAME_SIZE],
            pos: 0,
        }
    }
}

pub struct SlaveSPI;

// guarded access to the SPI object
static SPI_INT: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));

// guarded data PacketBuffer
static DATA_FRAME: Mutex<RefCell<Option<PacketBuffer>>> = Mutex::new(RefCell::new(None));

// ring buffer for the created commans
static COMMAND_RING: Mutex<RefCell<Option<Ring<Command, RING_SIZE>>>> =
    Mutex::new(RefCell::new(None));


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
            COMMAND_RING.borrow(&cs).replace(Some(Ring::<Command,RING_SIZE>::new()));
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
            if let Some(the_packet) = process_packet(data, pb) {
                // the packet is well formed
                //serial_println!("{:#?}", the_packet.data).void_unwrap();
                let comm = Command::generate(&the_packet);
                //serial_println!("{:#?}", comm).void_unwrap();
                if let Some(cr) = &mut *COMMAND_RING.borrow(&cs).borrow_mut() {
                    cr.append(comm);
                    //cr.get_absolute_mut(index)
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
        2 => Ok(()),  //Command
        3 => Ok(()),  //Checksum
        4 => Ok(()),  //data1
        5 => Ok(()),  //data2
        6 => Ok(()),  //data3
        7 => Ok(()),  //data4
        _ => Err(()), // everthing else
    };

    if val == Ok(()) {
        //  If the packet is good so far
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
