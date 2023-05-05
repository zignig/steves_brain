// File scanner 

use std::fs;

pub fn get_files(){
    let paths = fs::read_dir("./").unwrap();
    for path in paths { 
        println!("{:?}",path)
    }
}


