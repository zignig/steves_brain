use arduino_hal::Eeprom;
use eeprom_store::EepromSave;
/// The config wrangler
/// Uses eeprom on the avr to load and save configs
///
use hubpack::SerializedSize;
use serde_derive::{Deserialize, Serialize};
use ufmt::derive::uDebug;

#[derive(EepromSave, Serialize, Deserialize, PartialEq, SerializedSize, uDebug)]
// #[derive(Serialize, Deserialize, PartialEq, SerializedSize, uDebug)]
pub struct Test {
    id: u32,
    reference: u32,
    throttle: u32,
}

pub trait Saver {
    fn load(ee: &mut Eeprom) -> Self;
    fn save(&mut self, ee: &mut Eeprom);
}

impl Test {
    pub fn new() -> Self {
        Self {
            id: 4,
            reference: 200,
            throttle: 1000,
        }
    }
}

pub struct Wrangler {
    eeprom: arduino_hal::Eeprom,
}

impl Wrangler {
    pub fn new(eeprom: arduino_hal::Eeprom) -> Self {
        Self { eeprom: eeprom }
    }

    pub fn insert(&mut self,mut obj: impl Saver){ 
        obj.save(&mut self.eeprom);
    }

    pub fn save(&mut self) {
        let mut a = Test::new();
        a.save(&mut self.eeprom);
    }

    pub fn load(&mut self) -> Test {
        Test::load(&mut self.eeprom)
    }
    pub fn dump(&mut self) {
        // let capacity = self.eeprom.capacity();
        for offset in 0..32 {
            let b = self.eeprom.read_byte(offset);
            crate::print!("{}", b);
        }
    }
}
