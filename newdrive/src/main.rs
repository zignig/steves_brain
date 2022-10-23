#![no_std]
#![no_main]

//mod diff_drive;
mod compass;

use panic_halt as _;

use arduino_hal::prelude::*;
use embedded_hal::prelude::*;

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

    //let timer0 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    //let mut pwm_pin = pins.d3.into_output().into_pwm(&timer0);
    //let mut en_pin1 = pins.d8.into_output();
    //let mut en_pin2 = pins.d9.into_output();
    //let mut DD = diff_drive::DiffDrive::new(pwm_pin,en_pin1,en_pin2);
    // //pwm_pin.enable();
    // pwm_pin.disable();
    // //pwm_pin.set_duty(50);
    // en_pin1.set_low();
    // en_pin2.set_high();

    ufmt::uwriteln!(&mut serial,"this is the diff drive").void_unwrap();

    //let mut data:[u8;2] = [0,0];
    //i2c.write_read(compass::SLAVE_ADDRESS,&[0x41],&mut data).unwrap();
    //ufmt::uwriteln!(&mut serial,"i2c data {}{}",data[0],data[1]).void_unwrap();
    
    
    let compass = compass::Compass::new(&mut i2c).unwrap();
    let test = compass.get_bearing(&mut i2c).unwrap();
    ufmt::uwriteln!(&mut serial, "The Compass: {}",test).void_unwrap();
    
    
    // ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").void_unwrap();
    // i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Write).void_unwrap();


    let mut counter: u32 = 0;
    loop {
        if counter < 20 { 
        ufmt::uwriteln!(&mut serial,"counter {}",counter).void_unwrap();
        counter = counter + 1;
        
        }
    ufmt::uwriteln!(&mut serial, "The Compass: {}",compass.get_bearing(&mut i2c).unwrap()).void_unwrap();
    }
}
