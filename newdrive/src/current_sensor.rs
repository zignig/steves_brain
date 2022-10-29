// Current Sensor on analog port 1 
// an analog current sensor that measures the amount of 
// power that the drive module is using

use arduino_hal::adc::{ AdcOps, Channel } ;

use arduino_hal::prelude::*;
use arduino_hal::port::mode;
use arduino_hal::port::Pin;

use embedded_hal;

use crate::utils::MovingAverage16;
//use embedded_hal::adc::OneShot;

pub struct CurrentSensor
{ 
    channel: Channel,
    value: i16,
    overload: i16,
    average: MovingAverage16,
}

impl CurrentSensor
{
    pub fn new(channel: Channel) -> Self{
        Self {
            channel: channel,
            value: 0,
            overload: 0,
            average: MovingAverage16::new(),
        }
    }

    pub fn get_value(&mut self,adc: &mut arduino_hal::Adc) -> i16{ 
        let val = adc.read_blocking(&self.channel) as i16;
        self.average.feed(val as i16);
        val
    }

    pub fn set_upper(&mut self,val: i16) {
        self.overload = val;
    }

    pub fn overload(&mut self) -> bool {
        self.value > self.overload
    }
}