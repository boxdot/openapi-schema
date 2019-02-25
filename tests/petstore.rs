use openapi::v3_0::{ObjectOrReference, Spec};
use openapi_schema::OpenapiSchema;

/// A pet for sale in the pet store
#[allow(dead_code)]
#[derive(OpenapiSchema)]
// #[serde(rename_all = "camelCase")]
pub struct Pet {
    id: Option<i64>,
    category: Option<Category>,
    name: String,
    photo_urls: Vec<String>,
    tags: Option<Vec<Tag>>,
    status: Option<Status>,
}

/// Pet Tag
///
/// A tag for a pet
#[allow(dead_code)]
#[derive(OpenapiSchema)]
struct Tag {
    id: i64,
    name: Option<String>,
}

/// A category for a pet
#[allow(dead_code)]
#[derive(OpenapiSchema)]
struct Category {
    id: Option<i64>,
    name: Option<String>,
}

/// Pet status in the store
#[allow(dead_code)]
#[derive(OpenapiSchema)]
enum Status {
    Available,
    Pending,
    Sold,
}

#[test]
fn test_plain_struct_derive() {
    let mut spec = Spec::default();
    Tag::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let tag = match schemas.get("Tag") {
        Some(ObjectOrReference::Object(ref tag)) => tag,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(tag.title, Some("Pet Tag".into()));
    assert_eq!(tag.description, Some("A tag for a pet".into()));

    let properties = tag.properties.as_ref().unwrap();

    assert!(properties.contains_key("id"));
    let id = properties.get("id").unwrap();
    assert_eq!(id.schema_type, Some("number".into()));
    assert_eq!(id.format, Some("int64".into()));

    assert!(properties.contains_key("name"));
    let name = properties.get("name").unwrap();
    assert_eq!(name.schema_type, Some("string".into()));

    assert_eq!(tag.required, Some(vec![String::from("id")]));

    Category::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let cat = match schemas.get("Category") {
        Some(ObjectOrReference::Object(ref cat)) => cat,
        _ => panic!("missing Category in schemas"),
    };

    assert_eq!(cat.title, None);
    assert_eq!(cat.description, Some("A category for a pet".into()));

    let properties = cat.properties.as_ref().unwrap();

    assert!(properties.contains_key("id"));
    let id = properties.get("id").unwrap();
    assert_eq!(id.schema_type, Some("number".into()));
    assert_eq!(id.format, Some("int64".into()));

    assert!(properties.contains_key("name"));
    let name = properties.get("name").unwrap();
    assert_eq!(name.schema_type, Some("string".into()));

    assert_eq!(cat.required, None);
}

#[test]
fn test_trivial_enum_derive() {
    let mut spec = Spec::default();
    Status::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let status = match schemas.get("Status") {
        Some(ObjectOrReference::Object(ref status)) => status,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(status.description, Some("Pet status in the store".into()));
    assert_eq!(status.schema_type, Some("string".into()));
    assert_eq!(
        status.enum_values,
        Some(vec!["Available".into(), "Pending".into(), "Sold".into(),])
    );
}

#[test]
fn test_nested_struct_derive() {
    let mut spec = Spec::default();
    Pet::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("Pet"));
    assert!(schemas.contains_key("Category"));
    assert!(schemas.contains_key("Tag"));
    assert!(schemas.contains_key("Status"));
}
