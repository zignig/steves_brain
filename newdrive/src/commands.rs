//! Enumeration for the commands
//! Used but the comms module to make SPI frames
//!
//!

use crate::serial_println;
use arduino_hal::prelude::*;
use ufmt::derive::uDebug;

use crate::comms::{SYNC1, SYNC2};

use serde_derive::{Deserialize, Serialize};
use store::{Dump, Load};

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
    Empty,
}

impl Default for Command {
    fn default() -> Self {
        Command::Empty
    }
}

/// For packet debugging
pub fn show(comm: Command) {
    let mut buf: [u8; 8] = [0; 8];
    //buf[0] = SYNC1;
    //buf[1] = SYNC2;
    //buf[2] = 50;
    // serial_println!("des").void_unwrap();
    //comm.dump_into_bytes(&mut buf[..]).unwrap_or_default();
    //serial_println!("{:?}", buf).void_unwrap();
    let _up = Command::load_from_bytes(&buf[..]).unwrap_or_default();
    // let wtf = Command::load_from_be_bytes(&buf[..]);
    // match wtf {
    //       Ok(command) => {
    //         serial_println!("{:#?}",command).void_unwrap();
    //     }
    //     Err(_) => {
    //         serial_println!("BORK").void_unwrap();
    //     }
    // }
    // serial_println!("{:#?}", up).void_unwrap();
}
