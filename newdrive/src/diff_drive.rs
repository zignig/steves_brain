
// Differential drive setup
use arduino_hal::port;
use arduino_hal::simple_pwm;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

pub struct Config {
    enabled: bool, // If the motor is running or not
    rate: f32, // speed at which the rate approaches the goal
    timeout: u32, // how long it will run a command for before stopping
    interval: u32, // it update interval
}

impl Config { 
    fn default() -> Self{
    Self { 
        enabled: false,
        rate: 100.,
        timeout: 4000,
        interval: 50,
    }
    }
}
pub struct DiffDrive< E, P1,P2>
where
 E: PwmPin,
 P1: OutputPin,
 P2: OutputPin,
{ 
    en: E,
    p1: P1, 
    p2: P2,
    config: Config
}

impl< E,P1,P2> DiffDrive< E,P1,P2>
where 
 E: PwmPin,
 P1: OutputPin,
 P2: OutputPin,
  { 
    fn new(mut en: E,mut p1: P1,mut  p2:P2) -> Self{
        let conf = Config::default();
        Self { 
            en: en,
            p1: p1,
            p2: p2,
            config: conf,
        }
    }
}