// The various parts of the joystick reader

use crate::serial_println;
use arduino_hal::adc::Channel;
// Single axis

//#[derive(Serialize, Deserialize, PartialEq, SerializedSize)]
pub struct AxisConfig {
    zero: i16,
    min: i16,
    max: i16,
    dead_zone: i16,
}

pub struct Axis {
    channel: Channel,
    zero: i16,
    pub value: i16,
    min: i16,
    max: i16,
    // ? scaling factors
    // ? linearization
    // ?
}

impl Axis {
    fn new(channel: Channel) -> Self {
        Self {
            channel: channel,
            zero: 0,
            value: 0,
            min: -1000,
            max: 1000,
        }
    }

    pub fn get_value(&mut self, adc: &mut arduino_hal::Adc) -> i16 {
        let mut val = adc.read_blocking(&self.channel) as i16;
        val = self.zero - val;
        self.value = val;
        //self.average.feed(val as i16 - self.zero_offset);
        val
    }

    pub fn get_zero(&mut self, adc: &mut arduino_hal::Adc) {
        let mut val: i16 = adc.read_blocking(&self.channel) as i16;
        // get a bunch of readings and average
        for _ in 0..8 {
            val += adc.read_blocking(&self.channel) as i16;
            val = val / 2;
        }
        self.zero = val;
    }
}

pub struct Joy3Axis {
    pub x: Axis,
    pub y: Axis,
    pub z: Axis,
}

impl Joy3Axis {
    pub fn new(chx: Channel, chy: Channel, chz: Channel) -> Self {
        Self {
            x: Axis::new(chx),
            y: Axis::new(chy),
            z: Axis::new(chz),
        }
    }

    pub fn update(&mut self, adc: &mut arduino_hal::Adc) {
        self.x.get_value(adc);
        self.y.get_value(adc);
        self.z.get_value(adc);
    }

    pub fn show(&mut self) {
        serial_println!("X:{}", self.x.value);
        serial_println!("Y:{}", self.y.value);
        serial_println!("Z:{}", self.z.value);
        serial_println!("\n");
    }

    pub fn zero_out(&mut self, adc: &mut arduino_hal::Adc) {
        self.x.get_zero(adc);
        self.y.get_zero(adc);
        self.z.get_zero(adc);
    }
}

pub struct Throttle {
    pub t: Axis,
}

impl Throttle {
    pub fn new(t: Channel) -> Self {
        Self { t: Axis::new(t) }
    }

    pub fn update(&mut self, adc: &mut arduino_hal::Adc) {
        self.t.get_value(adc);
    }

    pub fn show(&mut self) {
        serial_println!("T:{}", self.t.value);
        serial_println!("\n");
    }
}
