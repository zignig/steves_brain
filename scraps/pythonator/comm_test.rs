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

pub const FRAME_SIZE:usize = 8;

// TODO use the store_u8 serialization ( and write a packet formatter)
// This is the primary command enum
#[derive(uDebug, Clone, Copy, Deserialize, Serialize)]
pub enum Incoming {
    Hello,
    Start,
    Stop,
    One(u8),
    Two(u8,u8),
    Three(i8,i8,i8),
    Four(i32),
    Stuff(i32),
    Other(u32),
    InterOther(i8,u8,i8)
}

