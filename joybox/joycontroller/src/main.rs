//# The avr section of the joystick interface
//# 3 axis joytick 
//# throttle 
//# two buttons
//# two switches


#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod commands;
mod comms;
mod ring_buffer;

mod buttons;
mod display;
mod shared;
mod systick;
mod utils;

mod joystick;
use joystick::AnalogController;

use comms::{fetch_command, send_command};

use arduino_hal::adc;
use arduino_hal::simple_pwm::*;

//use arduino_hal::hal::wdt;

use commands::Command;
use panic_halt as _;

// This is the primary state of the joystick.
enum State {
    Running,
    Sleeping,
    StartCallibration,
    EndCallibration,
    Idle(i32),
}

#[arduino_hal::entry]
fn main() -> ! {
    // get the peripherals and pins
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Watch dog timer ( for reboots )
    //let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);

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

    // buttons and switches
    let stop_button_pin = pins.d4.into_floating_input();
    let right_button_pin = pins.d3.into_floating_input();
    let left_button_pin = pins.d6.into_floating_input();
    let missile_switch_pin = pins.d5.into_floating_input();

    let mut stop_button = buttons::Button::new(stop_button_pin);
    let mut right_button = buttons::Button::new(right_button_pin);
    let mut left_button = buttons::Button::new(left_button_pin);
    let mut missile_switch = buttons::Button::new(missile_switch_pin);

    //let the_buttons = buttons::Buttons::new((stop_button,right_button,left_button,missile_switch));
    // set the overflow interrupt flag for the systick timer
    dp.TC0.timsk0.write(|w| w.toie0().set_bit());
    // start the timer ( for pwm , but not )
    let _timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    serial_println!("Behold Joycontroller");

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // joy stick and throttle ananlog pins.
    let a0 = pins.a0.into_analog_input(&mut adc).into_channel();
    let a1 = pins.a1.into_analog_input(&mut adc).into_channel();
    let a2 = pins.a2.into_analog_input(&mut adc).into_channel();
    let a3 = pins.a3.into_analog_input(&mut adc).into_channel();

    let the_joystick = joystick::Joy3Axis::new(a0, a1, a2);
    let the_throttle = joystick::Throttle::new(a3);

    // Put them into a single structure
    let mut the_controls = joystick::Controls::new(the_joystick, the_throttle);

    let mut the_mode = joystick::Mode::Running;
    let mut num: i32 = 1;
    // activate the display
    d.power_on();
    d.brightness(2);

    the_controls.load(&mut ee);
    //the_controls.load_fixed();
    //the_controls.show_config();
    //the_joystick.mode  = joystick::Mode::Running;

    //activate the interrupts
    // !! DRAGONS , beware the unsafe code !!
    unsafe { avr_device::interrupt::enable() }

    //let c = Command::XY(10, 10);
    //commands::show(c);
    let mut idle_counter: i32 = 0;
    let mut logging: bool = false;
    let mut verbose: bool = true;
    let mut state = State::Running;
    loop {
        // If there is a command in the ring buffer , fetch and execute.
        if let Some(comm) = fetch_command() {
            if verbose {
                serial_println!("{:?}", comm);
                d.scanner();
            }
            match comm {
                Command::Hello => {
                    serial_println!("Hello");
                    //send_command(Command::Hello);
                    send_command(Command::GetMillis(systick::millis()));
                }
                Command::RunOn => {
                    if let Some((a, b, c, d)) = the_controls.data(){
                        serial_println!("data");
                        send_command(Command::OutControl(a, b, c, d));
                    }
                }
                Command::Display(val) => {
                    d.show_number(val);
                }
                Command::HexDisplay(val) => {
                    d.show_hex(val as u32);
                }
                Command::Brightness(bright) => {
                    d.brightness(bright);
                }
                Command::StartCal => {
                    the_mode = joystick::Mode::RunCallibrate;
                    the_controls.zero_out(&mut adc);
                    state = State::StartCallibration;
                }
                Command::EndCal => {
                    state = State::EndCallibration;
                }
                Command::ResetCal => {
                    the_controls.resetcal();
                    the_controls.zero_out(&mut adc);
                }
                Command::ShowCal => {
                    the_controls.show_config();
                }
                Command::LoadCal => {
                    the_controls.load(&mut ee);
                }
                Command::LoadDefault => {
                    the_controls.load_fixed();
                }
                Command::Clear => {
                    d.clear();
                }
                Command::Logger => {
                    logging = !logging;
                }
                Command::Verbose => {
                    verbose = !verbose;
                }
                Command::DumpEeprom => {
                    let mut buf: [u8; 100] = [0; 100];
                    ee.read(0, &mut buf).unwrap();
                    serial_println!("{:?}", buf[..]);
                }
                Command::EraseEeprom(val) => {
                    avr_device::interrupt::free(|_cs| {
                        for i in 0..1024 {
                            ee.write_byte(i, val);
                        }
                    });
                    serial_println!("finshed erase");
                }
                Command::XY(x, y) => {
                    send_command(Command::XY(x, y));
                }
                Command::OutControl(a, b, c, d) => {
                    send_command(Command::OutControl(a, b, c, d));
                }
                _ => {
                    serial_println!("unbound {:#?}", comm)
                }
            }
        }
        // on the tick ... DO.
        if systick::is_tick() {
            let _time = systick::millis();
            //serial_println!("{:?}", &the_mode);
            the_controls.update(&the_mode, &mut adc);
            match state {
                State::Running => {
                    if logging {
                        the_controls.show();
                        serial_println!("Missile = {:?}", missile_switch.read());
                        serial_println!("Left = {:?}", left_button.read());
                        serial_println!("Right = {:?}", right_button.read());
                        serial_println!("EStop = {:?}", stop_button.read());
                    }
                }
                State::Sleeping => {}
                State::StartCallibration => {
                    the_controls.show_config();
                }
                State::EndCallibration => {
                    the_controls.save(&mut ee);
                    //the_controls.joystick.save(&mut ee);
                    the_mode = joystick::Mode::Running;
                    state = State::Running;
                },
                State::Idle(_) => {}
            }
            //d.show_number(the_controls.throttle.t.value as i32);
            //d.show_number(the_controls.throttle.t.value as i32);
            //d.show_number(_time as i32);
            //d.show_hex(num as u32);
            num = num + 1;
        }
    }
}

#[avr_device::interrupt(atmega328p)]
fn WDT() {}
