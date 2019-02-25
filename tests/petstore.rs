use openapi::v3_0::{ObjectOrReference, Spec};
use openapi_schema::OpenapiSchema;

/// A tag for a pet
#[allow(dead_code)]
#[derive(OpenapiSchema)]
struct Tag {
    id: i64,
    name: Option<String>,
}

#[test]
fn test_simple_derive() {
    let mut spec = Spec::default();
    Tag::generate_schema(&mut spec);

    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("Tag"));
    let tag = schemas.get("Tag").unwrap();
    let tag = match tag {
        ObjectOrReference::Object(ref tag) => tag,
        _ => panic!("unexpected reference"),
    };

    let properties = tag.properties.as_ref().unwrap();

    assert!(properties.contains_key("id"));
    let id = properties.get("id").unwrap();
    assert_eq!(id.schema_type, Some("number".into()));
    assert_eq!(id.format, Some("int64".into()));

    assert!(properties.contains_key("name"));
    let name = properties.get("name").unwrap();
    assert_eq!(name.schema_type, Some("string".into()));

    assert_eq!(tag.required, Some(vec![String::from("id")]));
}
