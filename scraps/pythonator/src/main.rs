use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use syn::visit::{self, Visit};
use syn::{ItemEnum };
use askama::Template;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: pythonator path/to/filename.rs");
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
    //println!("{:?}",vis.items);
    vis.show();
    println!("{}",vis.render().unwrap());
}

#[derive(Debug,Template,Clone)]
#[template(path="item.txt")]
struct Item{ 
    name: String , 
    values: Vec<String>
}

#[derive(Debug,Template)]
#[template(path="test.txt")]
struct FnVisitor{
    name: String,
    current: usize,
    pub items: Vec<Item>
}

impl FnVisitor { 
    pub fn new(name: String) -> Self{ 
        Self{
            name: name,
            current: 0,
            items: vec![]
        }
    }
}

impl FnVisitor { 
    fn show(&mut self){
        for item in self.items.iter(){
            println!("{:?}",item);
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
        let name = i.ident.to_string().clone();
        //println!("\t{:?} - {:?}",name,i.discriminant);
        let item = Item{ name : name , values : vec![]};
        self.items.push(item);
        //println!("{:?}",i);
        visit::visit_variant(self, i);
        self.current  = self.items.len();
    }

    fn visit_item_fn(&mut self,_i: &'ast syn::ItemFn) {
        
    }

    fn visit_item_impl(&mut self, _i: &'ast syn::ItemImpl) {
        
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

    fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
        let t = i.ident.to_string();
        println!("\t\t{:?} -- {:?} ",t,self.current);
        //let i = self.items.get_mut(self.current).unwrap();
        //println!("{}",i);
        if let Some(val)= self.items.get_mut(self.current){
             println!("bork {}",val);
             val.values.push(t);
        }
    } 
}