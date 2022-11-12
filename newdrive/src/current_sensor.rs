// Current Sensor on analog port 1
// an analog current sensor that measures the amount of
// power that the drive module is using

use arduino_hal::adc::Channel;

use crate::utils::MovingAverage16;
//use embedded_hal::adc::OneShot;

pub struct CurrentSensor {
    channel: Channel,
    value: i16,
    pub zero_offset: i16,
    overload: i16,
    average: MovingAverage16,
}

impl CurrentSensor {
    pub fn new(channel: Channel) -> Self {
        Self {
            channel: channel,
            value: 0,
            zero_offset: 0,
            overload: 0,
            average: MovingAverage16::new(),
        }
    }
    pub fn get_zero(&mut self, adc: &mut arduino_hal::Adc) {
        let mut val: i16 = adc.read_blocking(&self.channel) as i16;
        // get a bunch of readings and average
        for _ in 0..8 {
            val += adc.read_blocking(&self.channel) as i16;
            val = val / 2;
        }
        self.zero_offset = val;
    }

    pub fn get_value(&mut self, adc: &mut arduino_hal::Adc) -> i16 {
        let mut val = adc.read_blocking(&self.channel) as i16;
        val = self.zero_offset - val;
        self.average.feed(val as i16 - self.zero_offset);
        val
    }

    pub fn set_upper(&mut self, val: i16) {
        self.overload = val; 
    }

    pub fn overload(&mut self,adc: &mut arduino_hal::Adc) -> bool {
        if self.get_value(adc) > self.overload {
            true
        } else {
            false
        }
    }
}
