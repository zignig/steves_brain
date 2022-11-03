// This is using timer1 (16Bit) as a systick timer

use arduino_hal::pac::*;
use arduino_hal::prelude::*;
use core::cell;
use panic_halt as _;

use crate::serial_println;

//use avr_device::interrupt;
//pub fn test(tc1: arduino_hal::pac::TC1){
//    tc1.tccr1a.write(|w| w.wgm1());
//}

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u32 = 1024;
const TICK_INTERVAL: u32 = 2048;
const MILLIS_INCREMENT: u32 = PRESCALER * 256 / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

static TICK_FLAG: avr_device::interrupt::Mutex<cell::Cell<bool>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(false));

#[avr_device::interrupt(atmega328p)]
fn TIMER0_OVF() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
        //activate the TICK
        if counter % TICK_INTERVAL == 0 {
            let tick = TICK_FLAG.borrow(cs);
            tick.set(true);
        }
    })
}

pub fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

pub fn is_tick() -> bool {
    let mut flag: bool = false;
    avr_device::interrupt::free(|cs| {
        flag = TICK_FLAG.borrow(cs).get();
        TICK_FLAG.borrow(cs).set(false);
    });
    flag
}

// ----------------------------------------------------------------------------

// #[arduino_hal::entry]
// fn main() -> ! {
//     let dp = arduino_hal::Peripherals::take().unwrap();
//     let pins = arduino_hal::pins!(dp);
//     let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

//     millis_init(dp.TC0);

//     // Enable interrupts globally
//     unsafe { avr_device::interrupt::enable() };

//     // Wait for a character and print current time once it is received
//     loop {
//         let b = nb::block!(serial.read()).void_unwrap();

//         let time = millis();
//         ufmt::uwriteln!(&mut serial, "Got {} after {} ms!\r", b, time).void_unwrap();
//     }
// }
