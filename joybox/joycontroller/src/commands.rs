//! Enumeration for the commands
//! Used but the comms module to make SPI frames
//!
//!

use crate::serial_println;
use arduino_hal::prelude::*;
use ufmt::derive::uDebug;
use hubpack;

use crate::comms::{FrameBuffer, SYNC1, SYNC2};

use serde_derive::{Deserialize, Serialize};
//use store_u8::{Dump, Load};

// TODO use the store_u8 serialization ( and write a packet formatter)
// This is the primary command enum
#[derive(uDebug, Clone, Copy, Deserialize, Serialize)]
pub enum Command {
    Hello,
    Stop,
    Run(i16, i16),
    SetAcc(u8),
    SetJoy(i16, i16),
    SetTimeout(i16),
    SetTrigger(i16),
    SetMinspeed(u8),
    SetMaxCurrent(u8),
    Config,
    Count,
    Data(u8, u8, u8, u8),
    // Returns
    Compass(i16),
    Millis(u32),
    // Fail
    Fail,
}

impl Default for Command {
    fn default() -> Self {
        Command::Fail
    }
}


/// For packet debugging
pub fn show(comm: Command) {
    let mut buf = FrameBuffer::new();
    buf.data[0] = SYNC1;
    buf.data[1] = SYNC2;
    buf.data[2] = 50;
    //serial_println!("{:#?}", comm).void_unwrap();
    comm.
    comm.dump_into_bytes(&mut buf.data[3..]).unwrap_or_default();
    //serial_println!("{:?}", &mut buf.data).void_unwrap();
    let up = Command::load_from_bytes(&buf.data[3..]).unwrap_or_default();
    serial_println!("{:#?}", up);
}
