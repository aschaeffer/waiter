extern crate proc_macro;
extern crate syn;
extern crate regex;

use proc_macro::TokenStream;
use syn::*;
use component::generate_component_impl;
use provider::*;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::parse::Parser;
use std::str::FromStr;
use regex::Regex;
use syn::spanned::Spanned;

mod component;
mod provider;

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let comp = syn::parse::<ItemStruct>(item.clone()).unwrap();

    let mut res: TokenStream = TokenStream::from_str(
        Regex::new(r"#\[prop\(.+?\)]").unwrap()
            .replace_all(&item.to_string(), "")
            .as_ref()
    ).unwrap_or_default();

    res.extend(generate_component_impl(comp.clone()));
    res.extend(generate_component_provider_impl_struct(comp.clone()));

    return res;
}

#[proc_macro_attribute]
pub fn provides(attr: TokenStream, item: TokenStream) -> TokenStream {
    let profiles = <Punctuated<Path, Comma>>::parse_terminated.parse(attr)
        .expect("Can't parse profiles");
    let profiles: Vec<&Path> = profiles
        .iter()
        .collect();

    let mut res = item.clone();

    let impl_block = syn::parse::<ItemImpl>(item.clone());
    if impl_block.is_ok() {
        res.extend(generate_interface_provider_impl(profiles, impl_block.unwrap().clone()));
    } else {
        let fn_block = syn::parse::<ItemFn>(item.clone())
            .expect("#[provides] must be used only on impl blocks and factory functions");

        res.extend(generate_component_provider_impl_fn(profiles, fn_block.clone()));
    }

    return res;
}

#[proc_macro_attribute]
pub fn wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let wrapper = parse_macro_input!(item as ItemStruct);

    let type_to_wrap = if let Fields::Unnamed(fields) = &wrapper.fields {
        let field = fields.unnamed.first();
        if field.is_none() {
            return TokenStream::from(
                Error::new(wrapper.span(), "Struct annotated #[wrapper] must have exactly one field")
                    .to_compile_error()
            );
        }

        field.unwrap().ty.clone()
    } else {
        return TokenStream::from(
            Error::new(wrapper.span(), "Only tuple like struct supported for #[wrapper]")
                .to_compile_error()
        );
    };

    let wrapper_name = &wrapper.ident;

    return TokenStream::from(quote::quote! {
        #wrapper
        impl Deref for #wrapper_name {
            type Target = #type_to_wrap;

            fn deref(&self) -> &Self::Target {
                return &self.0;
            }
        }
    });
}