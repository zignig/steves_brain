use core::f32::consts::PI;

//use arduino_hal::port::mode::Output;
// Differential drive setup
//use arduino_hal::port;
use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::PwmPinOps;

use crate::serial_println;
use crate::shared::Update;
use crate::systick::millis;

// some math stuff
use libm;
use libm::{acosf, fabsf, fmaxf, roundf, sqrtf};

pub struct Config {
    enabled: bool,      // If the motor is running or not
    rate: u8,           // speed at which the rate approaches the goal
    timeout: u32,       // how long it will run a command for before stopping
    last_update: u32,   // last update in ms for the system clock
    current_speed: i16, // the current speed that the motor is running
    target_speed: i16,  // the goal speed changed at `rate` per update
}

impl Config {
    fn default() -> Self {
        Self {
            enabled: false,
            rate: 10,
            timeout: 20_000,
            last_update: 0,
            current_speed: 0,
            target_speed: 0,
        }
    }
}
pub struct SingleDrive<TC, E, P1, P2> {
    en: Pin<mode::PwmOutput<TC>, E>,
    p1: Pin<mode::Output, P1>,
    p2: Pin<mode::Output, P2>,
    config: Config,
}

impl<TC, E: PwmPinOps<TC>, P1: PinOps, P2: PinOps> SingleDrive<TC, E, P1, P2> {
    pub fn new(
        en: Pin<mode::PwmOutput<TC>, E>,
        p1: Pin<mode::Output, P1>,
        p2: Pin<mode::Output, P2>,
    ) -> Self {
        let config = Config::default();
        Self { en, p1, p2, config }
    }

    // Enable the drive
    pub fn enable(&mut self) {
        self.config.enabled = true;
        self.en.enable();
    }

    // Disable the drive
    pub fn disable(&mut self) {
        self.config.enabled = false;
        self.en.disable();
    }

    pub fn get_current(&self) -> Option<i16> {
        if self.config.current_speed == 0 {
            None
        } else {
            Some(self.config.current_speed)
        }
    }

    // pub fn remaining(&self) -> Option<u32> {
    //     if self.config.enabled {
    //         let now = millis();
    //         let remaining = self.config.last_update - now;
    //         Some(remaining)
    //     } else {
    //         None
    //     }
    // }

    pub fn stop(&mut self) {
        self.p1.set_low();
        self.p2.set_low();
        self.en.set_duty(0);
        self.disable();
        self.config.current_speed = 0;
    }

    // Set the time out value for the drive
    pub fn set_timeout(&mut self, timeout: i16) {
        self.config.timeout = (timeout as u32) << 10;
        let now = millis();
        self.config.last_update = now + self.config.timeout;
    }

    // Set the acceleration rate
    pub fn set_rate(&mut self, rate: u8) {
        self.config.rate = rate;
    }

    // Set the target speed with time out
    pub fn set_speed(&mut self, speed: i16) {
        let now = millis();
        self.config.last_update = now + self.config.timeout;
        self.config.target_speed = speed;
        self.enable();
    }

    fn set_target(&mut self, speed_i16: i16) {
        if self.config.enabled {
            // constrain the speed
            let speed = speed_i16.clamp(-255, 255);
            if speed >= 0 {
                self.p1.set_high();
                self.p2.set_low();
                let speed_u8: u8 = speed.try_into().unwrap();
                self.en.set_duty(speed_u8);
            } else {
                self.p1.set_low();
                self.p2.set_high();
                let speed_u8: u8 = (-speed).try_into().unwrap();
                self.en.set_duty(speed_u8);
            }
        }
    }
}

//use crate::systick::millis;

impl<TC, E: PwmPinOps<TC>, P1: PinOps, P2: PinOps> Update for SingleDrive<TC, E, P1, P2> {
    fn update(&mut self) {
        let cf = &self.config;
        let mut current = cf.current_speed;
        let target = cf.target_speed;
        let rate = cf.rate;
        if cf.enabled {
            let now = millis();
            // check the timeout
            if self.config.last_update < now {
                serial_println!("timeout").void_unwrap();
                self.stop();
                return;
            }
            // accelerate
            if current < target {
                current += rate as i16;
                // to far ?
                if current > target {
                    current = target;
                }
                self.config.current_speed = current;
                self.set_target(current);
            }
            // decellerate
            if current > target {
                current -= rate as i16;
                // to far ?
                if current < target {
                    current = target;
                }
                self.config.current_speed = current;
                self.set_target(current);
            }
        }
    }
}
// Dual drive takes two single drives
pub struct DiffDrive<TCL, EL, P1L, P2L, TCR, ER, P1R, P2R> {
    pub left: SingleDrive<TCL, EL, P1L, P2L>,
    pub right: SingleDrive<TCR, ER, P1R, P2R>,
}
use crate::shared::TankDrive;

impl<
        TCL,
        EL: PwmPinOps<TCL>,
        P1L: PinOps,
        P2L: PinOps,
        TCR,
        ER: PwmPinOps<TCR>,
        P1R: PinOps,
        P2R: PinOps,
    > DiffDrive<TCL, EL, P1L, P2L, TCR, ER, P1R, P2R>
{
    pub fn new(
        l_en: Pin<mode::PwmOutput<TCL>, EL>,
        l_p1: Pin<mode::Output, P1L>,
        l_p2: Pin<mode::Output, P2L>,
        r_en: Pin<mode::PwmOutput<TCR>, ER>,
        r_p1: Pin<mode::Output, P1R>,
        r_p2: Pin<mode::Output, P2R>,
    ) -> Self {
        Self {
            left: SingleDrive::new(l_en, l_p1, l_p2),
            right: SingleDrive::new(r_en, r_p1, r_p2),
        }
    }
}

impl<
        TCL,
        EL: PwmPinOps<TCL>,
        P1L: PinOps,
        P2L: PinOps,
        TCR,
        ER: PwmPinOps<TCR>,
        P1R: PinOps,
        P2R: PinOps,
    > TankDrive for DiffDrive<TCL, EL, P1L, P2L, TCR, ER, P1R, P2R>
{
    fn update(&mut self) {
        self.left.update();
        self.right.update();
    }

    fn enable(&mut self) {
        self.left.enable();
        self.right.enable();
    }

    fn disable(&mut self) {
        self.left.disable();
        self.right.disable();
    }

    fn stop(&mut self) {
        self.left.stop();
        self.right.stop();
    }

    fn set_speed(&mut self, l_speed: i16, r_speed: i16) {
        self.left.set_speed(l_speed);
        self.right.set_speed(r_speed);
    }

    fn set_timeout(&mut self, timeout: i16) {
        self.left.set_timeout(timeout);
        self.right.set_timeout(timeout);
    }

    fn set_rate(&mut self, rate: u8) {
        self.left.set_rate(rate);
        self.right.set_rate(rate);
    }

    fn set_joy(&mut self, x: i16, y: i16) {
        let mut raw_left: f32;
        let mut raw_right: f32;
        let rad: f32;
        let fx: f32 = x as f32;
        let fy: f32 = y as f32;

        let magnitude:f32 = sqrtf(fx * fx + fy * fy);
        if magnitude != 0.0 {
            rad = acosf(fabsf(fx) / magnitude);
        } else {
            rad = 0.0;
        }

        let angle: f32 = rad * 180.0 / PI;
        let tcoeff: f32 = -1.0 + (angle / 90.0) * 2.0;
        let mut turn = tcoeff * fabsf(fabsf(fy) - fabsf(fx));
        turn = libm::roundf(turn * 100.0) / 100.0;
        serial_println!("rad: {} , turn: {}", angle as i16, turn as i16).void_unwrap();
        
        let mov: f32 = fmaxf(fabsf(fy), fabsf(fx));

        // First and third quadrant
        if (fx >= 0.0 && fy >= 0.0) || (fx < 0.0 && fy < 0.0) {
            raw_left = mov;
            raw_right = turn;
        } else {
            raw_right = mov;
            raw_left = turn;
        }

        // Reverse polarity
        if fy < 0.0 {
            raw_left = 0.0 - raw_left;
            raw_right = 0.0 - raw_right;
        }
        let out_left = raw_left as i16;
        let out_right = raw_right as i16;
        serial_println!("mag: {} , rad: {}", magnitude as i16, rad as i16).void_unwrap();
        serial_println!("(x:{},y:{})", out_left, out_right).void_unwrap();

        self.set_speed(out_left, out_right);

        //serial_println!("mag {}",magnitude).void_unwrap();
        //serial_println!("rad {}",rad as i16).void_unwrap();
    }

    fn get_movement(&self) -> Option<(i16, i16)> {
        let mut val: (i16, i16) = (0, 0);
        let mut active: bool = false;
        if let Some(left_c) = self.left.get_current() {
            val.0 = left_c;
            active = true;
        };
        if let Some(right_c) = self.right.get_current() {
            val.1 = right_c;
            active = true;
        };
        if active {
            Some(val)
        } else {
            None
        }
    }
}

//let timer0 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
//let mut pwm_pin = pins.d3.into_output().into_pwm(&timer0);
//let mut en_pin1 = pins.d8.into_output();
//let mut en_pin2 = pins.d9.into_output();
//let mut DD = diff_drive::DiffDrive::new(pwm_pin,en_pin1,en_pin2);
// //pwm_pin.enable
// pwm_pin.disable();
// //pwm_pin.set_duty(50);
// en_pin1.set_low();
// en_pin2.set_high();
