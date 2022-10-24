// Current Sensor on analog port 1 

use arduino_hal::adc::AdcOps;
use arduino_hal::prelude::*;
use arduino_hal::port::mode;
use arduino_hal::port::Pin;

use embedded_hal;
//use embedded_hal::adc::OneShot;

pub struct CurrentSensor<PIN>
//where 
//    PO: arduino_hal::port::PinOps,
{ 
    pin: Pin<mode::Analog,PIN>,
    value: u16,
    overload: u16,
}

impl<PIN> CurrentSensor<PIN>
{
    pub fn new(pin: Pin<mode::Analog,PIN>) -> Self{
        Self {
            pin: pin,
            value: 0,
            overload: 0,
        }
    }

    pub fn get_value(&mut self,adc: arduino_hal::Adc) -> u16{ 
        self.pin.analog_read(&adc);
        0
    }

    pub fn set_upper(&mut self,val: u16) {
        self.overload = val;
    }

    pub fn overload(&mut self) -> bool {
        self.value > self.overload
    }
}