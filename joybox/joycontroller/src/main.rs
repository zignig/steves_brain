#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

//mod commands;
//mod comms;

mod ring_buffer;
mod shared;
mod systick;
mod utils;

mod joystick;

//use commands::Command;
//use comms::fetch_command;

use panic_halt as _;

use arduino_hal::prelude::*;
use systick::millis;

use arduino_hal::adc;

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

    // spi slave setup ( experimental )
    pins.d13.into_pull_up_input(); // sclk
    pins.d11.into_floating_input(); // mosi
    pins.d12.into_output(); // miso
    pins.d10.into_pull_up_input(); // cs
                                   // there is some evil magic in here.
                                   //comms::SlaveSPI::init(dp.SPI);

    // set the overflow interrupt flag for the systick timer
    dp.TC0.timsk0.write(|w| w.toie0().set_bit());
    serial_println!("Behold Joycontroller").void_unwrap();

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let mut last: u32 = millis();

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );

    serial_println!("Vbandgap: {}", vbg).void_unwrap();
    serial_println!("Ground: {}", gnd).void_unwrap();
    serial_println!("Temperature: {}", tmp).void_unwrap();

    let a0 = pins.a0.into_analog_input(&mut adc).into_channel();
    let a1 = pins.a1.into_analog_input(&mut adc).into_channel();
    let a2 = pins.a2.into_analog_input(&mut adc).into_channel();
    //let a3 = pins.a3.into_analog_input(&mut adc);

    //u activate the interrupts
    // !! DRAGONS , beware the unsafe code !!
    unsafe { avr_device::interrupt::enable() };
    let mut the_joystick = joystick::Joy3Axis::new(a0,a1,a2);
    the_joystick.zero_out(&mut adc);
    arduino_hal::delay_ms(500);
    the_joystick.zero_out(&mut adc);
    loop {
        //let x = the_joystick.x.get_value(&mut adc);
        the_joystick.update(&mut adc);
        the_joystick.show();
        //serial_println!("X:{}",the_joystick.x.value).void_unwrap();

        // let values = [
        //     a0.analog_read(&mut adc),
        //     a1.analog_read(&mut adc),
        //     a2.analog_read(&mut adc),
        //     //a3.analog_read(&mut adc),
        // ];

        // for (i, v) in values.iter().enumerate() {
        //     serial_println!("A{}: {} ", i, v).void_unwrap();
        // }

        // serial_println!("").void_unwrap();
        arduino_hal::delay_ms(600);
    }
    // loop {
    //     // on the tick ... DO.
    //     if systick::is_tick() {
    //         let time = systick::millis();
    //         serial_println!("bip").void_unwrap();
    //     }
    // }
}
