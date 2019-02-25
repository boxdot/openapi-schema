#[allow(unused_imports)]
#[macro_use]
extern crate openapi_schema_derive;
pub use openapi_schema_derive::*;

use openapi::v3_0::{ObjectOrReference, Schema, Spec};

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

impl<T> OpenapiSchema for Vec<T>
where
    T: OpenapiSchema,
{
    fn generate_schema(spec: &mut Spec) -> ObjectOrReference<Schema> {
        let reference = T::generate_schema(spec);
        let items_schema = match reference {
            ObjectOrReference::Object(schema) => schema,
            ObjectOrReference::Ref { ref_path } => Schema {
                ref_path: Some(ref_path),
                ..Schema::default()
            },
        };

        ObjectOrReference::Object(Schema {
            schema_type: Some("array".into()),
            items: Some(Box::new(items_schema)),
            ..Schema::default()
        })
    }
}
