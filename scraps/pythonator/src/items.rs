// Parse the file and create the items list form templating.

use crate::mapping::Mapper;
use serde_derive::Deserialize;
use syn::visit::{self, Visit};
use syn::{File, ItemEnum};

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub values: Vec<String>,
    pub format_string: String,
}

#[derive(Debug)]
pub struct EnumVisitor {
    name: String,
    current: usize,
    pub items: Vec<Item>,
    pub mapping: Mapper,
}

impl EnumVisitor {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            current: 0,
            items: vec![],
            mapping: Mapper::new(),
        }
    }
}

impl EnumVisitor {
    // map the rust types across to python sruct with the mapping file
    pub fn scan(&mut self) {
        for item in self.items.iter_mut() {
            //println!("{:?}", item);
            for f in item.values.iter() {
                //println!("{:?} - {:?}", item.name, f);
                if let Some(val) = self.mapping.types.get(f) {
                    item.format_string.push_str(&val.clone());
                }
            }
        }
    }

    // Create the collection of frames to export 
    pub fn build(&mut self, i: &File) -> &mut Self {
        self.visit_file(i);
        self.scan();
        self
    }
}


// The syn visitor functions 
impl<'ast> Visit<'ast> for EnumVisitor {
    // Process the 
    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        println!("{:?}", i.ident.to_string());
        self.name = i.ident.to_string();
        visit::visit_item_enum(self, i);
    }

    // Scan through the variants 
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

    // Some exclusions to do nothing 
    fn visit_item_fn(&mut self, _i: &'ast syn::ItemFn) {}

    fn visit_item_impl(&mut self, _i: &'ast syn::ItemImpl) {}

    // Get the values out of the enum
    fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
        let t = i.ident.to_string();
        //println!("\t\t{:?} -- {:?} ", t, self.current);
        //let i = self.items.get_mut(self.current).unwrap();
        //println!("{}",i);
        if let Some(val) = self.items.get_mut(self.current) {
            val.values.push(t);
        }
    }

    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        //println!("{:#?}", i);
        println!("CONSTANT -- {:#?}", i.ident.to_string());
        println!("VALUE -- {:#?}", i.expr);
    }

    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        println!("LIT -- {:#?}", i);
    }
}
