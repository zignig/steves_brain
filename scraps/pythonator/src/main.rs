use askama::Template;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process;
use std::process::exit;
use syn::visit::{self, Visit};
use syn::ItemEnum;
use toml;

// Settings Configuration file
#[derive(Deserialize, Debug)]
struct Data {
    settings: Settings,
}

#[derive(Deserialize, Debug)]
struct Settings {
    file: String,
    output: String,
    interval: u16,
    select_pin: u8,
    sck: u8,
    mosi: u8,
    miso: u8,
}

// Mapping File
#[derive(Deserialize, Debug)]
struct Mapper {
    types: HashMap<String, String>,
}
impl Mapper {
    fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }
}

fn main() {
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
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // Load the mapping file
    let mapping_file = "mapping.toml";
    let mappings = match fs::read_to_string(mapping_file) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(err) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}` with {} ", mapping_file, err);
            // Exit the program with exit code `1`.
            exit(1);
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
            exit(1);
        }
    };

    println!("Mapping {:?}", &map_data);

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
    let syntax = syn::parse_file(&contents).expect("Unable to parse file");

    //     // Debug impl is available if Syn is built with "extra-traits" feature.
    //     //println!("{:#?}", &syntax);
    let mut vis = EnumVisitor::new("testing".to_string());
    vis.mapping = map_data;

    vis.visit_file(&syntax);
    println!("{:?}", vis);
    vis.select_pin = data.settings.select_pin;
    vis.sck = data.settings.sck;
    vis.mosi = data.settings.mosi;
    vis.miso = data.settings.miso;
    vis.interval = data.settings.interval;
    vis.scan();

    println!("{:#?}", vis);

    let output = vis.render().unwrap();
    //println!("{}",output);
    let mut output_file = File::create(data.settings.output).expect("Write Fail");
    write!(output_file, "{}", output).expect("cannot write");
}

#[derive(Debug, Template, Clone)]
#[template(source = "", ext = "txt")]
struct Item {
    name: String,
    values: Vec<String>,
    format_string: String,
}

#[derive(Debug, Template)]
#[template(path = "interface.txt")]
struct EnumVisitor {
    name: String,
    current: usize,
    pub items: Vec<Item>,
    mapping: Mapper,
    pub interval: u16,
    pub select_pin: u8,
    pub sck: u8,
    pub mosi: u8,
    pub miso: u8
}

impl EnumVisitor {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            current: 0,
            items: vec![],
            mapping: Mapper::new(),
            interval: 15,
            select_pin: 1,
            sck: 1,
            mosi: 1,
            miso: 1 ,
        }
    }
}

impl EnumVisitor {
    fn scan(&mut self) {
        println!("update formatters");
        for item in self.items.iter_mut() {
            println!("{:?}", item);
            for f in item.values.iter() {
                println!("{:?} - {:?}", item.name, f);
                if let Some(val) = self.mapping.types.get(f) {
                    item.format_string.push_str(&val.clone());
                }
            }
        }
    }
}

impl<'ast> Visit<'ast> for EnumVisitor {
    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        println!("{:?}", i.ident.to_string());
        self.name = i.ident.to_string();
        visit::visit_item_enum(self, i);
    }

    fn visit_variant(&mut self, i: &'ast syn::Variant) {
        let name = i.ident.to_string().clone();
        //println!("\t{:?} - {:?}",name,i.discriminant);
        let item = Item {
            name: name,
            values: vec![],
            format_string: "".to_string(),
        };
        self.items.push(item);
        //println!("{:?}",i);
        visit::visit_variant(self, i);
        self.current = self.items.len();
    }

    fn visit_item_fn(&mut self, _i: &'ast syn::ItemFn) {}

    fn visit_item_impl(&mut self, _i: &'ast syn::ItemImpl) {}

    fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
        let t = i.ident.to_string();
        println!("\t\t{:?} -- {:?} ", t, self.current);
        //let i = self.items.get_mut(self.current).unwrap();
        //println!("{}",i);
        if let Some(val) = self.items.get_mut(self.current) {
            val.values.push(t);
        }
    }
}
