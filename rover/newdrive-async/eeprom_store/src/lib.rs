use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(EepromSave)]
pub fn eeprom_save(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let output = quote! {
        impl Saver for #name {
            const size: usize =  #name::MAX_SIZE ;

            fn load(ee: &mut Eeprom) -> Self {
                let mut buf: [u8; #name::size] = [0; #name::size];
                ee.read(0,&mut buf).unwrap();
                let (data,_) = hubpack::deserialize::<#name>(&buf).unwrap();
                data
            }

            fn save(&mut self, ee: &mut Eeprom) {
                let mut buf: [u8; #name::size] = [0; #name::size];
                ee.erase(0,#name::size as u16).unwrap();
                let _ = hubpack::serialize(&mut buf,self);
                let _ = ee.write(0,&buf);
            }
        }
    };
    output.into()
}
