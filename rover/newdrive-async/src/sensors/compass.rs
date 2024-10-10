use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use futures::future::poll_fn;
use core::task::Poll;

use fugit::ExtU64;
use crate::time;

// Driver for the compass
// HMC6352
// https://www.sparkfun.com/datasheets/Components/HMC6352.pdf
// i2c device on address 0x21
pub const SLAVE_ADDRESS: u8 = 0x21;
//pub const READ_ADDRESS: u8 = 0x41;


//use core::marker::PhantomData;

// this is a transliteration of the spec
#[repr(u8)]
pub enum Commands {
    // WriteEeprom = 0x77,           // 'w'   write eeprom address
    // ReadEeprom = 0x72,            // 'r' read eeprom address
    // WriteRam = 0x47,              // 'G' write ram register
    // ReadRam = 0x67,               // g write ram register
    // EnterSleepMode = 0x53,        // 'S' Enter sleep mode
    // ExitSleepMode = 0x57,         // 'W' Exit sleep mode
    // UpdateBridgeOffsets = 0x4F,   // 'O' update bridge ofsets
    GetData = 0x41, // 'A' get heading
                    // EnterCallibrationMode = 0x43, // 'C' enter callibration mode
                    // ExitCallibrationMode = 0x45,  // 'E' exit callibation mode
                    // SaveOpToEeprom = 0x4C,        // 'L' save op mode to eeprom
}

pub enum CompassState { 
    Init,
    Idle,
    Set(u16),
    Tracking,
    Fast,
    Slow,
}

pub struct Compass<I2C> {
    i2c: I2C,
    bearing: u16,
    address: u8,
    state: CompassState
}

impl<I2C, E> Compass<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    // create the device
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let com = Compass {
            i2c: i2c,
            bearing: 0,
            address: SLAVE_ADDRESS,
            state: CompassState::Init
        };
        Ok(com)
    }

    // fetch the data and put it into local storage
    // update should be a general trait for all devices
    pub fn update(&mut self) {
        // get two bytes from the device
        let mut data: [u8; 2] = [0; 2];
        let _res = self
            .i2c
            .write_read(self.address, &[Commands::GetData as u8], &mut data);
        let val = ((data[0] as u16) << 8) | data[1] as u16;
        // save it to me
        self.bearing = val.clone();
    }

    pub fn get_bearing(&self) -> u16 {
        self.bearing
    }

    pub async fn setup(&mut self){
        poll_fn(|_cx| {
            match self.state {
                CompassState::Init => {
                    crate::print!("Setup compass");
                    self.state = CompassState::Idle;
                    Poll::Ready(())
                }
                CompassState::Idle => {
                    Poll::Ready(())
                }
                _ => { 
                    Poll::Ready(())
                }
            }
        })
        .await
    }

    
    // Bridge the interrupt into an interal queue.
    pub async fn task(&mut self) {
        self.setup().await;
        loop {
            self.update();
            let  val = self.get_bearing();
            crate::print!("compass {}",val);
            time::delay(30.secs()).await;
        }
    }
}
