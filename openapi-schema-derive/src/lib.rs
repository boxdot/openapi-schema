#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, AttrStyle, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Type,
};

#[proc_macro_derive(OpenapiSchema)]
pub fn openapi_schema_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_openapi_schema(&input)
}

fn expand_derive_openapi_schema(input: &syn::DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(_) => derive_for_struct(input),
        Data::Enum(_) => derive_for_enum(input),
        Data::Union(_) => panic!("unsupported OpenapiSchema derive for union type"),
    }
}

fn derive_for_struct(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let (title, desc) = title_and_desc(&input.attrs);
    let properties = collect_struct_properties(&input.data);

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

                    // buffer for collecting flattened schemas
                    let flatten_spec = &mut openapi::v3_0::Spec::default();

                    for (name, prop, doc, optional, flatten) in vec![#(#properties)*] {
                        if flatten {
                            // get schema from flattened schemas
                            let mut flatten_schemas = flatten_spec.components
                                .as_mut()
                                .and_then(|c| c.schemas.as_mut())
                                .expect("logic error: missing flatten schemas");

                            let prop_schema = flatten_schemas
                                .remove(name)
                                .unwrap_or_else(|| panic!("logic error, missing: {}", name));
                            let prop_schema = match prop_schema {
                                ObjectOrReference::Object(schema) => schema,
                                _ => panic!("unexpected reference"),
                            };

                            let inner_properties = prop_schema.properties
                                .unwrap_or_else(Default::default);
                            for (inner_name, inner_prop_schema) in inner_properties
                            {
                                properties.insert(inner_name.clone(), inner_prop_schema);
                                if prop_schema.required
                                    .as_ref()
                                    .map(|r| r.contains(&inner_name))
                                    .unwrap_or(false)
                                {
                                    required.push(inner_name);
                                }
                            }
                        } else {
                            // create new schema
                            let prop_schema = match prop {
                                ObjectOrReference::Object(mut schema) => {
                                    if !doc.is_empty() {
                                        schema.description = Some(doc.into());
                                    }
                                    schema
                                },
                                ObjectOrReference::Ref { ref_path } => Schema {
                                    ref_path: Some(ref_path),
                                    ..Schema::default()
                                }
                            };

                            properties.insert(String::from(name), prop_schema);
                            if !optional {
                                required.push(String::from(name));
                            }
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
                        title: #title,
                        description: #desc,
                        properties,
                        required,
                        ..Default::default()
                    };

                    let components = spec.components.get_or_insert_with(Default::default);
                    let schemas = components.schemas.get_or_insert_with(Default::default);
                    schemas.insert(String::from(name), ObjectOrReference::Object(schema));

                    let flatten_schemas = flatten_spec.components
                        .as_mut()
                        .and_then(|c| c.schemas.take());
                    if let Some(mut flatten_schemas) = flatten_schemas {
                        schemas.extend(flatten_schemas);
                    }
                }
                ObjectOrReference::Ref { ref_path }
            }
        }
    };
    gen.into()
}

fn collect_struct_properties(data: &Data) -> Vec<proc_macro2::TokenStream> {
    match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => fields
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;

                let type_name = match field.ty {
                    Type::Path(ref field) => {
                        &field
                            .path
                            .segments
                            .last()
                            .expect("invalid ty path")
                            .value()
                            .ident
                    }
                    _ => panic!("not supported type for field: {:?}", field_name),
                };

                let ty = &field.ty;
                let doc = doc_string(&field.attrs);
                let optional = is_optional(&field);
                let flatten = has_serde_flatten(field);
                if flatten {
                    quote! {
                        (
                            stringify!(#type_name),
                            <#ty as OpenapiSchema>::generate_schema(flatten_spec),
                            #doc,
                            #optional,
                            #flatten,
                        ),
                    }
                } else {
                    quote! {
                        (
                            stringify!(#field_name),
                            <#ty as OpenapiSchema>::generate_schema(spec),
                            #doc,
                            #optional,
                            #flatten,
                        ),
                    }
                }
            })
            .collect(),
        _ => panic!("logic error"),
    }
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

fn has_serde_flatten(field: &Field) -> bool {
    field.attrs.iter().any(|attr| {
        if attr.style == AttrStyle::Outer && attr.path.is_ident("serde") {
            let meta = attr.interpret_meta();
            let ls = match meta {
                Some(Meta::List(MetaList { nested, .. })) => nested,
                _ => return false,
            };
            ls.into_iter().any(|item| match item {
                NestedMeta::Meta(Meta::Word(ref ident)) => ident == "flatten",
                _ => false,
            })
        } else {
            false
        }
    })
}

/// Returns the summary of the doc (first paragraph) and the optional body (other paragraphs).
fn title_and_desc(attrs: &[Attribute]) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let doc = doc_string(attrs);
    let mut split = doc.splitn(2, "\n\n");
    match (split.next(), split.next()) {
        (Some(title), Some(desc)) => (quote!(Some(#title.into())), quote!(Some(#desc.into()))),
        (Some(desc), None) => (quote!(None), quote!(Some(#desc.into()))),
        (None, None) => (quote!(None), quote!(None)),
        (None, _) => unreachable!(),
    }
}

fn doc_string(attrs: &[Attribute]) -> String {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.style == AttrStyle::Outer && attr.path.is_ident("doc") {
                let meta = attr.interpret_meta();
                match meta {
                    Some(Meta::NameValue(MetaNameValue {
                        lit: Lit::Str(ref s),
                        ..
                    })) => Some(s.value().trim().into()),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect();
    lines.join("\n")
}

fn derive_for_enum(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let (title, desc) = title_and_desc(&input.attrs);

    let enum_values: Vec<_> = match input.data {
        Data::Enum(DataEnum { ref variants, .. }) => variants
            .iter()
            .map(|var| {
                if !var.attrs.is_empty() {
                    panic!("cannot derive OpenapiSchema for non-trivial enums");
                }
                let ident = &var.ident;
                quote! {
                    String::from(stringify!(#ident)),
                }
            })
            .collect(),
        _ => panic!("logic error"),
    };

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
                    let schema = Schema {
                        title: #title,
                        description: #desc,
                        schema_type: Some("string".into()),
                        enum_values: Some(vec![#(#enum_values)*]),
                        ..Default::default()
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
