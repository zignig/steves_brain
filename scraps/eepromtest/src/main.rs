#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod utils;
use panic_halt as _;
use arduino_hal;
use arduino_hal::Eeprom;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut ee = arduino_hal::Eeprom::new(dp.EEPROM);


    let serial_port = arduino_hal::default_serial!(dp, pins, 115200);
    // bind the serial port to the macro in utils so it can be used anywhere
    utils::serial_init(serial_port);

    dump(&ee);
    serial_println!("Woot it works");
    for i in 0..1024 {
        ee.write_byte(i,(i % 256) as u8 );
        serial_println!("{:?}",i);
      }
    loop {
        arduino_hal::delay_ms(1000);
        dump(&ee);
    }
}

fn dump(ee: &Eeprom) {
    let mut buf: [u8;1024] = [0;1024];
    ee.read(0,&mut buf).unwrap();
    serial_println!("{:?}",buf[..]);
}
