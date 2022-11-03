// Current Sensor on analog port 1
// an analog current sensor that measures the amount of
// power that the drive module is using

use arduino_hal::adc::Channel;

use crate::utils::MovingAverage16;
//use embedded_hal::adc::OneShot;

pub struct CurrentSensor {
    channel: Channel,
    value: i16,
    zero_offset: i16,
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
        let mut val: i16 = self.get_value(adc);
        // get a bunch of readings and average
        for _ in 0..8 {
            val += self.get_value(adc);
            val = val / 2;
        }
        self.zero_offset = val;
    }

    pub fn get_value(&mut self, adc: &mut arduino_hal::Adc) -> i16 {
        let val = adc.read_blocking(&self.channel) as i16 - self.zero_offset;
        self.average.feed(val as i16 - self.zero_offset);
        val
    }

    pub fn set_upper(&mut self, val: i16) {
        self.overload = val;
    }

    pub fn overload(&mut self) -> bool {
        self.value > self.overload
    }
}
