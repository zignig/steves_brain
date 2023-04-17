// The various parts of the joystick reader
use crate::{serial_println};
use arduino_hal::adc::Channel;
use arduino_hal::Eeprom;
use hubpack::SerializedSize;
use serde_derive::{Deserialize, Serialize};
use ufmt::derive::uDebug;

use avr_device;


// Until the eeprom is fixed use a fixed callibration
impl Controls {
    pub fn load_fixed(&mut self) {
        self.joystick.x.config = AxisConfig {
            zero: 499,
            min: -129,
            max: 129,
            dead_zone: -20,
        };
        self.joystick.y.config = AxisConfig {
            zero: 519,
            min: -129,
            max: 125,
            dead_zone: -20,
        };
        self.joystick.z.config = AxisConfig {
            zero: 553,
            min: -203,
            max: 226,
            dead_zone: -20,
        };
        self.throttle.t.config = AxisConfig {
            zero: 642,
            min: 0,
            max: 250,
            dead_zone: -20,
        };
    }
}
// Single axis
#[derive(uDebug)]
pub enum Mode {
    Running,
    RunCallibrate,
}

pub trait AnalogController {
    fn save(&mut self, ee: &mut Eeprom);
    fn load(&mut self, ee: &mut Eeprom);
    //fn read(&mut self) -> Option<Command>;
    fn update(&mut self, mode: &Mode, adc: &mut arduino_hal::Adc);
    fn show(&mut self);
    fn show_config(&mut self);
    fn zero_out(&mut self, adc: &mut arduino_hal::Adc);
    fn resetcal(&mut self);
}

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
            zero: 0,
            min: 0,
            max: 0,
            dead_zone: -20,
        }
    }
}
pub struct Axis {
    channel: Channel,
    pub value: i16,
    pub test: f32,
    pub config: AxisConfig,
}

impl Axis {
    const CONFIG_SIZE: u16 = AxisConfig::MAX_SIZE as u16;

    fn new(channel: Channel) -> Self {
        Self {
            channel: channel,
            value: 0,
            test: 0.0,
            config: AxisConfig::new(),
        }
    }
}

impl Axis {
    fn save(&mut self, ee: &mut Eeprom, slot: u16) {
        let offset = slot * Axis::CONFIG_SIZE;
        let mut buf: [u8; Axis::CONFIG_SIZE as usize] = [0; Axis::CONFIG_SIZE as usize];
        ee.erase(offset, offset + Axis::CONFIG_SIZE).unwrap();
        let _ = hubpack::serialize(&mut buf, &self.config);
        //serial_println!("> {:?}", buf[..]);
        let err = ee.write(offset, &buf);
        match err {
            Ok(_) => {}
            Err(e) => serial_println!("{:?}", e),
        }
        // read back
        //ee.read(offset, &mut buf).unwrap();
        //serial_println!("< {:?}", buf[..]);
    }

    fn load(&mut self, ee: &mut Eeprom, slot: u16) {
        let offset = slot * Axis::CONFIG_SIZE;
        let mut buf: [u8; Axis::CONFIG_SIZE as usize] = [0; Axis::CONFIG_SIZE as usize];
        ee.read(offset, &mut buf).unwrap();
        let (config, _) = hubpack::deserialize::<AxisConfig>(&buf).unwrap();
        self.config = config;
    }

    pub fn get_value(&mut self, adc: &mut arduino_hal::Adc) -> i16 {
        let mut val = adc.read_blocking(&self.channel) as i16;
        val = self.config.zero - val;
        self.value = val;
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

    pub fn callibrate(&mut self, adc: &mut arduino_hal::Adc) {
        self.get_value(adc);
        if self.config.min > self.value {
            self.config.min = self.value;
        }
        if self.config.max < self.value {
            self.config.max = self.value;
        }
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
}

impl AnalogController for Joy3Axis {
    fn load(&mut self, ee: &mut Eeprom) {
        self.x.load(ee, 0);
        self.y.load(ee, 1);
        self.z.load(ee, 2);
    }

    fn save(&mut self, ee: &mut Eeprom) {
        self.x.save(ee, 0);
        self.y.save(ee, 1);
        self.z.save(ee, 2);
    }

    fn update(&mut self, mode: &Mode, adc: &mut arduino_hal::Adc) {
        match mode {
            Mode::Running => {
                self.x.get_value(adc);
                self.y.get_value(adc);
                self.z.get_value(adc);
            }
            Mode::RunCallibrate => {
                self.x.callibrate(adc);
                self.y.callibrate(adc);
                self.z.callibrate(adc);
            }
        }
    }

    fn show(&mut self) {
        serial_println!("X:{} Y:{} Z{}", self.x.value, self.y.value, self.z.value);
    }

    fn show_config(&mut self) {
        serial_println!("X:{:?} - {:?}", self.x.config, self.x.value);
        serial_println!("Y:{:?} - {:?}", self.y.config, self.y.value);
        serial_println!("Z:{:?} - {:?}", self.z.config, self.z.value);
        //serial_println!("\n");
    }

    fn zero_out(&mut self, adc: &mut arduino_hal::Adc) {
        self.x.get_zero(adc);
        self.y.get_zero(adc);
        self.z.get_zero(adc);
    }

    fn resetcal(&mut self) {
        self.x.config = AxisConfig::new();
        self.y.config = AxisConfig::new();
        self.z.config = AxisConfig::new();
    }
}

pub struct Throttle {
    pub t: Axis,
    pub mode: Mode,
}

impl Throttle {
    pub fn new(t: Channel) -> Self {
        let throt = Axis::new(t);
        // the zeroing on the throttle is different
        Self {
            t: throt,
            mode: Mode::Running,
        }
    }
}

impl AnalogController for Throttle {
    fn load(&mut self, ee: &mut Eeprom) {
        self.t.load(ee, 3);
    }

    fn save(&mut self, ee: &mut Eeprom) {
        self.t.save(ee, 3);
    }

    fn show(&mut self) {
        serial_println!("T:{}", self.t.value);
    }

    fn update(&mut self, mode: &Mode, adc: &mut arduino_hal::Adc) {
        match mode {
            Mode::Running => {
                self.t.get_value(adc);
            }
            Mode::RunCallibrate => {
                self.t.callibrate(adc);
            }
        }
    }
    fn show_config(&mut self) {
        serial_println!("T:{:?} - {:?}", self.t.config, self.t.value);
    }
    fn zero_out(&mut self, adc: &mut arduino_hal::Adc) {
        self.t.get_zero(adc);
    }

    fn resetcal(&mut self) {
        self.t.config = AxisConfig::new();
    }
}

pub struct Controls {
    pub joystick: Joy3Axis,
    pub throttle: Throttle,
}

impl Controls {
    pub fn new(joystick: Joy3Axis, throttle: Throttle) -> Self {
        Self { joystick, throttle }
    }
}

impl AnalogController for Controls {
    fn save(&mut self, ee: &mut Eeprom) {
        avr_device::interrupt::free(|_| {
            self.joystick.save(ee);
            self.throttle.save(ee);
        });
        serial_println!("save controls to eeprom");
    }

    fn load(&mut self, ee: &mut Eeprom) {
        self.joystick.load(ee);
        self.throttle.load(ee);
    }

    fn update(&mut self, mode: &Mode, adc: &mut arduino_hal::Adc) {
        self.joystick.update(mode, adc);
        self.throttle.update(mode, adc);
    }

    fn show(&mut self) {
        serial_println!(
            "X : {:?} , Y : {:?} , Z : {:?} , T : {:?}",
            self.joystick.x.value,
            self.joystick.y.value,
            self.joystick.z.value,
            self.throttle.t.value
        );
        //self.joystick.show();
        //self.throttle.show();
    }

    fn show_config(&mut self) {
        self.joystick.show_config();
        self.throttle.show_config();
        //serial_println!("\n");
    }

    fn zero_out(&mut self, adc: &mut arduino_hal::Adc) {
        self.joystick.zero_out(adc);
        self.throttle.zero_out(adc);
    }

    fn resetcal(&mut self) {
        self.joystick.resetcal();
        self.throttle.resetcal();
    }
}
