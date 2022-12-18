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
#[derive(uDebug, Clone, Copy,Deserialize, Serialize)]
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

fn from_u32(val: u32) -> [u8; 4] {
    u32::to_le_bytes(val)
}

#[inline(always)]
fn from_i16(val: i16) -> [u8; 4] {
    let mut data: [u8; 4] = [0; 4];
    let val: [u8; 2] = i16::to_le_bytes(val);
    data[0] = val[0];
    data[1] = val[1];
    data
}

fn from_u8(val: u8) -> [u8; 4] {
    let mut data: [u8; 4] = [0; 4];
    data[0] = u8::to_le(val).try_into().unwrap();
    data
}

fn from_2i16(l: i16, r: i16) -> [u8; 4] {
    let mut data: [u8; 4] = [0; 4];
    let first: [u8; 2] = i16::to_le_bytes(l).try_into().unwrap();
    let second: [u8; 2] = i16::to_le_bytes(r).try_into().unwrap();
    data = [first[0], first[1], second[0], second[1]];
    data
}

fn toi16(data: [u8; 2]) -> i16 {
    i16::from_le_bytes(data)
}

fn toi8_4(data: [u8; 4]) -> (u8, u8, u8, u8) {
    let d0 = data[0].try_into().unwrap();
    let d1 = data[1].try_into().unwrap();
    let d2 = data[2].try_into().unwrap();
    let d3 = data[3].try_into().unwrap();
    (d0, d1, d2, d3)
}

impl Command {
    pub fn deserialize(pb: &FrameBuffer) -> Self {
        // match on the third byte , command type
        let ctype_u8 = pb.data[2];
        let data: [u8; 4] = pb.data[4..8].try_into().unwrap();
        let comm: Command = match ctype_u8 {
            0 => Command::Hello,
            1 => Command::Stop,
            2 => Command::Run(toi16([data[0], data[1]]), toi16([data[2], data[3]])),
            3 => Command::SetAcc(data[0]),
            4 => Command::SetJoy(toi16([data[0], data[1]]), toi16([data[2], data[3]])),
            5 => Command::SetTimeout(toi16([data[0], data[1]])),
            6 => Command::SetTrigger(toi16([data[0], data[1]])),
            7 => Command::SetMinspeed(data[0].try_into().unwrap()),
            8 => Command::SetMaxCurrent(data[0].try_into().unwrap()),
            9 => Command::Config,
            10 => Command::Count,
            11 => {
                let val = toi8_4(data);
                Command::Data(val.0, val.1, val.2, val.3)
            }
            _ => Command::Fail,
        };
        comm
    }

    pub fn ser(&self) -> FrameBuffer {
        let mut descr: u8 = 0;
        let mut pb = FrameBuffer::new();
        let mut data: [u8; 4] = [0; 4];
        match self {
            Command::Hello => descr = 0,
            Command::Stop => descr = 1,
            Command::Run(l, r) => {
                descr = 2;
                data = from_2i16(*l, *r)
            }
            Command::SetAcc(val) => {
                descr = 3;
                data = from_u8(*val)
            }
            Command::SetJoy(x, y) => {
                descr = 4;
                data = from_2i16(*x, *y)
            }
            Command::SetTimeout(timeout) => {
                descr = 5;
                data = from_i16(*timeout);
            }
            Command::SetTrigger(_) => todo!(),
            Command::SetMinspeed(_) => todo!(),
            Command::SetMaxCurrent(_) => todo!(),
            Command::Config => todo!(),
            Command::Count => todo!(),
            Command::Fail => todo!(),
            // Return stuff >>
            Command::Compass(_) => todo!(),
            Command::Data(_, _, _, _) => todo!(),
            Command::Millis(val) => data = from_u32(*val),
        }
        pb.data[0] = SYNC1;
        pb.data[1] = SYNC2;
        pb.data[2] = descr;
        pb.data[3] = 0; //TODO checksum
        pb.data[4] = data[0];
        pb.data[5] = data[1];
        pb.data[6] = data[2];
        pb.data[7] = data[3];
        // return the data block
        pb
    }
}

/// For packet debugging
pub fn show(comm: Command) {
    let mut buf = FrameBuffer::new();
    buf.data[0] = SYNC1;
    buf.data[1] = SYNC2;
    buf.data[2] = 50;
    //serial_println!("{:#?}", comm).void_unwrap();
    comm.dump_into_bytes(&mut buf.data[3..]).unwrap_or_default();
    //serial_println!("{:?}", &mut buf.data).void_unwrap();
    let up = Command::load_from_bytes(&buf.data[3..]).unwrap_or_default();
    serial_println!("{:#?}", up).void_unwrap();
}
