use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{DeriveInput, ItemStruct, parse::{self, Parser}, parse_macro_input};

#[proc_macro_attribute]
pub fn entity_base(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as parse::Nothing);

    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let struct_gen = entity_attribute_macro(&mut item_struct);

    let derive_input = DeriveInput::from(item_struct);
    let impl_gen = entity_impl_macro(&derive_input);

    return quote! {
        #struct_gen
        #impl_gen
    }.into()
}

// 生成结构体属性
fn entity_attribute_macro(input: &mut ItemStruct) -> TokenStream2 {
    let field_list = quote! {
        id: u32
    };

    if let syn::Fields::Named(ref mut fields) = input.fields {
        match syn::Field::parse_named.parse2(field_list) {
            Ok(field) => fields.named.push(field),
            Err(e) => panic!("{}", e),
        };
    }

    quote! {
        #input
    }
}

// 生成结构体方法
fn entity_impl_macro(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let gen = quote! {
        impl ::elaiki_api::entities::Entity for #name {
            fn id(&self) -> u32 {
                self.id
            }

            fn set_id(&mut self, id: u32) {
                self.id = id
            }
        }
    };
    gen
}
