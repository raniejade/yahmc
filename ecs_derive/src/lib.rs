extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Meta, NestedMeta};

#[proc_macro_derive(Component, attributes(Storage))]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;
    // we only have 1 attribute anyway
    // extracts Storage(value)
    let meta = input.attrs[0].interpret_meta().unwrap();
    // extracts (value)
    let pair = match meta {
        Meta::List(mut list) => list.nested.pop().unwrap(),
        _ => panic!("expected Meta::List!"),
    };
    let punc = pair.into_value();
    // extracts value
    let storage = match punc {
        NestedMeta::Meta(meta) => match meta {
            Meta::Word(ident) => ident,
            _ => panic!("expected Meta::Word"),
        },
        _ => panic!("expected Nested::Meta"),
    };

    let expanded = quote! {
        impl Component for #name {
            type Storage = #storage<Self>;
        }
    };

    expanded.into()
}
