use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, LitStr, Token};
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(name);
}

pub struct EventMacroArgs {
    name: LitStr,
}

impl Parse for EventMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut _event_name = None;

        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::name) {
                input.parse::<kw::name>()?;
                input.parse::<Token![=]>()?;
                _event_name = Some(input.parse::<LitStr>()?);
            } else {
                return Err(input.error("expected `name` argument is string literal"));
            }
            if let Err(_) = input.parse::<Token![,]>() {
                break;
            }
        }

        match _event_name {
            Some(name) => Ok(EventMacroArgs { name }),
            None => Err(input.error("`name` argument not exist")),
        }
    }
}

// 生成结构体方法
pub fn event_impl_macro(args: &EventMacroArgs, ast: &DeriveInput) -> TokenStream2 {
    let event_name = &args.name;
    let struct_name = &ast.ident;
    quote! {
        impl #struct_name {
            fn name() -> &'static str {
                static EVENT_NAME: &str = #event_name;
                EVENT_NAME
            }
        }
        impl ::elaiki_api::events::Event for #struct_name {
            fn event_name(&self) -> &'static str {
                Self::name()
            }
            fn as_any(&self) -> & dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> & mut dyn Any {
                self
            }
        }
    }
}
