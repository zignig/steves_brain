use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;
//use quote::quote;
use syn::visit::{self, Visit};
use syn::{ItemEnum };

fn main() {
    let mut args = env::args();
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let mut file = File::open(&filename).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");

    // Debug impl is available if Syn is built with "extra-traits" feature.
    //println!("{:#?}", &syntax);
    let mut  vis = FnVisitor::new("testing".to_string());
    
    vis.visit_file(&syntax);
}

#[derive(Debug)]
struct FnVisitor{
    name: String,
    items: HashMap<String,String>
}

impl FnVisitor { 
    pub fn new(name: String) -> Self{ 
        Self{
            name: name,
            items: HashMap::new()
        }
    }
}

impl<'ast> Visit<'ast> for FnVisitor {

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        println!("{:?}", i.ident.to_string());
        self.name = i.ident.to_string();
        visit::visit_item_enum(self,i);
    }
    
    fn visit_variant(&mut self, i: &'ast syn::Variant) {
        println!("\t{:?} - {:?}", i.ident.to_string(),i.discriminant);        
        println!("{:?}",i)
        //visit::visit_variant(self, i);
    }


    // fn visit_fields(&mut self, i: &'ast syn::Fields) {
    //     //println!("\t\t{:?}",i);
    //     visit::visit_fields(self,i);
    // }

    // fn visit_fields_unnamed(&mut self, i: &'ast syn::FieldsUnnamed) {
    //     //println!("\t\t\t---{:?}",i);   
    //     visit::visit_fields_unnamed(self,i);
    // }

    // fn visit_field(&mut self, i: &'ast syn::Field) {
         
    // }

    // fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
    //     println!("\t\t{:?}",i);  
    // }

    // fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
    //     println!("\t\t{:?}",i);          
    // }
}