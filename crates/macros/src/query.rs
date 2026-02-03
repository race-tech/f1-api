use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemFn, Token, Type, parse_macro_input};

struct QueryArgs {
    fields: Vec<(Ident, Type)>,
    output: Option<Type>,
}

impl syn::parse::Parse for QueryArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut output = None;
        let mut fields = Vec::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let ty = input.parse()?;

            if ident.to_string().as_str() == "output" {
                output = Some(ty);
            } else {
                fields.push((ident, ty));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(QueryArgs { fields, output })
    }
}

pub fn query_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse::<QueryArgs>(args).expect("invalid args found");
    let mut input = parse_macro_input!(input as ItemFn);

    let fn_name = input.sig.ident.clone();
    let new_fn_name = format_ident!("__{}", fn_name);
    input.sig.ident = new_fn_name.clone();
    let is_async = input
        .sig
        .asyncness
        .map(|_| {
            quote! {.await}
        })
        .unwrap_or_default();
    let output = &args.output;

    let register_inputs: Vec<proc_macro2::TokenStream> = args
        .fields
        .iter()
        .map(|f| {
            let ty = &f.1;
            quote! {
                #ty::register(builder);
            }
        })
        .collect();
    let inputs: Vec<proc_macro2::TokenStream> = args
        .fields
        .iter()
        .map(|f| {
            let ident = &f.0;
            let ty = &f.1;
            quote! {
                .argument(async_graphql::dynamic::InputValue::new(
                    stringify!(#ident),
                    #ty::gql_input_type_ref(context),
                ))
            }
        })
        .collect();

    let register_output = args
        .output
        .clone()
        .map(|ty| {
            quote! {
                #ty::register(builder);
            }
        })
        .unwrap_or_default();

    let expanded = quote! {
        fn #fn_name(builder: &mut seaography::Builder) {
            use OutputObject;
            use InputObject;
            let context = builder.context;

            #input

            let query = async_graphql::dynamic::Field::new(
                stringify!(#fn_name),
                #output::gql_output_type_ref(context),
                move |ctx| {
                    async_graphql::dynamic::FieldFuture::new(async move {
                        let db = ctx.data::<DatabaseConnection>()?;

                        let res = #new_fn_name(db)#is_async?;
                        Ok(Some(async_graphql::dynamic::FieldValue::owned_any(res)))
                    })
                }
            )#(#inputs)*;

            #(#register_inputs)*
            #register_output

            builder.queries.push(query);
        }
    };

    eprintln!("{}", expanded);
    TokenStream::from(expanded)
}
