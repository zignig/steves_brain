//use crate::serial_println;
//use arduino_hal::prelude::*;

pub trait Update {
    fn update(&mut self);
}
