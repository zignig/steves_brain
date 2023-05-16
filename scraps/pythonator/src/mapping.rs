//# Simple data type mappings from rust to python 'struct'

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;

// Mapping File
#[derive(Deserialize, Debug)]
pub struct Mapper {
    pub types: HashMap<String, String>,
}
impl Mapper {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }
}

pub fn get_mappings(mapping_file: &str) -> Result<Mapper, Box<dyn std::error::Error>> {
    // Load the mapping file
    let mappings = match fs::read_to_string(mapping_file) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(err) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}` with {} ", mapping_file, err);
            // Exit the program with exit code `1`.
            return Err(Box::new(err));
        }
    };

    let map_data: Mapper = match toml::from_str(&mappings) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(err) => {
            eprintln!("Unable to load data from `{}` with {}", mapping_file, err);
            // Exit the program with exit code `1`.
            return Err(Box::new(err));
        }
    };
    Ok(map_data)
}
