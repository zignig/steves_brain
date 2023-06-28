// Spi interface
// micropython master to RUST slave
use askama::Template;
use serde_derive::Deserialize;

use crate::items::Item;

#[derive(Deserialize,Debug)]
pub struct SpiSettings {
    interval: u16,
    select_pin: u8,
    sck: u8,
    mosi: u8,
    miso: u8,
}

#[derive(Deserialize, Template, Debug)]
#[template(path = "spi.txt")]
pub struct SPIExport {
    items: Vec<Item>,
    interval: u16,
    select_pin: u8,
    sck: u8,
    mosi: u8,
    miso: u8
}

impl SPIExport {
    pub fn build(settings: SpiSettings,items:Vec<Item>) -> Self {
       Self { 
        items: items,
        interval: settings.interval,
        select_pin: settings.select_pin,
        sck: settings.sck,
        mosi: settings.mosi,
        miso: settings.miso,
       } 
    }
}