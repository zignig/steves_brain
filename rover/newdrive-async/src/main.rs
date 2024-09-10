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
use channel::{Channel, Receiver, Sender};

use drive::{Drive, DriveState};
use executor::run_tasks;
use time::{delay, TickDuration, Ticker};

use core::pin::pin;
use fugit::ExtU32;

use panic_halt as _;

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
    // let mut ser_incoming = serial::SerialIncoming::new();
    // let serial_task = pin!(ser_incoming.task(char_chan.get_sender()));

    // Led (blinky!)
    let led = pins.d13.into_output();

    // Some Test tasks
    let blink = pin!(blinker(led, 500.millis()));
    let t1 = pin!(show_name(874.millis(), "beep!"));
    let t2 = pin!(show_name(513.millis(), "boop!"));
    let t3 = pin!(show_name(1000.millis(), "blorp!"));
    let show = pin!(show_time());

    // Make a new Drive task

    // Make a comms channel to the motor
    let drive_chan: Channel<DriveState> = Channel::new();

    // Create  the drive
    let mut drive = Drive::new(50);
    let drive_task = pin!(drive.task(drive_chan.get_receiver()));

    // Make a drive starter , temp
    let drive_starter = pin!(drive_starter(drive_chan.get_sender(), 10.secs()));

    // let scc = pin!(single_char_command(
    //     char_chan.get_receiver(),
    //     drive_chan.get_sender()
    // ));

    // !! DRAGONS , beware the unsafe code !!
    // Enable interrupts
    unsafe { avr_device::interrupt::enable() };
    
    // Main Executor (asyncy goodness)
    loop {
        run_tasks(&mut [
            // scc,
            // serial_task,
            t1,
            t2,
            t3,
            blink,
            drive_task,
            drive_starter,
            show,
        ]);
    }
}

// async fn single_char_command(mut incoming: Receiver<'_, u8>, outgoing: Sender<'_, DriveState>) {
//     loop {
//         let ch = incoming.receive().await;
//         print!("{}", ch as char);
//         match ch {
//             // b'1' => outgoing.send(DriveState::Running),
//             _ => {
//                 print!("empty")
//             }
//         }
//     }
// }

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
