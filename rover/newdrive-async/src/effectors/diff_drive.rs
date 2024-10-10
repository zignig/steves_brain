use core::f32::consts::PI;
use core::marker::PhantomData;
use core::task::Poll;

//use arduino_hal::port::mode::Output;
// Differential drive setup
use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::simple_pwm::PwmPinOps;
// some math stuff
use fugit::ExtU64;
use futures::future::poll_fn;
use futures::{join, select_biased, FutureExt};
use libm::{acosf, fabsf, fmaxf, sqrtf};

use crate::{
    channel::Receiver,
    time,
    time::{TickDuration, TickInstant},
    Ticker,
};

// Overstate of the drive
#[derive(PartialEq)]
pub enum DriveState {
    Init,
    Running,
    SoftStop,
    Idle,
    Error,
}
// Traits
pub trait Update {
    fn update(&mut self);
    fn adjust_throttle(&mut self, rate: i16) -> bool;
}

pub trait TankDrive {
    fn update(&mut self);
    fn adjust_throttle(&mut self, rate: i16) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn stop(&mut self);
    fn set_speed(&mut self, l_speed: i16, r_speed: i16);
    fn set_timeout(&mut self, timeout: i16);
    fn set_min(&mut self, val: i16);
    fn set_rate(&mut self, rate: i16);
    fn get_movement(&self) -> Option<(i16, i16)>;
    fn set_joy(&mut self, x: i16, y: i16);
}

pub struct Config {
    enabled: bool, // If the motor is running or not
    rate: i16,     // speed at which the rate approaches the goal
    stop_rate: i16,
    timeout: TickDuration, // how long it will run a command for before stopping
    min_speed: i16,
}

impl Config {
    fn default() -> Self {
        Self {
            enabled: false,
            rate: 1,
            stop_rate: 10,
            timeout: 500.millis(),
            min_speed: 0,
        }
    }
}
pub struct SingleDrive<TC, E, P1, P2> {
    en: Pin<mode::PwmOutput<TC>, E>,
    p1: Pin<mode::Output, P1>,
    p2: Pin<mode::Output, P2>,
    pub config: Config,
    current: i16,
    throttle: i16,
    next_update: TickInstant,
    state: DriveState,
}

impl<TC, E: PwmPinOps<TC>, P1: PinOps, P2: PinOps> SingleDrive<TC, E, P1, P2> {
    pub fn new(
        en: Pin<mode::PwmOutput<TC>, E>,
        p1: Pin<mode::Output, P1>,
        p2: Pin<mode::Output, P2>,
    ) -> Self {
        let config = Config::default();
        Self {
            en,
            p1,
            p2,
            config,
            current: 0,
            throttle: 0,
            next_update: Ticker::now(),
            state: DriveState::Init,
        }
    }

    // Enable the drive
    pub fn enable(&mut self) {
        self.config.enabled = true;
        self.en.enable();
        self.state = DriveState::Running;
    }

    // Disable the drive
    pub fn disable(&mut self) {
        self.config.enabled = false;
        self.en.disable();
        self.state = DriveState::Idle;
    }

    pub fn get_current(&self) -> Option<i16> {
        if self.current == 0 {
            None
        } else {
            Some(self.current)
        }
    }

    pub fn stop(&mut self) {
        self.p1.set_low();
        self.p2.set_low();
        self.en.set_duty(0);
        self.disable();
        self.current = 0;
    }

    // Set the time out value for the drive
    pub fn set_timeout(&mut self, timeout: i16) {
        self.config.timeout = (timeout as u64).millis();
        let now = Ticker::now();
        self.next_update = now + self.config.timeout;
    }

    // Set the acceleration rate
    pub fn set_rate(&mut self, rate: i16) {
        self.config.rate = rate;
    }

    pub fn set_min(&mut self, val: i16) {
        self.config.min_speed = val;
    }

    // Set the target speed with time out
    pub fn set_speed(&mut self, speed: i16) {
        let now = Ticker::now();
        self.next_update = now + self.config.timeout;
        self.throttle = speed;
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
        if cf.enabled {
            let now = Ticker::now();
            // check the timeout
            if self.next_update < now {
                crate::print!("timeout");
                self.throttle = 0;
                self.state = DriveState::SoftStop;
                return;
            }
        }
        let _ = self.adjust_throttle(self.config.rate);
    }

    fn adjust_throttle(&mut self, rate: i16) -> bool {
        // accelerate
        // crate::print!("{}",self.current);
        if self.current < self.throttle {
            // if below minimum , accel faster
            self.current += rate as i16;
            // to far ?
            if self.current > self.throttle {
                self.current = self.throttle;
            }
            self.set_target(self.current);
        }
        // decellerate
        if self.current > self.throttle {
            self.current -= rate as i16;
            // to far ?
            if self.current < self.throttle {
                self.current = self.throttle;
            }
            self.set_target(self.current);
        }
        self.current == self.throttle
    }
}

// Async functions
impl<TC, E: PwmPinOps<TC>, P1: PinOps, P2: PinOps> SingleDrive<TC, E, P1, P2> {
    pub async fn run_if(&mut self) {
        poll_fn(|_cx| match self.state {
            DriveState::Init => {
                self.state = DriveState::Idle;
                Poll::Pending
            }
            DriveState::Running => {
                self.update();
                Poll::Ready(())
            }
            DriveState::SoftStop => {
                if self.adjust_throttle(self.config.stop_rate) {
                    self.state = DriveState::Idle
                }
                Poll::Ready(())
            }
            DriveState::Idle => {
                self.stop();
                Poll::Pending
            }
            DriveState::Error => Poll::Pending,
        })
        .await
    }
}

// Dual drive takes two single drives
// With async control

pub struct DiffDrive<'a, TCL, EL, P1L, P2L, TCR, ER, P1R, P2R> {
    left: SingleDrive<TCL, EL, P1L, P2L>,
    right: SingleDrive<TCR, ER, P1R, P2R>,
    state: DriveState, // For the async controller
    lt: PhantomData<&'a ()>,
}

impl<
        'a,
        TCL,
        EL: PwmPinOps<TCL>,
        P1L: PinOps,
        P2L: PinOps,
        TCR,
        ER: PwmPinOps<TCR>,
        P1R: PinOps,
        P2R: PinOps,
    > DiffDrive<'a, TCL, EL, P1L, P2L, TCR, ER, P1R, P2R>
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
            state: DriveState::Idle,
            lt: PhantomData,
        }
    }
}

impl<
        'a,
        TCL,
        EL: PwmPinOps<TCL>,
        P1L: PinOps,
        P2L: PinOps,
        TCR,
        ER: PwmPinOps<TCR>,
        P1R: PinOps,
        P2R: PinOps,
    > TankDrive for DiffDrive<'a, TCL, EL, P1L, P2L, TCR, ER, P1R, P2R>
{
    fn update(&mut self) {
        self.left.update();
        self.right.update();
    }

    fn adjust_throttle(&mut self, rate: i16) -> bool {
        let l = self.left.adjust_throttle(rate);
        let r = self.right.adjust_throttle(rate);
        l || r
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
        self.state = DriveState::Idle
    }

    fn set_speed(&mut self, l_speed: i16, r_speed: i16) {
        self.left.set_speed(l_speed);
        self.right.set_speed(r_speed);
    }

    fn set_timeout(&mut self, timeout: i16) {
        self.left.set_timeout(timeout);
        self.right.set_timeout(timeout);
    }

    fn set_min(&mut self, val: i16) {
        self.left.set_min(val);
        self.right.set_min(val);
    }

    fn set_rate(&mut self, rate: i16) {
        self.left.set_rate(rate);
        self.right.set_rate(rate);
    }

    fn set_joy(&mut self, x: i16, y: i16) {
        let mut raw_left: f32;
        let mut raw_right: f32;
        let rad: f32;
        let fx: f32 = x as f32;
        let fy: f32 = y as f32;

        let magnitude: f32 = sqrtf(fx * fx + fy * fy);
        if magnitude != 0.0 {
            rad = acosf(fabsf(fx) / magnitude);
        } else {
            rad = 0.0;
        }

        let angle: f32 = rad * 180.0 / PI;
        let tcoeff: f32 = -1.0 + (angle / 90.0) * 2.0;
        let mut turn = tcoeff * fabsf(fabsf(fy) - fabsf(fx));
        turn = libm::roundf(turn * 100.0) / 100.0;
        crate::print!("rad: {} , turn: {}", angle as i16, turn as i16);

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
        //serial_println!("mag: {} , rad: {}", magnitude as i16, rad as i16).void_unwrap();
        crate::print!("(x:{},y:{})", out_left, out_right);

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

// Driver commands
pub enum DriveCommands {
    Run(i16, i16),
    Stop,
}

// Asunc Functions
impl<
        'a,
        TCL,
        EL: PwmPinOps<TCL>,
        P1L: PinOps,
        P2L: PinOps,
        TCR,
        ER: PwmPinOps<TCR>,
        P1R: PinOps,
        P2R: PinOps,
    > DiffDrive<'a, TCL, EL, P1L, P2L, TCR, ER, P1R, P2R>
{
    fn set_command(&mut self, command: &DriveCommands) {
        // update as the update is lost in the select_biased
        self.update();
        // only run in running and idle
        if self.state != DriveState::Error {
            match command {
                DriveCommands::Run(l, r) => {
                    self.enable();
                    self.set_speed(*l, *r);
                    crate::print!("forward")
                }
                DriveCommands::Stop => {
                    self.disable();
                    self.stop();
                    crate::print!("stop");
                }
            }
        }
    }

    // Run setup
    pub async fn run_if(&mut self) {
        // poll_fn(|_cx| {
        crate::print!("{},{}",self.left.current,self.right.current);
        let _ = join!(self.left.run_if(), self.right.run_if(),);
        // })
        // .await
    }

    // this will wait for commands or run the drive if
    // it needs to.
    pub async fn task(&mut self, mut commands: Receiver<'a, DriveCommands>) {
        loop {
            select_biased! {
                // Wait for a command
                command = commands.receive().fuse()=>{
                    self.set_command(&command)
                }
                // If it is in process run again
                _ = self.run_if().fuse() => {}
                // Clippy was complaining
                complete => break
            }
            // Insert a timer to do it again.
            time::delay(10.millis()).await;
        }
    }
}
