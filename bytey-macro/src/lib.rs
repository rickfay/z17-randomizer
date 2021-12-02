use proc_macro2::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, Type, TypeGroup, TypeReference};

#[proc_macro]
#[proc_macro_error]
pub fn strip_lifetime(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Box<Type>);
    let output = strip_lifetime_inner(input);
    output.into()
}

fn strip_lifetime_inner(input: Box<Type>) -> TokenStream {
    match *input {
        Type::Group(TypeGroup { elem, .. }) => strip_lifetime_inner(elem),
        Type::Reference(TypeReference {
            and_token,
            mutability,
            elem,
            ..
        }) => quote!(#and_token #mutability #elem),
        input => quote!(#input),
    }
}
