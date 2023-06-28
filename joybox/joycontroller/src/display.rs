// seven segment display

use embedded_hal::digital::v2::OutputPin;
use max7219;

enum Direction {
    left,
    right,
}
pub struct Display<DATA, CS, SCK>
where
    DATA: OutputPin,
    CS: OutputPin,
    SCK: OutputPin,
{
    d: max7219::MAX7219<max7219::connectors::PinConnector<DATA, CS, SCK>>,
    pos: usize,
    dir: Direction,
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
            pos: 0,
            dir: Direction::left,
        }
    }

    pub fn power_on(&mut self) {
        self.d.power_on().unwrap();
    }

    pub fn power_off(&mut self) {
        self.d.power_off().unwrap();
    }

    pub fn clear(&mut self) {
        self.d.clear_display(0).unwrap();
    }

    pub fn brightness(&mut self, bright: u8) {
        self.d.set_intensity(0, bright).unwrap();
    }

    pub fn show_hex(&mut self, value: u32) {
        self.d.write_hex(0, value).expect("");
    }

    pub fn show_number(&mut self, value: i32) {
        self.d.write_integer(0, value).expect("");
        // let mut buf = [0u8; 8];

        // let mut dis = [0u8; 8];
        // let mut j = base_10_bytes(val, &mut buf);
        // dis = pad_empty(j);
        // //serial_println!("val -> {:?}", dis);
        // //serial_println!("{:?}",j);
        // self.d.write_str(0, &mut dis, 0b00000000).unwrap();
    }
    pub fn scanner(&mut self) {
        let mut out: [u8; 8] = *b"        ";
        match self.dir {
            Direction::left => self.pos += 1,
            Direction::right => self.pos -= 1,
        }
        if self.pos == 7 {
            self.dir = Direction::right;
        } else if self.pos == 0 {
            self.dir = Direction::left;
        }
        let mut dots: u8 = 1;
        dots = dots << self.pos;
        self.d.write_str(0, &mut out, dots).unwrap()
    }
}

fn base_10_bytes(mut n: i32, buf: &mut [u8]) -> &[u8] {
    let mut sign: bool = false;
    if n < 0 {
        n = -n;
        sign = true;
    }
    if n == 0 {
        return b"0";
    }
    // don't overflow the display
    if n >= 99999999 || n <= -999999 {
        return b"Err";
    }
    let mut i = 0;
    while n > 0 {
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    if sign {
        buf[i] = b'-';
        i += 1;
    }
    let slice = &mut buf[..i];
    slice.reverse();
    &*slice
}

fn pad_empty(val: &[u8]) -> [u8; 8] {
    let size: usize = 8;
    let pos: usize = val.len();
    let mut cur: usize = 1;
    let mut out: [u8; 8] = *b"        ";
    while cur <= pos {
        out[size - cur] = val[pos - cur];
        cur += 1;
    }
    out
}
