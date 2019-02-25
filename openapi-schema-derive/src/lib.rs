#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Type};

#[proc_macro_derive(OpenapiSchema)]
pub fn openapi_schema_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_openapi_schema(&input)
}

fn expand_derive_openapi_schema(input: &syn::DeriveInput) -> TokenStream {
    // use openapi::v3_0::{Spec, Schema, };

    let properties: Vec<proc_macro2::TokenStream> = match input.data {
        Data::Struct(ref s) => match s.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .map(|field| {
                    println!("{:#?}", field);
                    let field_name = &field.ident;
                    let ty = &field.ty;
                    let optional = is_optional(&field);
                    let gen = quote! {
                        (
                            stringify!(#field_name),
                            <#ty as OpenapiSchema>::generate_schema(spec),
                            #optional,
                        ),
                    };
                    gen.into()
                })
                .collect(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let name = &input.ident;
    let gen = quote! {
        impl OpenapiSchema for #name {
            fn generate_schema(spec: &mut openapi::v3_0::Spec) ->
                openapi::v3_0::ObjectOrReference<openapi::v3_0::Schema>
            {
                use openapi::v3_0::{ObjectOrReference, Schema, Components};

                let name = stringify!(#name);
                let ref_path = format!("#/components/schemas/{}", name);

                let already_generated = spec.components
                    .as_ref()
                    .and_then(|c| c.schemas.as_ref())
                    .map(|s| s.contains_key(name))
                    .unwrap_or(false);

                if !already_generated {
                    let mut properties = std::collections::BTreeMap::new();
                    let mut required = Vec::new();
                    for (name, prop, optional) in vec![#(#properties)*] {
                        let prop_schema = match prop {
                            ObjectOrReference::Object(schema) => schema,
                            ObjectOrReference::Ref{ ref_path } => Schema {
                                ref_path: Some(ref_path),
                                ..Schema::default()
                            }
                        };
                        properties.insert(String::from(name), prop_schema);
                        if !optional {
                            required.push(String::from(name));
                        }
                    }

                    let properties = if !properties.is_empty() {
                        Some(properties)
                    } else {
                        None
                    };

                    let required = if !required.is_empty() {
                        Some(required)
                    } else {
                        None
                    };

                    let schema = Schema {
                        properties,
                        required,
                        ..openapi::v3_0::Schema::default()
                    };

                    let components = spec.components.get_or_insert_with(Components::default);
                    let schemas = components.schemas
                        .get_or_insert_with(std::collections::BTreeMap::new);
                    schemas.insert(String::from(name), ObjectOrReference::Object(schema));
                }
                ObjectOrReference::Ref { ref_path }
            }
        }
    };
    gen.into()
}

fn is_optional(field: &Field) -> bool {
    match &field.ty {
        Type::Path(type_path) => {
            let option_ident = Ident::new("Option", Span::call_site());
            type_path.path.segments.len() == 1 && type_path.path.segments[0].ident == option_ident
        }
        _ => false,
    }
}
