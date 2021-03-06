use openapi::v3_0::{ObjectOrReference, Schema, Spec};
use openapi_schema::OpenapiSchema;
use serde::Serialize;

#[cfg(feature = "chrono")]
#[test]
fn test_datetime() {
    #[derive(OpenapiSchema)]
    #[allow(dead_code)]
    struct DatesContainer {
        date: chrono::Date<chrono::Utc>,
        date_time: chrono::DateTime<chrono::Utc>,
    }

    let mut spec = Spec::default();
    DatesContainer::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("DatesContainer"));

    let c = match schemas.get("DatesContainer") {
        Some(ObjectOrReference::Object(ref c)) => c,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(
        c.required,
        Some(vec![String::from("date"), String::from("date_time")])
    );

    let d = &c.properties.as_ref().unwrap().get("date").unwrap();
    assert_eq!(d.schema_type, Some("string".to_owned()));
    assert_eq!(d.format, Some("date".to_owned()));

    let dt = &c.properties.as_ref().unwrap().get("date_time").unwrap();
    assert_eq!(dt.schema_type, Some("string".to_owned()));
    assert_eq!(dt.format, Some("date-time".to_owned()));
}

#[test]
fn test_flatten() {
    #[derive(OpenapiSchema, Serialize)]
    struct A {
        primitive_field: u64,
    }

    #[derive(OpenapiSchema, Serialize)]
    struct B {
        inner: A,
    }

    #[derive(OpenapiSchema, Serialize)]
    #[allow(dead_code)]
    struct C {
        outer: u64,
        #[serde(flatten)]
        flatten: B,
    }

    let mut spec = Spec::default();
    C::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("A"));
    assert!(!schemas.contains_key("B"));

    let c = match schemas.get("C") {
        Some(ObjectOrReference::Object(ref user)) => user,
        _ => panic!("unexpected reference"),
    };

    let properties = c.properties.as_ref().unwrap();
    assert!(properties.contains_key("outer"));
    assert!(properties.contains_key("inner"));
    assert!(!properties.contains_key("flatten"));
    assert!(!properties.contains_key("primitive_field"));
}

#[test]
fn test_flatten_nested_module() {
    pub mod nested {
        use openapi_schema::OpenapiSchema;
        use serde::Serialize;
        #[derive(OpenapiSchema, Serialize)]
        pub struct A {
            primitive_field: u64,
        }
        #[derive(OpenapiSchema, Serialize)]
        pub struct B {
            pub inner: A,
        }
    }

    #[derive(OpenapiSchema, Serialize)]
    #[allow(dead_code)]
    struct C {
        outer: u64,
        #[serde(flatten)]
        flatten: nested::B,
    }

    let mut spec = Spec::default();
    C::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("A"));
    assert!(!schemas.contains_key("B"));

    let c = match schemas.get("C") {
        Some(ObjectOrReference::Object(ref user)) => user,
        _ => panic!("unexpected reference"),
    };

    let properties = c.properties.as_ref().unwrap();
    assert!(properties.contains_key("outer"));
    assert!(properties.contains_key("inner"));
    assert!(!properties.contains_key("flatten"));
    assert!(!properties.contains_key("primitive_field"));
}

#[test]
fn test_btreemap() {
    #[derive(OpenapiSchema, Serialize)]
    struct B {
        val: String,
    }

    #[derive(OpenapiSchema, Serialize)]
    struct A {
        map_with_raw_type: std::collections::BTreeMap<String, String>,
        map_with_complex_type: std::collections::BTreeMap<String, B>,
    }

    let mut spec = Spec::default();
    A::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("A"));
    assert!(schemas.contains_key("B"));

    let a = match schemas.get("A") {
        Some(ObjectOrReference::Object(ref user)) => user,
        _ => panic!("unexpected reference"),
    };

    let properties = a.properties.as_ref().unwrap();

    let ct = properties.get("map_with_complex_type").unwrap();
    assert_eq!(ct.schema_type, Some("object".to_owned()));
    assert_eq!(
        ct.additional_properties,
        Some(ObjectOrReference::Object(Box::new(Schema {
            ref_path: Some("#/components/schemas/B".to_owned()),
            ..Default::default()
        })))
    );
    let rt = properties.get("map_with_raw_type").unwrap();
    assert_eq!(rt.schema_type, Some("object".to_owned()));
    assert_eq!(
        rt.additional_properties,
        Some(ObjectOrReference::Object(Box::new(Schema {
            schema_type: Some("string".to_owned()),
            ..Default::default()
        })))
    );
}
