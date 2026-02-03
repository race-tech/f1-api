use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Error, Fields};

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let orig_ident = &input.ident;
    let name = quote! { stringify!(#orig_ident) };

    match &input.data {
        syn::Data::Struct(data_struct) => derive_output_type_struct(&input, data_struct, name),
        _ => Err(Error::new(input.ident.span(), "Expected a struct")),
    }
}

fn derive_output_type_struct(
    input: &DeriveInput,
    data_struct: &DataStruct,
    name: TokenStream,
) -> syn::Result<TokenStream> {
    let orig_ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Fields::Named(named) = &data_struct.fields else {
        return Err(Error::new(input.ident.span(), "Expected named fields"));
    };

    let mut fields: Vec<TokenStream> = Vec::new();

    for field in named.named.iter() {
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        fields.push(quote! {
            .field(async_graphql::dynamic::Field::new(
                stringify!(#field_ident),
                <#field_ty>::gql_output_type_ref(context),
                move |ctx| {
                    async_graphql::dynamic::FieldFuture::new(async move {
                        let obj = seaography::try_downcast_ref::<#orig_ident #ty_generics>(ctx.parent_value)?;
                        Ok(<#field_ty>::gql_field_value(
                            obj.#field_ident.clone(), context
                        ))
                    })
                }))
        });
    }

    let object_def: TokenStream = quote! {
        async_graphql::dynamic::Object::new(#name)
        #(#fields)*
    };
    let imports = output_type_imports();

    Ok(quote! {
        unsafe impl #impl_generics Send for #orig_ident #ty_generics #where_clause {}
        unsafe impl #impl_generics Sync for #orig_ident #ty_generics #where_clause {}

        impl #impl_generics Object for #orig_ident #ty_generics #where_clause {
            fn type_name() -> &'static str {
                stringify!(#name)
            }
        }

        impl #impl_generics seaography::CustomOutputType for #orig_ident #ty_generics #where_clause {
            fn gql_output_type_ref(ctx: &'static seaography::BuilderContext) -> async_graphql::dynamic::TypeRef {
                async_graphql::dynamic::TypeRef::named_nn(#name)
            }

            fn gql_field_value(self, ctx: &'static seaography::BuilderContext) -> Option<async_graphql::dynamic::FieldValue<'static>> {
                Some(async_graphql::dynamic::FieldValue::owned_any(self))
            }
        }

        impl #impl_generics seaography::CustomOutputObject for #orig_ident #ty_generics #where_clause {
            fn basic_object(
                context: &'static seaography::BuilderContext,
            ) -> async_graphql::dynamic::Object {
                #imports

                #object_def
            }
        }
    })
}

fn output_type_imports() -> TokenStream {
    quote! {
        use seaography::{CustomOutputType, GqlScalarValueType, GqlModelType, GqlModelHolderType};
    }
}
