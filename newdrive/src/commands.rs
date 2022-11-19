//! Enumeration for the commands
//! Used but the comms module to make SPI frames
//!
//!

use crate::serial_println;
use arduino_hal::prelude::*;
use ufmt::derive::uDebug;

use crate::comms::{PacketBuffer, SYNC1, SYNC2};

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

// data byte and sign bit
fn toi16(a: u8, b: u8) -> i16 {
    let mut val: i16 = a as i16;
    if b == 1 {
        val = -val;
    }
    val
    //(a as i16) << 8 | (b as i16)
}

fn split(a: &i16, b: &i16) -> (u8, u8, u8, u8) {
    (0, 0, 0, 0)
}

impl Default for Command {
    fn default() -> Self {
        Command::Empty
    }
}

impl Command {
    // this should be the return packet set
    pub fn serialize(&self) -> PacketBuffer {
        let mut pb = PacketBuffer::new();
        pb.data[0] = SYNC1;
        pb.data[1] = SYNC2;
        pb.data[2] = match self {
            Command::Hello => 0,
            Command::Stop => 1,
            Command::Run(x, y) => {
                let (a, b, c, d) = split(x, y);
                pb.data[4] = a;
                pb.data[5] = b;
                pb.data[6] = c;
                pb.data[7] = d;
                2
            }
            _ => 255,
        };
        pb
    }

    pub fn deserialize(pb: &PacketBuffer) -> Self {
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
            8 => Command::SetMaxCurrent(pb.data[4]),
            9 => Command::Config,
            10 => Command::Count,
            _ => Command::Empty,
        };
        comm
    }
}


pub fn show(comm: Command) {
    let mut buf: [u8; 8] = [0; 8];
    buf[0] = SYNC1;
    buf[1] = SYNC2;
    buf[2] = 50;
    serial_println!("des").void_unwrap();
    comm.dump_into_bytes(&mut buf[3..]).unwrap_or_default();
    serial_println!("{:?}", buf).void_unwrap();
    let up = Command::load_from_bytes( &buf[3..]).unwrap_or_default();
    serial_println!("{:#?}", up).void_unwrap();
}
