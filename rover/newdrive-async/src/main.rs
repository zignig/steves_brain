#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod channel;
mod drive;
mod executor;
mod serial;
mod time;

use channel::{Channel, Sender};

use drive::{Drive, DriveState};
use executor::run_tasks;
use time::{delay, TickDuration, Ticker};

// use arduino_hal::port::{mode::Output, Pin};
use core::pin::pin;
use fugit::ExtU32;
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

    //let mut led = pins.d13.into_output();

    // !! DRAGONS , beware the unsafe code !!
    unsafe { avr_device::interrupt::enable() };

    let chan: Channel<DriveState> = Channel::new();

    let tt = pin!(test_task(200.millis(), "quick"));
    let t2 = pin!(test_task(800.millis(), "slower"));
    let t3 = pin!(test_task(10.secs(), "every 10 seconds"));
    let show = pin!(show_time());

    // Make a new Drive task
    let mut drive = Drive::new(200);

    let drive_task = pin!(drive.task(chan.get_receiver()));

    // Make a drive starter
    let drive_starter = pin!(drive_starter(chan.get_sender(), 10.secs()));
    loop {
        //arduino_hal::delay_ms(1000);
        run_tasks(&mut [tt, t2, t3, drive_task, drive_starter,show]);
        //print!("{}", Ticker::now().duration_since_epoch().to_millis());
        // Ticker::show_timers();
        // print!("{}",Ticker::ticks());
    }
}

async fn test_task(interval: TickDuration, blurb: &str) {
    loop {
        delay(interval).await;
        print!("{}", blurb);
        //print!("task working");
    }
}

async fn show_time(){
    loop { 
        delay(1.secs()).await;
        print!("-----------");
        print!("time: {}",Ticker::now().duration_since_epoch().to_millis());
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
