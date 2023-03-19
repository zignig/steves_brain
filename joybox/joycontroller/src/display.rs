// seven segment display

use crate::serial_println;

use embedded_hal::digital::v2::OutputPin;
use max7219;
use numtoa::NumToA;
use serde::ser::SerializeMap;

use crate::utils::serial_init;

pub struct Display<DATA, CS, SCK>
where
    DATA: OutputPin,
    CS: OutputPin,
    SCK: OutputPin,
{
    d: max7219::MAX7219<max7219::connectors::PinConnector<DATA, CS, SCK>>,
    value: [u8; 8],
}

impl<DATA: OutputPin, CS: OutputPin, SCK: OutputPin> Display<DATA, CS, SCK>
where
    DATA: OutputPin,
    CS: OutputPin,
    SCK: OutputPin,
{
    pub fn new(data: DATA, cs: CS, sck: SCK) -> Self {
        let display = max7219::MAX7219::from_pins(1, data, cs, sck).unwrap();
        Self {
            d: display,
            value: [0; 8], 
        }
    }

    pub fn power_on(&mut self) {
        self.d.power_on().unwrap();
    }
    pub fn power_off(&mut self){
        self.d.power_off();
    }

    pub fn show_help(&mut self) {
        self.d.write_str(0, b"pls help", 0b10101010).unwrap();
    }

    pub fn clear(&mut self) {
        self.d.clear_display(0);
    }

    pub fn brightness(&mut self,bright: u8){
        self.d.set_intensity(0, bright);
    }

    pub fn show_number(&mut self, val: i32) {
        let mut buf = [0u8; 8];
        let mut dis = [0u8; 8];
        let mut j = base_10_bytes(val, &mut buf);
        dis = pad_empty(j);
        //serial_println!("val -> {:?}", dis);
        //serial_println!("{:?}",j);
        self.d.write_str(0, &mut dis, 0b00000000).unwrap();
    }
}

fn base_10_bytes(mut n: i32, buf: &mut [u8]) -> &[u8] {
    if n == 0 {
        return b"0";
    }
    let mut i = 0;
    while n > 0 {
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    let slice = &mut buf[..i];
    slice.reverse();
    &*slice
}

pub fn pad_empty(val: &[u8]) -> [u8; 8] {
    let size: usize = 7;
    let mut pos: usize = val.len() - 1;
    let mut cur: usize = 0;
    let mut out: [u8; 8] = *b"        ";
    while cur <= pos {
        //serial_println!("{} {}",pos,cur);
        out[size - cur] = val[pos - cur];
        cur += 1;
    }
    //while  size > pos  {
    //    out[pos] = val[cur];
    //    pos += 1;
    //    cur += 1;
    //}
    out
}
