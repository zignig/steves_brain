#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(panic_info_message)]

mod channel;
mod drive;
mod executor;
mod queue;
mod serial;
mod time;

use arduino_hal::prelude::*;
use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};
use core::pin::pin;
use fugit::ExtU32;
use futures::{select_biased, FutureExt};

use channel::{Channel, Receiver, Sender};
use drive::{Drive, DriveState};
use executor::run_tasks;
use queue::Queue;
use time::{delay, TickDuration, Ticker};

//use panic_halt as _;

use crate::time::Timer;

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

    // Led (blinky!)
    let led = pins.d13.into_output();

    // Some Test tasks
    let blink = pin!(blinker(led, 500.millis()));
    let t1 = pin!(show_name(1000.millis(), "beep!"));
    // let t2 = pin!(show_name(513.millis(), "boop!"));
    let t3 = pin!(show_name(24.secs(), "blorp!"));
    let show = pin!(show_time());

    // Make a new Drive task

    // Make a comms channel to the motor
    let drive_chan: Channel<DriveState> = Channel::new();

    // Create  the drive
    let mut drive = Drive::new(30);
    let drive_task = pin!(drive.task(drive_chan.get_receiver()));

    // Make a drive starter , temp
    let drive_starter = pin!(drive_starter(drive_chan.get_sender(), 10.secs()));

    // Queue testing
    let spooly: Queue<u8, 16> = Queue::new();

    let spool_task = pin!(spool_in(spooly.get_sender(), 4.secs()));
    let spool_out_task = pin!(spool_out(spooly.get_receiver(), 310.millis()));
    
    // DRAGONS! beware , unsafe code.
    unsafe { avr_device::interrupt::enable() };
    // Main Executor (asyncy goodness)
    loop {
        run_tasks(&mut [
            spool_task,
            spool_out_task,
            t1,
            // t2,
            t3,
            blink,
            drive_task,
            drive_starter,
            show,
        ]);
    }
}

async fn spool_in(sender: queue::Sender<'_, u8, 16>, interval: TickDuration) {
    print!("start spool task");
    let mut counter: u8 = 0;
    loop {
        delay(interval).await;
        //sender.send(counter);
        counter += 1;
        print!("spool counter: {}, len: {}",counter,sender.len());
        // print!("spool counter: {}", counter);
    }
}

async fn spool_out(mut rec: queue::Receiver<'_, u8, 16>, interval: TickDuration) {
    loop {
        delay(interval).await;
        print!("spoolout");
        // let val = rec.receive().await;
        // select_biased! {
        //     val = rec.receive().fuse() => {
        //         print!("{}",val);
        //     }
        //     _ = delay(interval).fuse() => {
        //         print!("timeout");
        //     }
        // }
    }
}

async fn blinker(mut led: Pin<Output, PB5>, interval: TickDuration) {
    loop {
        delay(interval).await;
        led.toggle();
    }
}

async fn show_name(interval: TickDuration, blurb: &str) {
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
fn panic(_info: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap_infallible();
    loop {}
}
