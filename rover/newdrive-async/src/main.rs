#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(panic_info_message)]

mod channel;
mod drive;
mod executor;
mod serial;
mod time;

use arduino_hal::prelude::*;
use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};
use channel::{Channel, Sender};

use drive::{Drive, DriveState};
use executor::run_tasks;
use time::{delay, TickDuration, Ticker};

// use arduino_hal::port::{mode::Output, Pin};
use core::pin::pin;
use fugit::ExtU32;

// use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Serial Port setup

    let serial_port = arduino_hal::default_serial!(dp, pins, 115200);
    // bind the serial port to the macro in utils so it can be used anywhere
    serial::serial_init(serial_port);

    print!("Woot it works!");

    // Set up timer 0 for system clock
    Ticker::init(dp.TC0);

    // Make a serial char channel for testing
    let char_chan: Channel<u8> = Channel::new();

    // Set up the serial incoming
    let mut ser_incoming = serial::SerialIncoming::new();
    let serial_task = pin!(ser_incoming.task(char_chan.get_sender()));

    // Led (blinky!)
    let mut led = pins.d13.into_output();

    // !! DRAGONS , beware the unsafe code !!
    // Enable interrupts
    unsafe { avr_device::interrupt::enable() };

    // Some Test tasks
    let blink = pin!(blinker(led, 500.millis()));
    let t1 = pin!(test_task(1000.millis(), "boop!"));
    //let t2 = pin!(test_task(800.millis(), "slower"));
    //let t3 = pin!(test_task(60.secs(), "every minute"));
    let show = pin!(show_time());

    // Make a new Drive task

    // Make a comms channel to the motor
    let chan: Channel<DriveState> = Channel::new();

    // Create  the drive
    let mut drive = Drive::new(50);
    let drive_task = pin!(drive.task(chan.get_receiver()));

    // Make a drive starter , temp
    let drive_starter = pin!(drive_starter(chan.get_sender(), 10.secs()));

    // Main Executor (asyncy goodness)
    loop {
        run_tasks(&mut [serial_task, t1, blink, drive_task, drive_starter, show]);
    }
}

async fn blinker(mut led: Pin<Output, PB5>, interval: TickDuration) {
    loop {
        delay(interval).await;
        led.toggle();
    }
}

async fn test_task(interval: TickDuration, blurb: &str) {
    loop {
        delay(interval).await;
        print!("{}", blurb);
    }
}

async fn show_time() {
    loop {
        delay(5.secs()).await;
        print!("-----------");
        // print!("time: {}", Ticker::now().duration_since_epoch().to_millis());o
        print!("time ticks {}", Ticker::ticks());
        print!("-----------");
        Ticker::show_timers();
        print!("-----------");
    }
}

async fn drive_starter(sender: Sender<'_, DriveState>, interval: TickDuration) {
    loop {
        delay(interval).await;
        print!("Start the drive");
        sender.send(DriveState::Running);
    }
}

#[cfg(not(doc))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    // Print out panic location
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap_infallible();
    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}
