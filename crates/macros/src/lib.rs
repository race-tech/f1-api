use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn, parse_macro_input};

mod filter_input_object;
mod output_object;
mod query;

#[proc_macro_derive(OutputObject)]
pub fn output_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match output_object::expand(input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn query(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse::<query::QueryArgs>(args).expect("invalid args found");
    let input = parse_macro_input!(input as ItemFn);

    match query::expand(args, input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(FilterInputObject, attributes(filter_input_object))]
pub fn filter_input_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match filter_input_object::expand(input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
