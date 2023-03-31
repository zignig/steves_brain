//! Enumeration for the commands
//! Used but the comms module to make SPI frames
//!
//!

use crate::serial_println;
//use arduino_hal::prelude::*;
use hubpack::SerializedSize;
use serde_derive::{Deserialize, Serialize};
use ufmt::derive::uDebug;

//use store_u8::{Dump, Load};

// TODO use the store_u8 serialization ( and write a packet formatter)
// This is the primary command enum
#[derive(uDebug, Clone, Copy, Deserialize, Serialize, SerializedSize)]
pub enum Command {
    Hello,
    XY(i8, i8),
    ZT(i8, i8),
    ShowCal(),
    StartCal(),
    EndCal(),
    ResetCal(),
    Display(i32),
    Brightness(u8),
    Clear(),
    OutControl(i8,i8,i8,i8),
    OutSwitches(i8),
    Logger,
    Fail,
}

impl Default for Command {
    fn default() -> Self {
        Command::Fail
    }
}

/// For packet debugging
pub fn show(comm: Command) {
    // let mut buf = FrameBuffer::new();
    // buf.data[0] = SYNC1;
    // buf.data[1] = SYNC2;
    // buf.data[2] = 50;
    const SIZE: usize = Command::MAX_SIZE;
    let mut buf: [u8; SIZE] = [0; SIZE];
    let _ = hubpack::serialize(&mut buf, &comm);
    serial_println!("{:#?}", comm);
    serial_println!("{:#?}", buf);
}
