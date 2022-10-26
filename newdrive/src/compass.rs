// Driver for the compass
// HMC6352
// https://www.sparkfun.com/datasheets/Components/HMC6352.pdf
// i2c device on address 0x21
pub const SLAVE_ADDRESS: u8 = 0x21;
//pub const READ_ADDRESS: u8 = 0x41;

// get bearing 2 bytes , reformatted
//use arduino_hal::prelude::*;
use embedded_hal::blocking::i2c::{Write, WriteRead};
//use core::marker::PhantomData;


// this is a transliteration of the spec

// pub enum Command<u8>{
//     write_eeprom(u8) = 0x77 , // 'w'   write eeprom address
//     read_eeprom(u8) = 0x72 , // 'r' read eeprom address
//     write_ram(u8) = 0x47 , // 'G' write ram register
//     read_ram(u8) = 0x67, // g write ram register
//     enter_sleep_mode(u8) = 0x53, // 'S' Enter sleep mode
//     exit_sleep_mode(u8) = 0x57, // 'W' Exit sleep mode
//     update_bridge_offsets(u8) = 0x4F, // 'O' update bridge ofsets
//     enter_callibration_mode(u8) = 0x4f, // 'C' enter callibration mode
//     exit_callibration_mod(u8) = 0x43, // 'E' exit callibation mode
//     save_op_to_eeprom(u8) = 0x4C, // 'L' save op mode to eeprom
//     get_data(u8) = 0x41  // 'A' get heading
// }

pub struct Compass<I2C> {
    i2c: I2C,
    bearing: u16,
    avebearing: u16,
    address: u8,
}

impl<I2C, E> Compass<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    // create the device
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let com = Compass {
            i2c: i2c,
            bearing: 0,
            avebearing: 0,
            address: SLAVE_ADDRESS,
        };
        Ok(com)
    }

    // enable the device 
    pub fn enable(&mut self) {
        todo!();
    }

    pub fn set_slave_address(&mut self, address: u8) {
        todo!();
    }

    // fetch the data and put it into local storage
    // update should be a general trait for all devices
    pub fn update(&mut self) {
        // get two bytes from the device
        let mut data: [u8; 2] = [0; 2];
        self.i2c.write_read(self.address, &[0x41], &mut data);
        let val = ((data[0] as u16) << 8) | data[1] as u16;
        // save it to me
        self.bearing = val.clone();
    }

    pub fn get_bearing(&self) -> Result<u16, E> {
        Ok(self.bearing)
    }
}

//i2c.write_read(compass::SLAVE_ADDRESS,&[0x57],&mut empty).unwrap();
// i2c.write_read(compass::SLAVE_ADDRESS,&[0x41],&mut data).unwrap();
// ufmt::uwriteln!(&mut serial,"i2c data {}{}",data[0],data[1]).void_unwrap();
// i2c.write(compass::SLAVE_ADDRESS, &[0x41]).unwrap();
// i2c.read(compass::SLAVE_ADDRESS,&mut data).unwrap();
// ufmt::uwriteln!(&mut serial, "Finished read").void_unwrap();
// ufmt::uwriteln!(&mut serial,"i2c data {}{}",data[0],data[1]).void_unwrap();
