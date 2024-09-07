#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]


mod serial;
mod time;

use time::Ticker;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // serial port
    let serial_port = arduino_hal::default_serial!(dp, pins, 115200);
    // bind the serial port to the macro in utils so it can be used anywhere
    serial::serial_init(serial_port);

    print!("Woot it works");

    // Set up timer 0 for system clock 
    Ticker::init(dp.TC0);

    let mut led = pins.d13.into_output();

    // !! DRAGONS , beware the unsafe code !!
    unsafe { avr_device::interrupt::enable() };

    print!("{:x}",Ticker::now().duration_since_epoch().to_millis());

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
        print!("{}",Ticker::now().duration_since_epoch().to_millis());
    }
}

