// uart interface 
// micropython to rust

use askama::Template;
use serde_derive::Deserialize;

use crate::items::Item;

#[derive(Deserialize, Debug)]
pub struct UARTSettings {
    tx: u8,
    rx: u8,
    baud: usize,
}