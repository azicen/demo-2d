use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, ItemStruct, parse::{self, Parser}, parse_macro_input};

#[proc_macro_derive(Entity)]
pub fn entity_macro_derive(input: TokenStream) -> TokenStream {
    // 将proc_macro::TokenStream转换为非proc_macro类型，以便于测试
    let derive_input = parse_macro_input!(input as DeriveInput);

    // 建立特征的实现
    // entity_attribute_macro(&mut derive_input);
    entity_impl_macro(&derive_input)
}

// 生成结构体属性
#[proc_macro_attribute]
pub fn entity_attribute_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    let field_list = quote! {
        id: u32
    };

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        match syn::Field::parse_named.parse2(field_list) {
            Ok(field) => fields.named.push(field),
            Err(e) => panic!("{}", e),
        };
    }

    return quote! {
        #item_struct
    }
        .into();
}

// 生成结构体方法
fn entity_impl_macro(ast: &DeriveInput) -> TokenStream {
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
    gen.into()
}
