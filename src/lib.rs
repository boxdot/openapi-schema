#[allow(unused_imports)]
#[macro_use]
extern crate openapi_schema_derive;
pub use openapi_schema_derive::*;

use openapi::v3_0::{ObjectOrReference, Schema, Spec};
use serde_json::{Number, Value};

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
            schema_type: Some("integer".into()),
            format: Some("int64".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for u64 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("integer".into()),
            format: Some("int64".into()),
            minimum: Some(Value::Number(Number::from(0))),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for usize {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("integer".into()),
            minimum: Some(Value::Number(Number::from(0))),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for isize {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("integer".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for i32 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("integer".into()),
            format: Some("int32".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for u32 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("integer".into()),
            format: Some("int32".into()),
            minimum: Some(Value::Number(Number::from(0))),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for u16 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("integer".into()),
            format: Some("int32".into()),
            minimum: Some(Value::Number(Number::from(0))),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for bool {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("boolean".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for f32 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("number".into()),
            format: Some("float".into()),
            ..Default::default()
        })
    }
}

impl OpenapiSchema for f64 {
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("number".into()),
            format: Some("float".into()),
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

impl<K, V> OpenapiSchema for std::collections::BTreeMap<K, V>
where
    V: OpenapiSchema,
{
    fn generate_schema(spec: &mut Spec) -> ObjectOrReference<Schema> {
        let values = V::generate_schema(spec);

        let items_schema = match values {
            ObjectOrReference::Object(schema) => schema,
            ObjectOrReference::Ref { ref_path } => Schema {
                ref_path: Some(ref_path),
                ..Schema::default()
            },
        };

        ObjectOrReference::Object(Schema {
            schema_type: Some("object".into()),
            additional_properties: Some(ObjectOrReference::Object(Box::new(items_schema))),
            ..Schema::default()
        })
    }
}

#[cfg(feature = "chrono")]
impl<T> OpenapiSchema for chrono::DateTime<T>
where
    T: chrono::TimeZone,
{
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("string".into()),
            format: Some("date-time".into()),
            ..Default::default()
        })
    }
}

#[cfg(feature = "chrono")]
impl<T> OpenapiSchema for chrono::Date<T>
where
    T: chrono::TimeZone,
{
    fn generate_schema(_spec: &mut Spec) -> ObjectOrReference<Schema> {
        ObjectOrReference::Object(Schema {
            schema_type: Some("string".into()),
            format: Some("date".into()),
            ..Default::default()
        })
    }
}
