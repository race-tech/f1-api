use proc_macro2::TokenStream;
use quote::quote;
use syn::{parenthesized, parse::Parse, punctuated::Punctuated, Data, DeriveInput, Ident, Token};

pub fn expand(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = collect_fields(&ast.data);

    let expanded_unique = expand_unique_fields(&fields);
    let expanded_compatible = expand_compatible_fields(&fields);

    let expanded = quote! {
        impl FilterValidation for #name {
            fn validate(&self) -> Result<(), crate::error::Error> {
                #expanded_unique
                #expanded_compatible

                Ok(())
            }
        }
    };

    expanded
}

fn expand_unique_fields(fields: &FieldCollection) -> TokenStream {
    fields.unique.iter().fold(quote! {}, |acc, f| {
        let incompatible = fields.unskiped.iter().filter(|o| o != &f);
        let error_msg = format!("field {} should be unique", f);
        let error = quote! {
            crate::error::Error {
                kind: crate::error::ErrorKind::InvalidParameter,
                message: Some(#error_msg.to_string()),
            }
        };

        quote! {
            #acc

            if self.#f.is_some() {
                #(
                    if self.#incompatible.is_some() {
                        return Err(#error);
                    }
                )*
            }
        }
    })
}

fn expand_compatible_fields(fields: &FieldCollection) -> TokenStream {
    fields
        .compatible
        .iter()
        .fold(quote! {}, |acc, (f, others)| {
            let incompatible = fields
                .unskiped
                .iter()
                .filter(|o| o != &f && !others.contains(o));

            let error_msg = format!(
                "field {} is only compatible with fields {}",
                f,
                others
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let error = quote! {
                crate::error::Error {
                    kind: crate::error::ErrorKind::InvalidParameter,
                    message: Some(#error_msg.to_string()),
                }
            };

            quote! {
                #acc

                if self.#f.is_some() {
                    #(
                        if self.#incompatible.is_some() {
                            return Err(#error);
                        }
                    )*
                }
            }
        })
}

fn collect_fields(data: &Data) -> FieldCollection {
    let data = match data {
        Data::Struct(s) => s,
        _ => panic!("FilterValidation only works on structs"),
    };

    data.fields
        .iter()
        .map(|f| {
            FieldValidation::from_attrs(
                f.ident
                    .clone()
                    .expect("Expected field to have an identifier"),
                &f.attrs,
            )
        })
        .fold(FieldCollection::default(), |mut acc, f| {
            match f {
                FieldValidation::Skip(_) => (),
                FieldValidation::Unique(ident) => {
                    acc.unique.push(ident.clone());
                    acc.unskiped.push(ident);
                }
                FieldValidation::Compatible(ident, fields) => {
                    acc.unique.push(ident.clone());
                    acc.compatible.push((ident, fields));
                }
                FieldValidation::All(ident) => {
                    acc.unskiped.push(ident.clone());
                    acc.compatible_all.push(ident);
                }
            }

            acc
        })
}

#[derive(Default)]
struct FieldCollection {
    unskiped: Vec<Ident>,
    unique: Vec<Ident>,
    compatible: Vec<(Ident, Vec<Ident>)>,
    compatible_all: Vec<Ident>,
}

enum FieldValidation {
    Skip(Ident),
    Unique(Ident),
    Compatible(Ident, Vec<Ident>),
    All(Ident),
}

impl FieldValidation {
    fn from_attrs(ident: Ident, attrs: &[syn::Attribute]) -> Self {
        let mut skip = false;
        let mut unique = false;
        let mut compatible = None;

        attrs.iter().for_each(|a| {
            if a.path().is_ident("validation") {
                let _ = a.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        skip = true;
                    } else if meta.path.is_ident("unique") {
                        unique = true;
                    } else if meta.path.is_ident("compatible") {
                        let content;
                        parenthesized!(content in meta.input);
                        let punc: Punctuated<Ident, Token![,]> =
                            content.parse_terminated(Ident::parse, Token![,])?;
                        compatible = Some(punc);
                    }

                    Ok(())
                });
            }
        });

        if skip {
            Self::Skip(ident)
        } else if unique {
            Self::Unique(ident)
        } else if let Some(compatible) = compatible {
            Self::Compatible(ident, compatible.into_iter().collect::<Vec<_>>())
        } else {
            Self::All(ident)
        }
    }
}
