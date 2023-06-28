// Parse the file and create the items list form templating.

use syn::visit::{self, Visit};
use syn::{ItemEnum,File};
use crate::mapping::Mapper;

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    values: Vec<String>,
    format_string: String,
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
    pub fn scan(&mut self) {
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

    pub fn build(&mut self,i: &File) -> &mut Self{
        self.visit_file(i);
        self
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
