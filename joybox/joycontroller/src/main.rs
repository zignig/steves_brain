#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod commands;
mod comms;
mod ring_buffer;

mod display;
mod shared;
mod systick;
mod utils;

mod joystick;
use joystick::AnalogController;
//use commands::Command;
use comms::fetch_command;

use arduino_hal::adc;
use arduino_hal::simple_pwm::*;

use arduino_hal::hal::wdt;

use commands::Command;
use panic_halt as _;

enum State { 
  Running,
  Sleeping,
  StartCallibration,
  EndCallibration,
}


#[arduino_hal::entry]
fn main() -> ! {
    // get the peripherals and pins
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Watch dog timer ( for reboots )
    let mut watchdog  = wdt::Wdt::new(dp.WDT,&dp.CPU.mcusr);
    // SPI interface
    pins.d13.into_pull_up_input(); // sclk
    pins.d11.into_floating_input(); // mosi
    pins.d12.into_output(); // miso
    pins.d10.into_pull_up_input(); // cs

    // Attach the slave spi interface
    comms::SlaveSPI::init(dp.SPI);

    // serial port
    let serial_port = arduino_hal::default_serial!(dp, pins, 115200);
    // bind the serial port to the macro in utils so it can be used anywhere
    utils::serial_init(serial_port);

    serial_println!("Woot it works");

    // eeprom device
    let mut ee = arduino_hal::Eeprom::new(dp.EEPROM);

    // let mut buf: [u8;100] = [0;100];
    // ee.read(0,&mut buf).unwrap();
    // serial_println!("{:?}",buf[..]);

    // 8 - 7 digit disaplay
    let data = pins.d9.into_output();
    let cs = pins.d8.into_output_high();
    let sck = pins.d7.into_output();

    let mut d = display::Display::new(data, cs, sck);
    //d.power_off();
    d.power_on();

    // set the overflow interrupt flag for the systick timer
    dp.TC0.timsk0.write(|w| w.toie0().set_bit());
    // start the timer ( for pwm , but not )
    let _timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    serial_println!("Behold Joycontroller");

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );

    serial_println!("Vbandgap: {}", vbg);
    serial_println!("Ground: {}", gnd);
    serial_println!("Temperature: {}", tmp);

    // joy stick and throttle ananlog pins.
    let a0 = pins.a0.into_analog_input(&mut adc).into_channel();
    let a1 = pins.a1.into_analog_input(&mut adc).into_channel();
    let a2 = pins.a2.into_analog_input(&mut adc).into_channel();
    let a3 = pins.a3.into_analog_input(&mut adc).into_channel();

    let mut the_joystick = joystick::Joy3Axis::new(a0, a1, a2);
    let mut the_throttle = joystick::Throttle::new(a3);

    // Put them into a single structure
    let mut the_controls = joystick::Controls::new(the_joystick,the_throttle);

    let mut the_mode = joystick::Mode::Running;
    let mut num: i32 = 1;
    // activate the display
    d.power_on();
    d.brightness(1);

    the_controls.load(&mut ee);
    the_controls.show_config();
    //the_joystick.mode  = joystick::Mode::Running;

    //activate the interrupts
    // !! DRAGONS , beware the unsafe code !!
    unsafe { avr_device::interrupt::enable() }

    //let c = Command::XY(10, 10);
    //commands::show(c);
    let mut logging: bool = false;
    let mut state = State::Running;
    loop {
        if let Some(comm) = fetch_command() {
            serial_println!("{:#?}", comm);
            //commands::show(comm);
            match comm {
              Command::Hello => serial_println!("hello"),
              Command::Display(val) => { 
                d.show_number(val);
              }
              Command::Brightness(bright) => {
                d.brightness(bright);
              }
              Command::StartCal() => {
                the_mode = joystick::Mode::RunCallibrate;
                state = State::StartCallibration;
              }
              Command::EndCal() => { 
                state = State::EndCallibration;
              }
              Command::ResetCal() => { 
                the_controls.resetcal();
                the_controls.zero_out(&mut adc);
              }
              Command::ShowCal() => { 
                the_controls.show_config();
              }
              Command::Clear() =>{ 
                d.clear();
              }
              Command::Logger =>{
                logging = !logging;
              }
                _ => serial_println!("unbound {:#?}", comm),
            }
        }
        // on the tick ... DO.
        if systick::is_tick() {
            //let time = systick::millis();
            //serial_println!("{:?}", &the_mode);
            the_controls.update(&the_mode,&mut adc);

            match state { 
              State::Running => {
                if logging { 
                  the_controls.show();
                }
              }
              State::Sleeping => { 

              }
              State::StartCallibration => { 
                the_controls.show_config();
              }
              State::EndCallibration => {
                the_controls.save(&mut ee);
                the_mode = joystick::Mode::Running;
                state = State::Running;
              }
            }
            d.show_number(the_controls.throttle.t.value as i32);
            //d.show_number(the_joystick.x.value as i32);
            //d.show_number(time as i32);
            num = num + 1;

        }
    }
}
