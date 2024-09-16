// This is an Async rewrite of the newdrive
// https://github.com/zignig/steves_brain/tree/main/rover/newdrive
// the executor is based on chapter 6 of
// https://github.com/therustybits/zero-to-async

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(panic_info_message)]

mod channel;
mod config;
mod drive;
mod executor;
mod isrqueue;
mod overlord;
mod queue;
mod serial;
mod time;

//use arduino_hal::prelude::*;
use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
};
use core::pin::pin;
use fugit::ExtU32;


use config::Wrangler;

use channel::Channel;
use drive::{Drive, DriveCommands, DriveState};
use executor::run_tasks;
use overlord::OverLord;
use queue::Queue;
use time::{delay, TickDuration, Ticker};

use panic_halt as _;

use crate::serial::SerialIncoming;

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
    // Just blink the LED to show that it's running
    // SPI takes this ! WATCH OUT !
    let blink = pin!(blinker(led, 500.millis()));
    // See that it's running on the serial console
    let t1 = pin!(show_name(2000.millis(), "boop!"));
    // Longer task , perhaps shut everything down and go to low power
    let t3 = pin!(show_name(30.secs(), "check idle"));
    // Show the current timer queue for debug
    let show = pin!(show_time());

    // Grab the eeprom out of the
    let ee = arduino_hal::Eeprom::new(dp.EEPROM);
    let mut wrangler = Wrangler::new(ee);
    //wrangler.save();
    let b = wrangler.load();
    print!("{:?}",b);
    wrangler.insert(config::Test::new());
    // wrangler.dump();

    // Make a new Drive task

    // Make a comms channel to the motor
    let drive_state: Channel<DriveState> = Channel::new();
    let drive_commands: Channel<DriveCommands> = Channel::new();

    // Create  the drive
    let mut drive = Drive::new(1.secs());
    let drive_task = pin!(drive.task(drive_state.get_receiver(), drive_commands.get_receiver()));

    // Make a drive starter , temp
    // let drive_starter = pin!(drive_starter(drive_chan.get_sender(), 10.secs()));

    // Serial command system.
    let serial_chars: Queue<u8, 16> = Queue::new();
    let mut serial_incoming = SerialIncoming::new();
    let serial_task = pin!(serial_incoming.task(serial_chars.get_sender()));

    // Push The commands into another task
    let command_out = pin!(make_commands(
        serial_chars.get_receiver(),
        drive_state.get_sender(),
        drive_commands.get_sender()
    ));

    // Create the overlord task.
    // This is the top level state machine

    let mut overlord = OverLord::new();
    let overlord_task = pin!(overlord.task());

    // DRAGONS! beware , unsafe code.
    unsafe { avr_device::interrupt::enable() };

    // Main Executor (asyncy goodness)
    loop {
        run_tasks(&mut [
            overlord_task,
            t1,
            t3,
            blink,
            drive_task,
            //drive_starter,
            serial_task,
            command_out,
            show,
        ]);
    }
}

async fn make_commands(
    mut rec: queue::Receiver<'_, u8, 16>,
    drive_state: channel::Sender<'_, DriveState>,
    drive_commands: channel::Sender<'_, DriveCommands>,
) {
    loop {
        let val = rec.receive().await;
        match val {
            b'1' => drive_state.send(DriveState::Running),
            b'2' => drive_state.send(DriveState::Idle),
            b'w' => drive_commands.send(DriveCommands::Forward),
            b's' => drive_commands.send(DriveCommands::Backwards),
            b'a' => drive_commands.send(DriveCommands::Left),
            b'd' => drive_commands.send(DriveCommands::Right),
            b' ' => drive_commands.send(DriveCommands::Stop),
            _ => print!("{}", val.to_ascii_lowercase()),
        }
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

// async fn drive_starter(sender: Sender<'_, DriveState>, interval: TickDuration) {
//     loop {
//         delay(interval).await;
//         print!("Start the drive");
//         sender.send(DriveState::Running);
//     }
// }

// #[cfg(not(doc))]
// #[panic_handler]
// fn panic(info: &core::panic::PanicInfo) -> ! {
//     avr_device::interrupt::disable();
//     let dp = unsafe { arduino_hal::Peripherals::steal() };
//     let pins = arduino_hal::pins!(dp);
//     let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
//     ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap_infallible();
//     if let Some(data) = info.message() {
//         let stuff = data.as_str().unwrap();
//         ufmt::uwriteln!(&mut serial, "{}", stuff).unwrap_infallible();
//     }
//     loop {}
// }
