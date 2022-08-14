use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse;
use syn::parse::Parser;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

use crate::entities::eneity::{entity_attribute_macro, entity_impl_macro};
use crate::events::event::{event_impl_macro, EventMacroArgs};
use crate::events::event_revocable::{event_revocable_attribute_macro, event_revocable_impl_macro};

mod entities;
mod events;

fn simple_attribute_macro(input: &mut ItemStruct, field_content: TokenStream2) -> TokenStream2 {
    if let syn::Fields::Named(ref mut fields) = input.fields {
        match syn::Field::parse_named.parse2(field_content) {
            Ok(field) => fields.named.push(field),
            Err(e) => panic!("{}", e),
        };
    }

    quote! {
        #input
    }
}

// 实体基础
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
    }
    .into();
}

// 事件基础
#[proc_macro_attribute]
pub fn event_base(args: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_gen = quote! {
        #item_struct
    };

    let args = parse_macro_input!(args as EventMacroArgs);
    let derive_input = DeriveInput::from(item_struct);
    let impl_gen = event_impl_macro(&args, &derive_input);

    return quote! {
        #struct_gen
        #impl_gen
    }
    .into();
}

// 可撤销的事件基础
#[proc_macro_attribute]
pub fn event_revocable_base(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as parse::Nothing);

    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let struct_gen = event_revocable_attribute_macro(&mut item_struct);

    let derive_input = DeriveInput::from(item_struct);
    let impl_gen = event_revocable_impl_macro(&derive_input);

    return quote! {
        #struct_gen
        #impl_gen
    }
    .into();
}
