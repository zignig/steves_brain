// This is an Async rewrite of the newdrive
// https://github.com/zignig/steves_brain/tree/main/rover/newdrive
// the executor is based on chapter 6 of
// https://github.com/therustybits/zero-to-async

// Robot rewrite in async

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin},
    simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer2Pwm},
};
use core::pin::pin;
use fugit::ExtU64;
use panic_halt as _;

// Modules
mod channel;
mod commands;
mod config;
// mod drive;
mod effectors;
mod executor;
mod isrqueue;
mod overlord;
mod queue;
mod sensors;
mod serial;
mod spi;
mod time;

// Imports
use crate::serial::SerialIncoming;
use channel::Channel;
use commands::Command;
use config::Wrangler;
// use drive::{Drive, DriveCommands};
use effectors::{DriveCommands, TankDrive};
use executor::run_tasks;
use overlord::OverLord;
use queue::Queue;
use sensors::Compass;
use spi::SlaveSPI;
use time::{delay, TickDuration, Ticker};

#[arduino_hal::entry]
fn main() -> ! {
    // grab the stuff from the micro
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Serial Port setup

    let serial_port = arduino_hal::default_serial!(dp, pins, 115200);
    // bind the serial port to the macro in utils so it can be used anywhere
    serial::serial_init(serial_port);

    print!("Activate the awsome!");

    // Set up timer 0 for system clock
    Ticker::init(&dp.TC0);

    // Get the i2c
    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    // Create an i2c bus because there is going to more than 1 i2c device.
    let bus = shared_bus::BusManagerSimple::new(i2c);
    let mut compass = Compass::new(bus.acquire_i2c()).unwrap();
    let compass_task = pin!(compass.task());

    // create the current sensor
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let current_channel = pins.a0.into_analog_input(&mut adc).into_channel();
    let _current_sensor = sensors::CurrentSensor::new(current_channel);

    // Some Test tasks

    // See that it's running on the serial console
    let t1 = pin!(show_name(20.secs(), "boop!"));

    // Grab the eeprom out of the hal
    // the Wrangler should be a config manager , but ...
    // It may be better to just hard address them.
    // proc macto eeprom_store is in progress
    // let ee = arduino_hal::Eeprom::new(dp.EEPROM);
    // let mut wrangler = Wrangler::new(ee);
    // // Config testing.
    // //wrangler.save();
    // let b = wrangler.load();
    // print!("{:?}", b);

    // wrangler.insert(config::Test::new());
    // wrangler.dump();
    // End config testing
    // Setup the SPI interface
    // spi slave setup
    pins.d13.into_pull_up_input(); // sclk
    pins.d11.into_floating_input(); // mosi
    pins.d12.into_output(); // miso
    pins.d10.into_pull_up_input(); // cs
    let mut slave_spi = SlaveSPI::new(dp.SPI);

    // Channels to and from the SPI interface
    let spi_incoming: Channel<Command> = Channel::new();
    let spi_outgoing: Channel<Command> = Channel::new();
    // extract the task
    let spi_task = pin!(slave_spi.task(spi_incoming.get_receiver(), spi_outgoing.get_sender()));

    // Make a new Drive task

    // create left drive
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let l_pwm_pin = pins.d3.into_output().into_pwm(&timer2);
    let l_en_pin1 = pins.d9.into_output();
    let l_en_pin2 = pins.d8.into_output();

    // create the right drive
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let r_pwm_pin = pins.d5.into_output().into_pwm(&timer0);
    let r_en_pin1 = pins.d6.into_output();
    let r_en_pin2 = pins.d7.into_output();

    // Make a comms channel to the motor
    let drive_commands: Channel<DriveCommands> = Channel::new();

    //Create the drive
    let mut diff_drive = effectors::DiffDrive::new(
        l_pwm_pin, l_en_pin1, l_en_pin2, r_pwm_pin, r_en_pin1, r_en_pin2,
    );

    // Create  the drive
    // TODO get the config from eeprom
    // let mut drive = Drive::new(drive::DriveConfig::new());
    // extract the task
    let drive_task = pin!(diff_drive.task(drive_commands.get_receiver()));

    // Serial command system.
    let serial_chars: Queue<u8, 16> = Queue::new();
    let mut serial_incoming = SerialIncoming::new();
    let serial_task = pin!(serial_incoming.task(serial_chars.get_sender()));

    // Push The commands into another task
    let command_out = pin!(make_commands(
        serial_chars.get_receiver(),
        drive_commands.get_sender()
    ));

    // Create the overlord task.
    // This is the top level state machine
    let mut overlord = OverLord::new(drive_commands.get_sender(),spi_incoming.get_sender());

    let overlord_task =
        pin!(overlord.task(spi_outgoing.get_receiver()));

    // DRAGONS! beware , unsafe code.
    unsafe { avr_device::interrupt::enable() };

    // Main Executor (asyncy goodness)
    // Run all the defined tasks
    loop {
        run_tasks(&mut [
            overlord_task,
            spi_task,
            compass_task,
            t1,
            drive_task,
            serial_task,
            command_out,
        ]);
    }
}

// Serial input to control the micro
// react to incoming serial events
// mainly used for testing , interactivty.
async fn make_commands(
    mut rec: queue::Receiver<'_, u8, 16>,
    drive_commands: channel::Sender<'_, DriveCommands>,
) {
    let mut throt: i16 = 150;
    loop {
        let val = rec.receive().await;
        match val {
            b'w' => drive_commands.send(DriveCommands::Run(throt, throt)),
            b's' => drive_commands.send(DriveCommands::Run(-throt, -throt)),
            b'+' => throt += 5,
            b'-' => throt -= 5,
            _ => print!("{}", val.to_ascii_lowercase()),
        }
    }
}

// Just blink the led
async fn blinker(mut led: Pin<Output, PB5>, interval: TickDuration) {
    loop {
        delay(interval).await;
        led.toggle();
    }
}

// Largely for testing boop a name onto the serial console.
async fn show_name(interval: TickDuration, blurb: &str) {
    loop {
        delay(interval).await;
        print!(
            "{} @ {} seconds",
            blurb,
            Ticker::now().duration_since_epoch().to_secs()
        );
    }
}

fn divider() {
    print!("-----------");
}

// Show the current timers in stack
async fn show_time() {
    loop {
        delay(5.secs()).await;
        divider();
        print!("time: {}", Ticker::now().duration_since_epoch().to_secs());
        Ticker::show_timers();
        divider();
    }
}
