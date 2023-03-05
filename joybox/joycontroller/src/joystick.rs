// The various parts of the joystick reader

use arduino_hal::adc::Channel;
use crate::serial_println;
// Single axis

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

    pub fn get_value(&mut self,adc: &mut arduino_hal::Adc) -> i16 {
        let mut val = adc.read_blocking(&self.channel) as i16;
        //val = self.zero_offset - val;
        self.value = val;
        //self.average.feed(val as i16 - self.zero_offset);
        val
    }

    
}

pub struct Joy3Axis {
    pub x: Axis,
    pub y: Axis,
    pub z: Axis,
}

impl Joy3Axis {
    pub fn new(chx: Channel,chy: Channel,chz: Channel) -> Self {
        Self {
            x: Axis::new(chx),
            y: Axis::new(chy),
            z: Axis::new(chz),
        }
    }

    pub fn update(&mut self,adc : &mut arduino_hal::Adc){
        self.x.get_value(adc);
        self.y.get_value(adc);
        self.z.get_value(adc);
    }

    pub fn show(&mut self){
        serial_println!("X:{}",self.x.value);
        serial_println!("Y:{}",self.y.value);
        serial_println!("Z:{}",self.z.value);
        serial_println!("\n");
    }
}
