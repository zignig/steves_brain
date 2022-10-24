#![no_std]
#![no_main]

//mod diff_drive;
mod compass;
mod current_sensor; 

use panic_halt as _;

use arduino_hal::prelude::*;
use embedded_hal::prelude::*;

// wrap the robot in a struct
pub struct Robot<I2C> { 
    compass: compass::Compass<I2C>,
    //diff drive: diffdrive::DiffDrive,
    // current_sensor: Current Sensor
}

#[arduino_hal::entry]
fn main() -> ! {
    // get the peripherals and pins
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // serial port
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        20000,
    );
    // create the compass
    let mut compass = compass::Compass::new(i2c).unwrap();
    
    // create the current sensor
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let current_pin = pins.a0.into_analog_input(&mut adc);
    //  let mut current = current_sensor::CurrentSensor::new(current_pin);

    ufmt::uwriteln!(&mut serial,"this is the diff drive").void_unwrap();

    let mut counter: u32 = 0;
    loop {
        if (counter % 10000) == 0  { 
        ufmt::uwriteln!(&mut serial,"counter {}",counter).void_unwrap();
        ufmt::uwriteln!(&mut serial, "The Compass: {}",compass.get_bearing().unwrap()).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Current: {}",current_pin.analog_read(&mut adc)).void_unwrap();
        }    
        counter = counter + 1;
    }
}
