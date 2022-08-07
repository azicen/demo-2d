use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, ItemStruct};

use crate::simple_attribute_macro;

// 生成结构体属性
pub fn event_revocable_attribute_macro(input: &mut ItemStruct) -> TokenStream2 {
    let field_content = quote! {
        __is_revoke: bool
    };

    simple_attribute_macro(input, field_content)
}

// 生成结构体方法
pub fn event_revocable_impl_macro(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    quote! {
        impl ::elaiki_api::events::EventRevocable for #name {
            fn is_revoke(&self) -> bool {
                self.__is_revoke
            }

            fn revoke(&mut self) {
                self.__is_revoke = true;
            }

            fn set_revoke(&mut self, is: bool) {
                self.__is_revoke = is;
            }
        }
    }
}
