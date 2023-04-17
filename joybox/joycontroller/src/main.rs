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
mod buttons;

mod joystick;
use joystick::AnalogController;

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
    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
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
    let stop_button_pin  = pins.d6.into_floating_input();
    let right_button_pin = pins.d5.into_floating_input();
    let left_button_pin = pins.d4.into_floating_input();
    let missile_switch_pin = pins.d3.into_floating_input();

    let stop_button = buttons::Button::new(stop_button_pin);
    let right_button = buttons::Button::new(right_button_pin);
    let left_button = buttons::Button::new(left_button_pin);
    let mut missile_switch = buttons::Button::new(missile_switch_pin);

    //let the_buttons = buttons::Buttons::new((stop_button,right_button,left_button,missile_switch));
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
    let mut the_controls = joystick::Controls::new(the_joystick, the_throttle);

    let mut the_mode = joystick::Mode::Running;
    let mut num: i32 = 1;
    // activate the display
    d.power_on();
    d.brightness(1);

    the_controls.load(&mut ee);
    //the_controls.load_fixed();
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
            serial_println!("{:?}", comm);
            //commands::show(comm);
            match comm {
                Command::Hello => serial_println!("hello"),
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
                Command::DumpEeprom => {
                    let mut buf: [u8; 100] = [0; 100];
                    ee.read(0, &mut buf).unwrap();
                    serial_println!("{:?}", buf[..]);
                }
                Command::EraseEeprom(val) => {
                    avr_device::interrupt::free(|cs| {
                        for i in 0..1024 {
                            ee.write_byte(i, val);
                        }
                    });
                    serial_println!("finshed erase");
                }
                _ => serial_println!("unbound {:#?}", comm),
            }
        }
        // on the tick ... DO.
        if systick::is_tick() {
            let time = systick::millis();
            //serial_println!("{:?}", &the_mode);
            the_controls.update(&the_mode, &mut adc);

            match state {
                State::Running => {
                    if logging {
                        //the_controls.show();
                        serial_println!("Missile = {:?}",missile_switch.read());
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
                }
            }
            //d.show_number(the_controls.throttle.t.value as i32);
            //d.show_number(the_controls.throttle.t.value as i32);
            d.show_number((time )  as i32);
            //d.show_hex(num as u32);
            num = num + 1;
        }
    }
}

#[avr_device::interrupt(atmega328p)]
fn WDT() {}
