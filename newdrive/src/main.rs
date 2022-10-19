#![no_std]
#![no_main]

mod diff_drive;

use panic_halt as _;

use arduino_hal::port;
use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;

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
        50000,
    );
    //let left_eanble = arduino_hal::simple_pwm::IntoPwmPin!();
    
    let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    
    let mut d9 = pins.d9.into_output().into_pwm(&mut timer1);
    let mut d10 = pins.d10.into_output().into_pwm(&mut timer1);
    
    // let mut test_drive = diff_drive::Drive::new(
    //     &timer0,
    //     pins.b1.into_output(),
    //     pins.b0.into_output(),
    // );
    ufmt::uwriteln!(&mut serial,"this is the diff drive").void_unwrap();
    loop {}
}
