use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemFn, Token, Type};

pub struct QueryArgs {
    filters: Option<Type>,
    output_object: Option<Type>,
    output_type: Option<Type>,
    fields: Vec<(Ident, Type)>,
}

impl syn::parse::Parse for QueryArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut output_object = None;
        let mut output_type = None;
        let mut filters = None;
        let mut fields = Vec::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let ty = input.parse::<Type>()?;

            match ident.to_string().as_str() {
                "output_object" => output_object = Some(ty),
                "output_type" => output_type = Some(ty),
                "filters" => {
                    filters = Some(ty.clone());
                    fields.push((ident, ty));
                }
                _ => {
                    return Err(syn::Error::new(input.span(), "Unexpected ident"));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(QueryArgs {
            fields,
            output_object,
            output_type,
            filters,
        })
    }
}

fn expand_inputs(fields: &[(Ident, Type)]) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let register_inputs: Vec<TokenStream> = fields
        .iter()
        .map(|(_, ty)| {
            quote! {
                #ty::register(builder);
            }
        })
        .collect();
    let inputs: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, ty)| {
            quote! {
                .argument(async_graphql::dynamic::InputValue::new(
                    stringify!(#ident),
                    #ty::gql_input_type_ref(context),
                ))
            }
        })
        .collect();

    (register_inputs, inputs)
}

pub fn expand(args: QueryArgs, mut input: ItemFn) -> syn::Result<TokenStream> {
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

    let output_type = &args.output_type;
    let (register_inputs, inputs) = expand_inputs(&args.fields);

    let filters = args
        .filters
        .map(|ty| {
            quote! {
                let condition = #ty::build_filter(context, ctx);
            }
        })
        .unwrap_or_default();

    let register_output = args
        .output_object
        .clone()
        .map(|ty| {
            quote! {
                fn register_output<O: seaography::CustomOutputObject>(builder: &mut seaography::Builder) {
                    let output_object = O::basic_object(builder.context);
                   
                    builder.outputs.extend([output_object]);
                }

                register_output::<#ty>(builder);
            }
        })
        .unwrap_or_default();

    Ok(quote! {
        fn #fn_name(builder: &mut seaography::Builder) {
            use OutputObject;
            use InputObject;
            let context = builder.context;

            #input

            let query = async_graphql::dynamic::Field::new(
                stringify!(#fn_name),
                #output_type::gql_output_type_ref(context),
                move |ctx| {
                    async_graphql::dynamic::FieldFuture::new(async move {
                        let db = ctx.data::<DatabaseConnection>()?;
                        let condition = sea_orm::Condition::all();
                        #filters

                        let res = #new_fn_name(db, condition)#is_async?;
                        Ok(res.gql_field_value(context))
                    })
                }
            )#(#inputs)*;


            #(#register_inputs)*
            #register_output

            builder.queries.push(query);
        }
    })
}
