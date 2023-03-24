// The various parts of the joystick reader
use crate::serial_println;
use arduino_hal::adc::Channel;
use arduino_hal::Eeprom;
use hubpack::SerializedSize;
use serde_derive::{Deserialize, Serialize};
use ufmt::derive::uDebug;
// Single axis

#[derive(Serialize, Deserialize, PartialEq, SerializedSize, uDebug)]
pub struct AxisConfig {
    pub zero: i16,
    pub min: i16,
    pub max: i16,
    pub dead_zone: i16,
}

impl AxisConfig {
    pub fn new() -> Self {
        Self {
            zero: 128,
            min: -255,
            max: 255,
            dead_zone: 5,
        }
    }
}
pub struct Axis {
    channel: Channel,
    pub value: i16,
    pub config: AxisConfig, 
}

impl Axis {

    const CONFIG_SIZE: u16 = AxisConfig::MAX_SIZE as u16;

    fn new(channel: Channel) -> Self {
        Self {
            channel: channel,
            value: 0,
            config: AxisConfig::new(),
        }
    }

    pub fn save(&mut self, ee: &mut Eeprom,slot: u16){
        let offset = slot * Axis::CONFIG_SIZE;
        let mut buf: [u8;Axis::CONFIG_SIZE as usize] = [0;Axis::CONFIG_SIZE as usize];
        let _ = hubpack::serialize(&mut buf, &self.config);
        ee.write(offset,&buf).unwrap();
    }

    pub fn load(&mut self, ee: &mut Eeprom,slot: u16){
        let offset = slot * Axis::CONFIG_SIZE;
        let mut buf: [u8;Axis::CONFIG_SIZE as usize] = [0;Axis::CONFIG_SIZE as usize];
        ee.read(offset, &mut buf).unwrap();
        let (config,_) = hubpack::deserialize::<AxisConfig>(&buf).unwrap();
        self.config = config;
    }

    pub fn get_value(&mut self, adc: &mut arduino_hal::Adc) -> i16 {
        let mut val = adc.read_blocking(&self.channel) as i16;
        val = self.config.zero - val;
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
        self.config.zero = val;
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

    pub fn load(&mut self,ee: &mut Eeprom){
        self.x.load(ee, 1);
        self.y.load(ee,2);
        self.z.load(ee,3);
    }

    pub fn save(&mut self,ee: &mut Eeprom){
        self.x.save(ee, 1);
        self.y.save(ee,2);
        self.z.save(ee,3);
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

    pub fn show_config(&mut self){
        serial_println!("X:{:#?}", self.x.config);
        serial_println!("Y:{:#?}", self.y.config);
        serial_println!("Z:{:#?}", self.z.config);
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
