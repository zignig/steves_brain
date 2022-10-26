// Current Sensor on analog port 1 

use arduino_hal::adc::{ AdcOps, Channel } ;

use arduino_hal::prelude::*;
use arduino_hal::port::mode;
use arduino_hal::port::Pin;

use embedded_hal;
//use embedded_hal::adc::OneShot;

pub struct CurrentSensor
//where 
//    PO: arduino_hal::port::PinOps,
{ 
    channel: Channel,
    value: u16,
    overload: u16,
}

impl CurrentSensor
{
    pub fn new(channel: Channel) -> Self{
        Self {
            channel: channel,
            value: 0,
            overload: 0,
        }
    }

    pub fn get_value(&mut self,adc: &mut arduino_hal::Adc) -> u16{ 
        adc.read_blocking(&self.channel)
    }

    pub fn set_upper(&mut self,val: u16) {
        self.overload = val;
    }

    pub fn overload(&mut self) -> bool {
        self.value > self.overload
    }
}