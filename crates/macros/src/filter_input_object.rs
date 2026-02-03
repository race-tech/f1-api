use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataStruct, DeriveInput, Error, Fields, Ident, Path, Token, TypePath, spanned::Spanned,
};

#[derive(Debug)]
pub struct FilterInputObjectFieldAttr {
    entity: TypePath,
    column: Path,
}

impl syn::parse::Parse for FilterInputObjectFieldAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entity: Option<TypePath> = None;
        let mut column: Option<Path> = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "entity" => {
                    entity = Some(input.parse()?);
                }
                "column" => column = Some(input.parse()?),
                other => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown argument `{other}`"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        match (entity, column) {
            (Some(entity), Some(column)) => Ok(FilterInputObjectFieldAttr { entity, column }),
            _ => Err(input.error("invalid attributes for named field")),
        }
    }
}

fn derive_filter_input_object_struct(
    input: &DeriveInput,
    data_struct: &DataStruct,
) -> syn::Result<TokenStream> {
    let Fields::Named(named) = &data_struct.fields else {
        return Err(Error::new(input.ident.span(), "Expected named fields"));
    };

    let fields = named.named.iter().try_fold(
        Vec::<(Ident, FilterInputObjectFieldAttr)>::new(),
        |mut acc, f| {
            let field_name = f.ident.clone().unwrap();
            if let Some(attr) = f
                .attrs
                .iter()
                .find(|a| a.path().is_ident("filter_input_object"))
            {
                let attrs = attr.parse_args::<FilterInputObjectFieldAttr>()?;

                acc.push((field_name, attrs));
            } else {
                return Err(Error::new(
                    f.ident.span(),
                    "fields needs attributes in order to work",
                ));
            }

            Ok(acc)
        },
    )?;

    let input_fields: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|(_, attrs)| {
            let column = &attrs.column;
            let entity = &attrs.entity;
            quote! {
                match _filter_types_map_helper.get_column_filter_input_value::<#entity>(&#column) {
                    Some(field) => input.field(field),
                    None => input,
                }
            }
        })
        .collect();
    let filter_fields: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|(field_name, attrs)| {
            let column = &attrs.column;
            let entity = &attrs.entity;
            quote! {
                if let Some(#field_name) = filters.get(stringify!(#field_name)) {
                    let #field_name = #field_name.object().expect("cannot convert field to object");
                    condition = filter_types_map_helper
                        .prepare_column_condition::<#entity>(
                            condition,
                            &#field_name,
                            &#column,
                        )
                        .unwrap();
                }
            }
        })
        .collect();

    let name = &input.ident;
    let input_name = format_ident!("{}Input", name);

    Ok(quote! {
        impl InputObject for #name {
           fn gql_input_type_ref(
                ctx: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::TypeRef {
                async_graphql::dynamic::TypeRef::named(stringify!(#input_name))
            }

            fn build_filter<'a>(context: &'static seaography::BuilderContext, ctx: async_graphql::dynamic::ResolverContext<'a>) -> sea_orm::Condition {
                let filters = ctx.args.get(&context.entity_query_field.filters);
                let filter_types_map_helper = seaography::FilterTypesMapHelper { context };
                let mut condition = sea_orm::Condition::all();

                if let Some(filters) = filters.map(|o| o.object().unwrap()) {
                    #(#filter_fields)*
                }

                condition
            }

            fn to_object(context: &'static seaography::BuilderContext) -> async_graphql::dynamic::InputObject {
                let input_name = stringify!(#input_name);
                let mut input = async_graphql::dynamic::InputObject::new(input_name)
                    .field(async_graphql::dynamic::InputValue::new(
                        "and",
                        async_graphql::dynamic::TypeRef::named_nn_list(input_name),
                    ))
                    .field(async_graphql::dynamic::InputValue::new(
                        "or",
                        async_graphql::dynamic::TypeRef::named_nn_list(input_name),
                    ))
                    .field(async_graphql::dynamic::InputValue::new(
                        "not",
                        async_graphql::dynamic::TypeRef::named(input_name),
                    ));

                {
                    let _filter_types_map_helper = seaography::FilterTypesMapHelper { context };
                    #(
                        input = #input_fields;
                    )*
                }



                input
            }
        }
    })
}

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(data_struct) => derive_filter_input_object_struct(&input, data_struct),
        _ => Err(Error::new(input.ident.span(), "Expected a struct")),
    }
}
