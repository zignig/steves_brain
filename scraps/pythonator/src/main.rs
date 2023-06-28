use askama::Template;
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;
use std::process::exit;

use toml;

mod mapping;
mod items;

use mapping::{get_mappings, Mapper};
use items::EnumVisitor;

mod protocol;
use protocol::spi::SpiSettings;


// Settings Configuration file
#[derive(Deserialize, Debug)]
struct Data {
    settings: Settings,
    spi: Option<SpiSettings>,
    i2c: Option<I2cSettings>,
    uart: Option<UARTSettings>,
}

#[derive(Deserialize, Debug)]
struct UARTSettings {
    tx: u8,
    rx: u8,
    baud: usize,
}
#[derive(Deserialize, Debug)]
struct I2cSettings {
    scl: u8,
    sda: u8,
}

#[derive(Deserialize, Debug)]
struct Settings {
    file: String,
    output: String,
}

fn main() {
    println!("Load Mappings");
    let map_data = match get_mappings("mapping.toml") {
        Ok(map) => map,
        Err(e) => {
            println!("Error {:?}", e);
            exit(1)
        }
    };
    //println!("Mapping {:?}", &map_data);

    let mut args = env::args();
    let _ = args.next(); // executable name
    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: pythonator settings.toml");
            process::exit(1);
        }
    };

    // Load the contents file
    let contents = match fs::read_to_string(filename.as_str()) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(e) => {
            eprintln!("Error `{:#?}` in {}", e, filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    println!("Settings {:#?}\n", data);

    // build the enum cross
    let contents = match fs::read_to_string(&data.settings.file.as_str()) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    }; 

    // Parse the rust file into AST
    let syntax = syn::parse_file(&contents).expect("Unable to parse file");
    //println!("{:#?}",syntax);
    // Get the structure out into an EnumVisitor
    let mut vis = EnumVisitor::new("testing".to_string());
    // bind the mapping
    vis.mapping = map_data;
    // build the extras
    vis.build(&syntax);
    //println!("{:?}", vis);
    //println!("{:?}",data);

    let mut output: String = "unbuilt".to_string();

    if let Some(settings) = data.spi { 
        let spi_out = protocol::spi::SPIExport::build(settings, vis.items);
        //println!("{:?}",spi_out);
        output = spi_out.render().unwrap();
        //println!("{:?}",output);

    }

    let mut output_file = File::create(data.settings.output).expect("Write Fail");
    write!(output_file, "{}", output).expect("cannot write");
}
