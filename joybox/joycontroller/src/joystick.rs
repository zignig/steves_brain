// The various parts of the joystick reader
use crate::serial_println;
use arduino_hal::adc::Channel;
use arduino_hal::Eeprom;
use hubpack::SerializedSize;
use serde_derive::{Deserialize, Serialize};
use ufmt::derive::uDebug;

use avr_device;

// some math stuff
use libm::{self, fabs};
use libm::{acosf, fabsf, fmaxf, roundf, sqrtf};

// Until the eeprom is fixed use a fixed callibration
impl Controls {
    pub fn load_fixed(&mut self) {
        self.joystick.x.config = AxisConfig {
            zero: 501,
            min: -128,
            max: 132,
            dead_zone: 10,
            invert: true,
        };
        self.joystick.y.config = AxisConfig {
            zero: 519,
            min: -130,
            max: 124,
            dead_zone: 10,
            invert: true,
        };
        self.joystick.z.config = AxisConfig {
            zero: 550,
            min: -207,
            max: 225,
            dead_zone: 10,
            invert: false,
        };
        self.throttle.t.config = AxisConfig {
            zero: 643,
            min: 0,
            max: 250,
            dead_zone: 10,
            invert: false,
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
    pub invert: bool,
}

impl AxisConfig {
    pub fn new() -> Self {
        Self {
            zero: 0,
            min: 0,
            max: 0,
            dead_zone: 5,
            invert: false,
        }
    }
}
pub struct Axis {
    channel: Channel,
    pub reading: i16,
    pub value: i8,
    pub prev: i8,
    pub config: AxisConfig,
}

impl Axis {
    const CONFIG_SIZE: u16 = AxisConfig::MAX_SIZE as u16;

    fn new(channel: Channel) -> Self {
        Self {
            channel: channel,
            reading: 0,
            value: 0,
            prev: 0,
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
        // calculate and load the floating point parameters
    }

    pub fn get_value(&mut self, adc: &mut arduino_hal::Adc) -> i16 {
        let mut val = adc.read_blocking(&self.channel) as i16;
        val = self.config.zero - val;
        self.reading = val;
        val
    }

    pub fn get_scaled(&mut self) -> i8 {
        // convert to floating point
        // scale to i8
        let mut fvalue: f32 = self.reading.into();
        let mut fzero: f32 = self.config.zero.into();
        let mut fmax: f32 = (self.config.max as f32) + 1.0;
        let mut fmin: f32 = (self.config.min as f32) - 1.0;
        let mut fdead: f32 = self.config.dead_zone.into();

        let mut fscaled: f32 = 0.0;
        let mut val: i8 = 0;

        // limit the dead_zone
        if self.reading.abs() < self.config.dead_zone {
            return 0;
        }
        // rescale the -ve and +ve values
        if fvalue > 0.0 {
            fscaled = fvalue / fmax;
        }
        if fvalue < 0.0 {
            fscaled = -fvalue / fmin;
        }
        // invert the axis if needed;
        if self.config.invert {
            fscaled = -fscaled;
        }
        // bound check on callibration
        if fscaled > 1.0 {
            fscaled = 1.0;
        }
        if fscaled < -1.0 {
            fscaled = -1.0;
        }
        // convert to u8  and return
        val = (fscaled * 127.0) as i8;
        //serial_println!("val = {:?}", val);
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
        if self.config.min > self.reading {
            self.config.min = self.reading;
        }
        if self.config.max < self.reading {
            self.config.max = self.reading;
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
        serial_println!(
            "X:{} Y:{} Z{}",
            self.x.get_scaled(),
            self.y.get_scaled(),
            self.z.get_scaled()
        );
    }

    fn show_config(&mut self) {
        serial_println!("X:{:?} - {:?}", self.x.config, self.x.reading);
        serial_println!("Y:{:?} - {:?}", self.y.config, self.y.reading);
        serial_println!("Z:{:?} - {:?}", self.z.config, self.z.reading);
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
        serial_println!("T:{}", self.t.get_scaled());
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

    pub fn data(&mut self) -> (i8, i8, i8, i8) {
        let a = self.joystick.x.get_scaled();
        let b = self.joystick.y.get_scaled();
        let c = self.joystick.z.get_scaled();
        let d = self.throttle.t.get_scaled();
        (a, b, c, d)
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
            self.joystick.x.get_scaled(),
            self.joystick.y.get_scaled(),
            self.joystick.z.get_scaled(),
            self.throttle.t.get_scaled()
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
