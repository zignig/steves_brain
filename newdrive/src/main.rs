#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod compass;
mod current_sensor;
mod diff_drive;
mod utils;
mod systick;
use panic_halt as _;

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;

// wrap the robot in a struct
pub struct Robot<I2C> {
    compass: compass::Compass<I2C>,
    //diff drive: diffdrive::DiffDrive,
    current_sensor: current_sensor::CurrentSensor,
}

#[arduino_hal::entry]
fn main() -> ! {
    // get the peripherals and pins
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // serial port
    let serial_port = arduino_hal::default_serial!(dp, pins, 57600);
    // bind the serial port to the macro in utils so it can be used anywhere
    utils::serial_init(serial_port);

    serial_println!("Woot it works");

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

    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let l_pwm_pin = pins.d3.into_output().into_pwm(&timer2);
    let l_en_pin1 = pins.d8.into_output();
    let l_en_pin2 = pins.d9.into_output();
    let left_drive = diff_drive::SingleDrive::new(l_pwm_pin, l_en_pin1, l_en_pin2);

    // create the right drive
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let r_pwm_pin = pins.d5.into_output().into_pwm(&timer0);
    let r_en_pin1 = pins.d6.into_output();
    let r_en_pin2 = pins.d7.into_output();
    let right_drive = diff_drive::SingleDrive::new(r_pwm_pin, r_en_pin1, r_en_pin2);

    //Create the drives
    let diff_drive = diff_drive::DiffDrive {
        left: left_drive,
        right: right_drive,
    };

    // create the current sensor
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let current_pin = pins.a0.into_analog_input(&mut adc).into_channel();
    let mut current = current_sensor::CurrentSensor::new(current_pin);
    current.get_zero(&mut adc);

    //ufmt::uwriteln!(&mut serial, "Behold steve's minibrain").void_unwrap();

    //systick::millis_init(dp.TC1);
    unsafe { avr_device::interrupt::enable() };

    let mut counter: u32 = 0;
    loop {
        if (counter % 10000) == 0 {
            //ufmt::uwriteln!(&mut serial, "counter {}", counter).void_unwrap();
            compass.update();
            // ufmt::uwriteln!(
            //     &mut serial,
            //     "The Compass: {}",
            //     compass.get_bearing().unwrap()
            // )
            // .void_unwrap();
            //ufmt::uwriteln!(&mut serial, "Current: {}", current.get_value(&mut adc)).void_unwrap();
            //ufmt::uwriteln!(&mut serial, "Time {}",systick::millis()).void_unwrap();
            
        }
        counter = counter + 1;
    }
}
