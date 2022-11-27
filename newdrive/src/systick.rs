// This is using timer0 (8Bit) as a systick timer

//use arduino_hal::pac::*;
//use arduino_hal::prelude::*;
use core::cell;

const PRESCALER: u32 = 1024;
pub(crate) const TICK_INTERVAL: u32 = 512;
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
        if flag {
            TICK_FLAG.borrow(cs).set(false);
        }
    });
    flag
}
