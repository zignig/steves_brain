use askama::Template;
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;
use std::process::exit;

use toml;

mod items;
mod mapping;

use items::EnumVisitor;
use mapping::get_mappings;

mod protocol;
use protocol::spi::SpiSettings;
use protocol::i2c::I2cSettings;
use protocol::uart::UARTSettings;


// Settings Configuration file
#[derive(Deserialize, Debug)]
struct Data {
    settings: Settings,
    spi: Option<SpiSettings>,
    i2c: Option<I2cSettings>,
    uart: Option<UARTSettings>,
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

    // Load the rust file to scan
    let settings  = match fs::read_to_string(filename.as_str()) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    // Load the config file from toml
    let data: Data = match toml::from_str(&settings) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error `{:#?}` in {}", e, filename);
            exit(1);
        }
    };

    // Show the settings
    println!("Settings {:#?}\n", data);

    // build the enum cross
    let contents = match fs::read_to_string(&data.settings.file.as_str()) {
        Ok(c) => c,
        Err(_) => {
            println!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    // Parse the rust file into AST
    let syntax = syn::parse_file(&contents).expect("Unable to parse file");
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
    else if let Some(settings) = data.i2c { 
        let i2c_out = protocol::i2c::I2CExport::build(settings, vis.items);
        output = i2c_out.render().unwrap();
    }

    let mut output_file = File::create(data.settings.output).expect("Write Fail");
    write!(output_file, "{}", output).expect("cannot write");
}
