// I2c interface
// Micropython master to slave
use askama::Template;
use serde_derive::Deserialize;

use crate::items::Item;

#[derive(Deserialize, Debug)]
pub struct I2cSettings {
    address: u8,
    scl: u8,
    sda: u8,
}

#[derive(Deserialize,Template,Debug)]
#[template(path="i2c.txt")]
pub struct I2CExport {
    items: Vec<Item>,
    address: u8,
    scl: u8,
    sda: u8,
}

impl I2CExport { 
    pub fn build(settings: I2cSettings,items:Vec<Item>) -> Self { 
        Self { 
            items: items,
            address: settings.address,
            scl: settings.scl,
            sda: settings.sda,
        }
    }
}