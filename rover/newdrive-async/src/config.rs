/// The config wrangler
/// Uses eeprom on the avr to load and save configs
///

pub struct Wrangler {
    eeprom: arduino_hal::Eeprom,
}

impl Wrangler {
    pub fn new(eeprom: arduino_hal::Eeprom) -> Self {
        Self { eeprom: eeprom }
    }

    pub fn dump(&mut self) {
        let capacity = self.eeprom.capacity();
        for offset in 0..capacity {
            let b = self.eeprom.read_byte(offset);
            crate::print!("{}", b);
        }
    }
}
