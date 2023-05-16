#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod commands;
mod comms;
mod compass;
mod current_sensor;
mod diff_drive;
mod ring_buffer;
mod shared;
mod systick;
mod utils;

//mod robot;

use commands::Command;
use comms::fetch_command;
use diff_drive::DiffDrive;

use panic_halt as _;

use shared::TankDrive;
//use shared::Update;

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;
use systick::millis;

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
    pins.d13.into_pull_up_input(); // sclk
    pins.d11.into_floating_input(); // mosi
    pins.d12.into_output(); // miso
    pins.d10.into_pull_up_input(); // cs
                                   // there is some evil magic in here.
    comms::SlaveSPI::init(dp.SPI);

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

    // create the right drive
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let r_pwm_pin = pins.d5.into_output().into_pwm(&timer0);
    let r_en_pin1 = pins.d6.into_output();
    let r_en_pin2 = pins.d7.into_output();

    //Create the drive
    let mut diff_drive = DiffDrive::new(
        l_pwm_pin, l_en_pin1, l_en_pin2, r_pwm_pin, r_en_pin1, r_en_pin2,
    );

    // create the current sensor
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let current_pin = pins.a0.into_analog_input(&mut adc).into_channel();
    let mut current = current_sensor::CurrentSensor::new(current_pin);
    // sensor starts at 512  ( measures +ve and -ve, sample at rest and create a zero point )
    current.get_zero(&mut adc);

    serial_println!("Behold steve's minibrain").void_unwrap();
    //u activate the interrupts
    // !! DRAGONS , beware the unsafe code !!
    unsafe { avr_device::interrupt::enable() };

    //let r = robot::Robot::new(diff_drive,compass,current);

    // set the current limit above normal usage.
    current.set_upper(100);

    // Show some compass data.
    compass.update();
    serial_println!("The Compass: {}", compass.get_bearing().unwrap()).void_unwrap();

    // find the now
    let mut last: u32 = millis();
    let mut verbose: bool = false;

    // let c = commands::Command::Run(10, -10);
    // commands::show(c);

    loop {
        // if the current is too big , stop
        if current.overload(&mut adc) {
            serial_println!("STOP").void_unwrap();
            diff_drive.stop();
        }
        // on the tick ... DO.
        if systick::is_tick() {
            let time = systick::millis();
            diff_drive.update();
            if let Some(value) = diff_drive.get_movement() {
                //serial_println!("drive {},{}", value.0, value.1).void_unwrap();
                //serial_println!("min {}", diff_drive.left.config.min_speed).void_unwrap();

                //     //serial_println!("current {}", current.get_value(&mut adc)).void_unwrap();
                //     //serial_println!("zero {}", current.zero_offset).void_unwrap();
            }
            if let Some(comm) = fetch_command() {
                //serial_println!("tick {}", time - last).void_unwrap();
                // if time - last > systick::TICK_INTERVAL {
                //     serial_println!("MISSED").void_unwrap();
                // }
                last = time;
                //serial_println!(" {}", time-last).void_unwrap();
                //commands::show(comm);
                serial_println!("{:#?}", comm).void_unwrap();
                //serial_println!("").void_unwrap();
                //commands::show(comm);

                match comm {
                    Command::Run(x, y) => {
                        diff_drive.set_speed(x, y);
                    }
                    Command::Stop => {
                        diff_drive.stop();
                    }
                    Command::Cont => { 
                        //let cs = current.get_value(&mut adc);
                        //comms::send_command(Command::Current(cs));
                        compass.update();
                        let bearing = compass.get_bearing().unwrap();
                        comms::send_command(Command::Compass(bearing));
                        //comms::send_command(Command::GetMillis(systick::millis()));
                    }
                    Command::SetAcc(rate) => {
                        diff_drive.set_rate(rate);
                    }
                    Command::SetMinspeed(val) => {
                        diff_drive.set_min(val);
                    }
                    Command::SetTimeout(timeout) => {
                        diff_drive.set_timeout(timeout);
                    }
                    Command::SetMaxCurrent(cur) => {
                        current.set_upper(cur as i16);
                    }
                    Command::SetJoy(x, y) => {
                        diff_drive.set_joy(x, y);
                    }
                    Command::Verbose => { 
                        verbose = !verbose;
                    }
                    _ => serial_println!("unbound {:#?}", comm).void_unwrap(),
                }
            }
            // serial_println!("data {}",comms::get_data()).void_unwrap();
            // serial_println!("data {}",j.is_high()).void_unwrap();
            // serial_println!("drive {}",right_drive.get_current()).void_unwrap();
            if verbose { 
                compass.update();
                serial_println!("The Compass: {}", compass.get_bearing().unwrap()).void_unwrap();
            }
            // serial_println!("Current: {}", current.get_value(&mut adc)).void_unwrap();
            // serial_println!("").void_unwrap();
        }
    }
}
