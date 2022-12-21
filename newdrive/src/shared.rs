//use crate::serial_println;
//use arduino_hal::prelude::*;

pub trait Update {
    fn update(&mut self);
}

pub trait TankDrive {
    fn update(&mut self);
    fn enable(&mut self);
    fn disable(&mut self);
    fn stop(&mut self);
    fn set_speed(&mut self, l_speed: i16, r_speed: i16);
    fn set_timeout(&mut self, timeout: i16);
    fn set_min(&mut self, val: u8);
    fn set_rate(&mut self, rate: u8);
    fn get_movement(&self) -> Option<(i16, i16)>;
    fn set_joy(&mut self, x: i16, y: i16);
}
