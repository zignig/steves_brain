#![no_std]
#![no_main]

mod compass;
//mod diff_drive;
mod current_sensor;

use panic_halt as _;

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;
//use embedded_hal::prelude::*;

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

    // i2c driver
    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    // create the compass
    let mut compass = compass::Compass::new(i2c).unwrap();

    // Create the drive parts
    // left drive

    let timer0 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut pwm_pin = pins.d3.into_output().into_pwm(&timer0);
    let mut en_pin1 = pins.d8.into_output();
    let mut en_pin2 = pins.d9.into_output();
    //let mut left_drive = diff_drive::DiffDrive::new(pwm_pin, en_pin1, en_pin2);

    // create the current sensor
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let current_pin = pins.a0.into_analog_input(&mut adc).into_channel();
    let mut current = current_sensor::CurrentSensor::new(current_pin);

    ufmt::uwriteln!(&mut serial, "this is the diff drive").void_unwrap();

    let mut counter: u32 = 0;
    loop {
        if (counter % 10000) == 0 {
            ufmt::uwriteln!(&mut serial, "counter {}", counter).void_unwrap();
            compass.update();
            ufmt::uwriteln!(
                &mut serial,
                "The Compass: {}",
                compass.get_bearing().unwrap()
            )
            .void_unwrap();
            ufmt::uwriteln!(
                &mut serial,
                "Current: {}",
                current.get_value(&mut adc)
                //current_pin.analog_read(&mut adc)
            )
            .void_unwrap();
        }
        counter = counter + 1;
    }
}
