//! Enumeration for the commands
//! Used but the comms module to make SPI frames
//!
//!

use crate::serial_println;
use arduino_hal::prelude::*;
use ufmt::derive::uDebug;

use crate::comms::{PacketBuffer, SYNC1, SYNC2};

//use serde_derive::{Deserialize, Serialize};
//use store::{Dump, Load};

#[derive(uDebug, Clone, Copy)] //, Deserialize, Serialize)]
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
    Data(u8,u8,u8,u8),
    // Returns
    Compass(i16),
    // Fail
    Empty,
}

impl Default for Command {
    fn default() -> Self {
        Command::Empty
    }
}

fn toi16(data: [u8; 2]) -> i16 {
    i16::from_le_bytes(data)
}


fn toi8_4(data: [u8 ; 4]) -> (u8,u8,u8,u8) { 
    let d0 = data[0].try_into().unwrap();
    let d1 = data[1].try_into().unwrap();
    let d2 = data[2].try_into().unwrap();
    let d3 = data[3].try_into().unwrap();
    (d0,d1,d2,d3)
}

impl Command {
    pub fn deserialize(pb: &PacketBuffer) -> Self {
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
                Command::Data(val.0,val.1,val.2,val.3)
            },
            _ => Command::Empty,
            
        };
        comm
    }

    pub fn ser(&self) -> PacketBuffer {
        let mut descr: u8 = 0;
        let mut pb = PacketBuffer::new();
        match self {
            Command::Hello => descr = 0,
            Command::Stop => descr = 1,
            Command::Run(l, r) => todo!(),
            Command::SetAcc(_) => todo!(),
            Command::SetJoy(_, _) => todo!(),
            Command::SetTimeout(_) => todo!(),
            Command::SetTrigger(_) => todo!(),
            Command::SetMinspeed(_) => todo!(),
            Command::SetMaxCurrent(_) => todo!(),
            Command::Config => todo!(),
            Command::Count => todo!(),
            Command::Empty => todo!(),
            Command::Compass(_) => todo!(),
            Command::Data(_, _, _, _) => todo!(),
        }
        pb.data[0] = SYNC1;
        pb.data[1] = SYNC2;
        pb.data[3] = descr;
        pb.data[4] = 0; //TODO checksum

        pb
    }
}
/// For packet debugging
pub fn show(comm: Command) {
    let buf = PacketBuffer::new();
    //buf[0] = SYNC1;
    //buf[1] = SYNC2;
    //buf[2] = 50;
    serial_println!("{:#?}", comm).void_unwrap();
    //comm.dump_into_bytes(&mut buf[..]).unwrap_or_default();
    //serial_println!("{:?}", buf).void_unwrap();
    //let up = Command::load_from_bytes(&buf[..]).unwrap_or_default();
    let up = Command::deserialize(&buf);
    //let _wtf = Command::load_from_bytes(&buf[..]);
    // match wtf {
    //       Ok(command) => {
    //         serial_println!("{:#?}",command).void_unwrap();
    //     }
    //     Err(_) => {
    //         serial_println!("BORK").void_unwrap();
    //     }
    // }
    serial_println!("{:#?}", up).void_unwrap();
}
