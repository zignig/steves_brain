// Driver for the compass
// HMC6352
// https://www.sparkfun.com/datasheets/Components/HMC6352.pdf
// i2c device on address 0x21
pub const SLAVE_ADDRESS: u8 = 0x21;
//pub const READ_ADDRESS: u8 = 0x41;

// get bearing 2 bytes , reformatted
//use arduino_hal::prelude::*;
use embedded_hal::blocking::i2c::{ WriteRead };
//use core::marker::PhantomData;

pub struct Compass<I2C> {
    i2c: I2C,
    bearing: u16,
    avebearing: u16,
    address: u8,
}


impl<I2C, E> Compass<I2C>
where
    I2C: WriteRead<Error= E>,
{
       
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let com = Compass {
            i2c: i2c,
            bearing: 0,
            avebearing: 0,
            address: SLAVE_ADDRESS,
        };
        Ok(com)
    }

    pub fn get_bearing(&mut self) -> Result<u16,E>{
        let mut data:[u8;2] = [0;2];
        self.i2c.write_read(self.address,&[0x41],&mut data)?;
        let val = ((data[0] as u16) << 8 ) | data[1] as u16;
        self.bearing = val.clone();
        Ok(val)
    }

}


    //i2c.write_read(compass::SLAVE_ADDRESS,&[0x57],&mut empty).unwrap();
    // i2c.write_read(compass::SLAVE_ADDRESS,&[0x41],&mut data).unwrap();
    // ufmt::uwriteln!(&mut serial,"i2c data {}{}",data[0],data[1]).void_unwrap();
    // i2c.write(compass::SLAVE_ADDRESS, &[0x41]).unwrap();
    // i2c.read(compass::SLAVE_ADDRESS,&mut data).unwrap();
    // ufmt::uwriteln!(&mut serial, "Finished read").void_unwrap();
    // ufmt::uwriteln!(&mut serial,"i2c data {}{}",data[0],data[1]).void_unwrap();