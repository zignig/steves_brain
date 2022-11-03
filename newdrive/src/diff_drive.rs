// Differential drive setup
//use arduino_hal::port;

use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::simple_pwm::PwmPinOps;

pub struct Config {
    enabled: bool,      // If the motor is running or not
    rate: i16,          // speed at which the rate approaches the goal
    timeout: u32,       // how long it will run a command for before stopping
    interval: u32,      // it update
    current_speed: i16, // the current speed that the motor is running
    target_speed: i16,  //
}

impl Config {
    fn default() -> Self {
        Self {
            enabled: false,
            rate: 20,
            timeout: 4000,
            interval: 50,
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

    pub fn get_current(&self) ->i16{
        self.config.current_speed
    }

    pub fn forward(&mut self, value: u8) {
        if self.config.enabled {
            self.p1.set_high();
            self.p2.set_low();
            self.en.set_duty(value);
        }
    }

    pub fn reverse(&mut self, value: u8) {
        if self.config.enabled {
            self.p2.set_high();
            self.p1.set_low();
            self.en.set_duty(value);
        }
    }

    pub fn stop(&mut self) {
        self.p1.set_low();
        self.p2.set_low();
        self.en.set_duty(0);
        self.config.current_speed =0 ;
        self.disable();
    }

    pub fn set_speed(&mut self,speed: i16){
        self.config.target_speed = speed;
    }

    pub fn set_target(&mut self, speed_i16: i16) {
        if self.config.enabled {
            // constrain the speed
            let speed = speed_i16.clamp(-255, 255);
            if (speed >= 0) {
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


use crate::systick::millis;

pub trait Update {
    fn update(&mut self);
}

impl<TC, E: PwmPinOps<TC>, P1: PinOps, P2: PinOps> Update for SingleDrive<TC, E, P1, P2> {
    fn update(&mut self) {
        //let time = millis();
        let mut cf = &self.config;
        let mut current = cf.current_speed;
        let target = cf.target_speed;
        let rate = cf.rate;
        if cf.enabled {
            // accelerate
            if (current < target) {
                current += rate;
                // to far ?
                if (current > target) {
                    current = target;
                }
                self.config.current_speed = current;
                self.set_target(current);
            }
            // decellerate
            if (current > target) {
                current -= rate;
                // to far ?
                if (current < target) {
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
