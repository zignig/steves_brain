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
// use futures::{select_biased, FutureExt};

use channel::{Channel, Sender};
use drive::{Drive, DriveCommands, DriveState};
use executor::run_tasks;
use queue::Queue;
use time::{delay, TickDuration, Ticker};

//use panic_halt as _;

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
    let blink = pin!(blinker(led, 500.millis()));
    let t1 = pin!(show_name(1000.millis(), "beep!"));
    let t3 = pin!(show_name(24.secs(), "blorp!"));
    let show = pin!(show_time());

    // Make a new Drive task

    // Make a comms channel to the motor
    let drive_state: Channel<DriveState> = Channel::new();
    let drive_commands: Channel<DriveCommands> = Channel::new();

    // Create  the drive
    let mut drive = Drive::new(500.millis());
    let drive_task = pin!(drive.task(drive_state.get_receiver(), drive_commands.get_receiver()));

    // Make a drive starter , temp
    // let drive_starter = pin!(drive_starter(drive_chan.get_sender(), 10.secs()));

    // Queue testing
    let spooly: Queue<u8, 16> = Queue::new();
    let spool_task = pin!(spool_in(spooly.get_sender(), 600.millis()));
    let spool_out_task = pin!(spool_out(spooly.get_receiver(), 50.millis()));

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

    // DRAGONS! beware , unsafe code.
    unsafe { avr_device::interrupt::enable() };

    // Main Executor (asyncy goodness)
    loop {
        run_tasks(&mut [
            // spool_task,
            // spool_out_task,
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
    drive_commands: channel::Sender<'_,DriveCommands>
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
            _ => print!("{}",val.D),
        }
    }
}

async fn spool_in(sender: queue::Sender<'_, u8, 16>, interval: TickDuration) {
    print!("start spool task");
    for i in 0..10 {
        sender.send(i);
    }
    let mut counter: u8 = 0;
    loop {
        delay(interval).await;
        sender.send(counter);
        (counter, _) = counter.overflowing_add(1);
        print!("spool counter: {}, len: {}", counter, sender.len());
    }
}

async fn spool_out(mut rec: queue::Receiver<'_, u8, 16>, _interval: TickDuration) {
    loop {
        let val = rec.receive().await;
        print!("out {}", val);
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

#[cfg(not(doc))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap_infallible();
    if let Some(data) = info.message() {
        let stuff = data.as_str().unwrap();
        ufmt::uwriteln!(&mut serial, "{}", stuff).unwrap_infallible();
    }
    loop {}
}
