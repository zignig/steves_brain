// Differential drive setup
//use arduino_hal::port;

use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::simple_pwm::PwmPinOps;

pub struct Config {
    enabled: bool, // If the motor is running or not
    rate: f32,     // speed at which the rate approaches the goal
    timeout: u32,  // how long it will run a command for before stopping
    interval: u32, // it update interval
}

impl Config {
    fn default() -> Self {
        Self {
            enabled: false,
            rate: 100.,
            timeout: 4000,
            interval: 50,
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
