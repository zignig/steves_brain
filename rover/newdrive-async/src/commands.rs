use serde_derive::{Deserialize, Serialize};
use ufmt::derive::uDebug;

// This is the primary command enum for the drive
#[derive(uDebug, Clone, Copy, Deserialize, Serialize)]
pub enum Command {
    Hello,
    Stop,
    Cont,
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
    Compass(u16),
    GetMillis(u32),
    Current(i16),
    Verbose,
    Fail,
}

impl Default for Command {
    fn default() -> Self {
        Command::Fail
    }
}