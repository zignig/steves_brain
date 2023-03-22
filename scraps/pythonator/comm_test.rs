//! Enumeration for the commands
//! Used but the comms module to make SPI frames
//!
//!

use crate::serial_println;
use arduino_hal::prelude::*;
use ufmt::derive::uDebug;

use crate::comms::{FrameBuffer, SYNC1, SYNC2};

use serde_derive::{Deserialize, Serialize};
use store_u8::{Dump, Load};

// TODO use the store_u8 serialization ( and write a packet formatter)
// This is the primary command enum
#[derive(uDebug, Clone, Copy, Deserialize, Serialize)]
pub enum Incoming {
    Hello,
    Start,
    Stop,
    Joy(i8,i8,i8),
    Throttle(i8),
    Callibrate(i8),
    Display(u8,u8)
}

pub enum Outgoing { 
    One,
    Two,
    Three(i8,i8,i8),
    Five([u8;10]),
    Size(i32,i32)
}
