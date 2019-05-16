use openapi::v3_0::{ObjectOrReference, Spec};
use openapi_schema::OpenapiSchema;

/// A pet for sale in the pet store
#[derive(OpenapiSchema)]
#[allow(dead_code)]
// #[serde(rename_all = "camelCase")]
pub struct Pet {
    id: Option<i64>,
    category: Option<Category>,
    name: String,
    photo_urls: Vec<String>,
    tags: Option<Vec<Tag>>,
    status: Option<Status>,
}

/// Pet status in the store
#[derive(OpenapiSchema)]
#[allow(dead_code)]
pub enum Status {
    Available,
    Pending,
    Sold,
}

/// A category for a pet
#[derive(OpenapiSchema)]
#[allow(dead_code)]
struct Category {
    id: Option<i64>,
    name: Option<String>,
}

/// A tag for a pet
#[derive(OpenapiSchema)]
#[allow(dead_code)]
struct Tag {
    id: Option<i64>,
    name: Option<String>,
}

/// An uploaded response
///
/// Describes the result of uploading an image resource
#[derive(OpenapiSchema)]
#[allow(dead_code)]
pub struct ApiResponse {
    code: Option<i32>,
    _type: Option<String>,
    message: Option<String>,
}

/// Pet Order
///
/// An order for a pets from the pet store
#[derive(OpenapiSchema)]
#[allow(dead_code)]
// #[serde(rename_all = "camelCase")]
pub struct Order {
    id: Option<i64>,
    pet_id: Option<i64>,
    quantity: Option<i32>,
    ship_date: Option<String>,
    status: Option<OrderStatus>,
    // #[serde(default)]
    complete: Option<bool>,
}

#[derive(OpenapiSchema)]
#[allow(dead_code)]
enum OrderStatus {
    Placed,
    Approved,
    Delivered,
}

/// a User
///
/// A User who is purchasing from the pet store
#[derive(OpenapiSchema)]
#[allow(dead_code)]
// #[serde(rename_all = "camelCase")]
pub struct User {
    id: Option<i64>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    phone: Option<String>,
    /// User Status
    user_status: Option<i32>,
}

/// List of user object
pub type UserArray = Vec<User>;

#[test]
fn test_tag_derive() {
    let mut spec = Spec::default();
    Tag::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let tag = match schemas.get("Tag") {
        Some(ObjectOrReference::Object(ref tag)) => tag,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(tag.title, None);
    assert_eq!(tag.description, Some("A tag for a pet".into()));

    let properties = tag.properties.as_ref().unwrap();

    assert!(properties.contains_key("id"));
    let id = properties.get("id").unwrap();
    assert_eq!(id.schema_type, Some("integer".into()));
    assert_eq!(id.format, Some("int64".into()));

    assert!(properties.contains_key("name"));
    let name = properties.get("name").unwrap();
    assert_eq!(name.schema_type, Some("string".into()));

    assert_eq!(tag.required, None);
}

#[test]
fn test_status_derive() {
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
fn test_pet_derive() {
    let mut spec = Spec::default();
    Pet::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("Pet"));
    assert!(schemas.contains_key("Category"));
    assert!(schemas.contains_key("Tag"));
    assert!(schemas.contains_key("Status"));

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let pet = match schemas.get("Pet") {
        Some(ObjectOrReference::Object(ref pet)) => pet,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(
        pet.required,
        Some(vec![String::from("name"), String::from("photo_urls")])
    );
}

#[test]
fn test_attr_doc() {
    let mut spec = Spec::default();
    User::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    assert!(schemas.contains_key("User"));

    let user = match schemas.get("User") {
        Some(ObjectOrReference::Object(ref user)) => user,
        _ => panic!("unexpected reference"),
    };

    let properties = user.properties.as_ref().unwrap();
    let property = properties.get("user_status").unwrap();
    assert_eq!(property.description, Some(String::from("User Status")));
}
