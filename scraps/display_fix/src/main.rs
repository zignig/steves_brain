#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

//mod commands;
//mod comms;

use max7219;

mod display;
mod ring_buffer;
mod shared;
mod systick;
mod utils;

mod joystick;

//use commands::Command;
//use comms::fetch_command;

use panic_halt as _;

use arduino_hal::adc;
use arduino_hal::prelude::*;
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

    serial_println!("Woot it works");

    // eeprom device
    let ee = arduino_hal::Eeprom::new(dp.EEPROM);
    let mut buf: [u8; 100] = [0; 100];
    //ee.write(0,&data);
    ee.read(0, &mut buf).unwrap();
    serial_println!("{:?}", buf[..]);

    let data = pins.d9.into_output();
    let cs = pins.d8.into_output_high();
    let sck = pins.d7.into_output();

    let mut d = display::Display::new(data, cs, sck);
    d.power_on();

    // spi slave setup ( experimental )
    pins.d13.into_pull_up_input(); // sclk
    pins.d11.into_floating_input(); // mosi
    pins.d12.into_output(); // miso
    pins.d10.into_pull_up_input(); // cs
                                   // there is some evil magic in here.
                                   //comms::SlaveSPI::init(dp.SPI);

    // set the overflow interrupt flag for the systick timer
    dp.TC0.timsk0.write(|w| w.toie0().set_bit());
    serial_println!("Behold Joycontroller");

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    //let mut last: u32 = millis();

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );

    serial_println!("Vbandgap: {}", vbg);
    serial_println!("Ground: {}", gnd);
    serial_println!("Temperature: {}", tmp);

    let a0 = pins.a0.into_analog_input(&mut adc).into_channel();
    let a1 = pins.a1.into_analog_input(&mut adc).into_channel();
    let a2 = pins.a2.into_analog_input(&mut adc).into_channel();
    let a3 = pins.a3.into_analog_input(&mut adc).into_channel();

    //let a3 = pins.a3.into_analog_input(&mut adc);

    //u activate the interrupts
    // !! DRAGONS , beware the unsafe code !!
    unsafe { avr_device::interrupt::enable() };
    let mut the_joystick = joystick::Joy3Axis::new(a0, a1, a2);
    the_joystick.zero_out(&mut adc);
    arduino_hal::delay_ms(500);
    the_joystick.zero_out(&mut adc);
    let mut the_throttle = joystick::Throttle::new(a3);
    //d.show_number(100);
    let mut num: i32 = 0;
    d.power_on();
    d.brightness(0);
    
    loop {
        //display.write_str(0, b"12345678", 0b00000000).unwrap();
        //arduino_hal::delay_ms(600);
        //d.power_on();
        //d.clear();

        d.show_number(num);
        num += 1;
        the_joystick.update(&mut adc);
        the_joystick.show();
        the_throttle.update(&mut adc);
        the_throttle.show();
        arduino_hal::delay_ms(500);
    }
    // loop {
    //     // on the tick ... DO.
    //     if systick::is_tick() {
    //         let time = systick::millis();
    //         serial_println!("bip").void_unwrap();
    //     }
    // }
}
