// Differential drive setup
use arduino_hal::port;
use arduino_hal::simple_pwm;
pub struct PwmDriver<S> {
    pub drive: S,
    pos: u32,
    rate: u32,
    
}

impl<S> PwmDriver<S>{
    pub fn new(){

    }
}

pub struct Drive {
    active: bool,
    direction: bool,
    speed: u8,
    //enable: arduino_hal::simple_pwm::Timer0Pwm,
    pin1: port::Pin<port::mode::Output>,
    pin2: port::Pin<port::mode::Output>,
}

impl Drive{
    pub fn new(
        timer: arduino_hal::simple_pwm::Timer0Pwm,
        //enable:  port::Pin<port>,
        pin1: port::Pin<port::mode::Output>,
        pin2: port::Pin<port::mode::Output>
    ) -> Self {
        Self {
            active: false,
            direction: false,
            speed: 0,
            //enable: arduino_hal::IntoPwmPin!(enable);
            pin1: pin1,
            pin2: pin2,
        }
    }

    pub fn enable(&mut self) {
        self.active = true;
    }

    pub fn disable(&mut self) {
        self.active = false;
    }

    pub fn forward(&mut self) {
        self.pin1.set_low();
        self.pin2.set_high();
    }

    pub fn backwards(&mut self) {
        self.pin1.set_high();
        self.pin2.set_low();
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;
    }

    pub fn update(&mut self) {}
}
