#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod compass;
mod current_sensor;
mod diff_drive;
mod systick;
mod utils;
mod shared;
mod comms;

use embedded_hal::blocking::serial;
use shared::Update  ;

use panic_halt as _;

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;
use arduino_hal::spi;

#[arduino_hal::entry]
fn main() -> ! {
    // get the peripherals and pins
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // serial port
    let serial_port = arduino_hal::default_serial!(dp, pins, 115200);
    // bind the serial port to the macro in utils so it can be used anywhere
    utils::serial_init(serial_port);

    serial_println!("Woot it works").void_unwrap();

    // i2c driver
    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    // spi slave setup ( experimental )
    //let f = dp.SPI.spcr.write(|w| w.mstr().clear_bit)
    pins.d13.into_pull_up_input(); // sclk
    pins.d11.into_floating_input(); // mosi
    pins.d12.into_output();         // miso
    let j = pins.d10.into_pull_up_input(); // cs
    // there is some evil magic in here.
    comms::SlaveSPI::init(
        dp.SPI,
    );

    // let (mut spi, _) = arduino_hal::Spi::new(
    //     dp.SPI,
    //     pins.d13.into_output(), // sclk
    //     pins.d11.into_output(), // mosi
    //     pins.d12.into_pull_up_input(), //miso
    //     pins.d10.into_output(), //cs
    //     spi::Settings::default(),
    // );
    
    // set the overflow interrupt flag for the systick timer
    dp.TC0.timsk0.write(|w| w.toie0().set_bit());

    // create the compass
    let mut compass = compass::Compass::new(i2c).unwrap();

    // Create the drive parts
    // left drive

    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let l_pwm_pin = pins.d3.into_output().into_pwm(&timer2);
    let l_en_pin1 = pins.d9.into_output();
    let l_en_pin2 = pins.d8.into_output();
    let mut left_drive = diff_drive::SingleDrive::new(l_pwm_pin, l_en_pin1, l_en_pin2);

    // create the right drive
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let r_pwm_pin = pins.d5.into_output().into_pwm(&timer0);
    let r_en_pin1 = pins.d6.into_output();
    let r_en_pin2 = pins.d7.into_output();
    let mut right_drive = diff_drive::SingleDrive::new(r_pwm_pin, r_en_pin1, r_en_pin2);

    //Create the drives
    // let diff_drive = diff_drive::DiffDrive {
    //     left: left_drive,
    //     right: right_drive,
    // };

    // create the current sensor
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let current_pin = pins.a0.into_analog_input(&mut adc).into_channel();
    let mut current = current_sensor::CurrentSensor::new(current_pin);
    // sensor starts at 512  ( measures +ve and -ve, sample at rest and create a zero point )
    current.get_zero(&mut adc);

    serial_println!("Behold steve's minibrain").void_unwrap();
    // Set the overflow interupt for the millis system

    unsafe { avr_device::interrupt::enable() };

    left_drive.enable();
    right_drive.enable();
    right_drive.set_speed(255);
    left_drive.set_speed(-255);

    loop {
        let time = systick::millis();
        if systick::is_tick() {
            right_drive.update();  
            left_drive.update();
            // serial_println!("data {}",comms::get_data()).void_unwrap();
            // serial_println!("data {}",j.is_high()).void_unwrap();
            // serial_println!("time {}", time).void_unwrap();
            // serial_println!("drive {}",right_drive.get_current()).void_unwrap();
            // compass.update();
            // serial_println!("The Compass: {}", compass.get_bearing().unwrap()).void_unwrap();
            // serial_println!("Current: {}", current.get_value(&mut adc)).void_unwrap();
            // serial_println!("").void_unwrap();
        }
    }
}
