#[allow(unused_imports)]
#[macro_use]
extern crate openapi_schema_derive;
pub use openapi_schema_derive::*;

use openapi::v3_0::{ObjectOrReference, Schema, Spec};
use serde::Serialize;

use std::collections::BTreeMap;

mod de;

pub type Schemas = BTreeMap<String, openapi::v3_0::Schema>;
pub use de::Error;

pub trait OpenapiSchema {
    fn generate_schema(spec: &mut Spec) -> ObjectOrReference<Schema>;
}

impl OpenapiSchema for String {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("string".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for i64 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("number".into()),
            format: Some("int64".into()),
            ..Default::default()
        })
    }
}

impl<T> OpenapiSchema for Option<T>
where
    T: OpenapiSchema,
{
    fn generate_schema(spec: &mut Spec) -> ObjectOrReference<Schema> {
        T::generate_schema(spec)
    }
}

pub fn to_schema<T: Serialize + Default>() -> Result<Schemas, Error> {
    let t = T::default();
    t.serialize(de::SchemaSerializer::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        let mut spec = Spec::default();
        type T = Option<String>;
        T::generate_schema(&mut spec);
    }
}

// /// A tag for a pet
// #[derive(Debug, Clone)]
// struct Tag {
//     id: Option<i64>,
//     name: Option<String>,
// }

// impl OpenapiSchema for Tag {
//     fn generate_schema(
//         spec: &mut openapi::v3_0::Spec,
//     ) -> openapi::v3_0::ObjectOrReference<openapi::v3_0::Schema> {
//         let ref_path = format!("#/components/schemas/{}", stringify!(Tag));

//         let components = spec
//             .components
//             .get_or_insert_with(openapi::v3_0::Components::default);
//         let schemas = components
//             .schemas
//             .get_or_insert_with(std::collections::BTreeMap::new);

//         if !schemas.contains_key(stringify!(#name)) {
//             let schema = openapi::v3_0::Schema::default();
//             schemas.insert(
//                 "Tag".into(),
//                 openapi::v3_0::ObjectOrReference::Object(schema),
//             );
//         }
//         openapi::v3_0::ObjectOrReference::Ref { ref_path }
//     }
// }

// #[test]
// fn test_generate_schema() {
//     let mut spec = Spec::default();
//     Tag::generate_schema(&mut spec);
//     println!("{:#?}", spec);
//     assert!(false);
// }
