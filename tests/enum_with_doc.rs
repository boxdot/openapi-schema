use openapi::v3_0::{ObjectOrReference, Spec};
use openapi_schema::OpenapiSchema;

#[test]
fn test_enum_with_doc() {
    /// Pet status in the store
    #[derive(OpenapiSchema)]
    #[allow(dead_code)]
    pub enum Status {
        Available,
        Pending,
        /// Already sold
        Sold,
    }
    let mut spec = Spec::default();
    Status::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let status = match schemas.get("Status") {
        Some(ObjectOrReference::Object(ref tag)) => tag,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(
        status.description,
        Some(
            r#"Pet status in the store
* Sold: Already sold"#
                .to_owned()
        )
    );
    assert_eq!(status.schema_type, Some("string".to_owned()));
    assert_eq!(
        status.enum_values,
        Some(vec![
            "Available".to_owned(),
            "Pending".to_owned(),
            "Sold".to_owned()
        ])
    );
}

#[test]
fn test_enum_without_doc() {
    #[derive(OpenapiSchema)]
    #[allow(dead_code)]
    pub enum Status {
        /// some clever comment
        Available,
        Pending,
        /// Already sold
        Sold,
    }
    let mut spec = Spec::default();
    Status::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let status = match schemas.get("Status") {
        Some(ObjectOrReference::Object(ref tag)) => tag,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(
        status.description,
        Some(
            r#"
* Available: some clever comment
* Sold: Already sold"#
                .to_owned()
        )
    );
    assert_eq!(status.schema_type, Some("string".to_owned()));
    assert_eq!(
        status.enum_values,
        Some(vec![
            "Available".to_owned(),
            "Pending".to_owned(),
            "Sold".to_owned()
        ])
    );
}

#[test]
fn test_enum_fail() {
    #[derive(OpenapiSchema)]
    #[allow(dead_code)]
    pub enum Status {
        Available(usize),
        Pending,
        /// Already sold
        Sold,
    }
    let mut spec = Spec::default();
    Status::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());

    let schemas = spec.components.as_ref().unwrap().schemas.as_ref().unwrap();
    let status = match schemas.get("Status") {
        Some(ObjectOrReference::Object(ref tag)) => tag,
        _ => panic!("unexpected reference"),
    };

    assert_eq!(
        status.description,
        Some(
            r#"
* Sold: Already sold"#
                .to_owned()
        )
    );
    assert_eq!(status.schema_type, Some("string".to_owned()));
    assert_eq!(
        status.enum_values,
        Some(vec![
            "Available".to_owned(),
            "Pending".to_owned(),
            "Sold".to_owned()
        ])
    );
}
