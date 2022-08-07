use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, ItemStruct};

use crate::simple_attribute_macro;

// 生成结构体属性
pub fn entity_attribute_macro(input: &mut ItemStruct) -> TokenStream2 {
    let field_content = quote! {
        __id: u32
    };

    simple_attribute_macro(input, field_content)
}

// 生成结构体方法
pub fn entity_impl_macro(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    quote! {
        impl ::elaiki_api::entities::Entity for #name {
            fn id(&self) -> u32 {
                self.__id
            }

            fn set_id(&mut self, id: u32) {
                self.__id = id
            }
        }
    }
}
