# openapi-schema [![build Status]][travis]

[OpenAPI 3.0 Schema] generation library for Rust types.

The implementation is *experimental*. For now, we support only features needed to implement a simple
spec generation from an `actix-web` application (cf. https://github.com/actix/actix-web/issues/310).
If you are interested in the library and want to use it in your project, feel free to extend the
supported types.

## Example

```rust
use openapi_schema::OpenapiSchema;

/// A tag for a pet
#[derive(OpenapiSchema)]
pub struct Tag {
    pub id: u64,
    pub name: Option<String>,
}

fn main() {
    let mut spec = openapi::v3_0::Spec::default();
    Tag::generate_schema(&mut spec);
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());
}
```

The above example generates the following schema:

```json
{
  "openapi": "",
  "info": {
    "title": "",
    "version": ""
  },
  "paths": {},
  "components": {
    "schemas": {
      "Tag": {
        "description": "A tag for a pet",
        "required": [
          "id"
        ],
        "properties": {
          "id": {
            "type": "number",
            "format": "int64",
            "minimum": 0
          },
          "name": {
            "type": "string"
          }
        }
      }
    }
  }
}
```

## Features

* [x] Primitive types `i64`, `u64`, `i32`, `u32`, `bool`, `String`
* [x] `Option<T>`
* [x] `Vec<T>`
* [x] Simple Rust structs (no tuple and unit structs)
* [x] C-like Rust enums (no non-trivial variants)
* [x] Doc comments are used as `title` and `description` of the schema.
* [x] Doc comments of attributes are used as `description` of the property.
* [x] Support for `serde(flatten)`.

TODO

* [ ] Support for `serde(rename)`.
* [ ] Support for `serde(default)`.


## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[OpenAPI 3.0 Schema]: https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md#schemaObject
[build Status]: https://travis-ci.com/boxdot/openapi-schema.svg?branch=master
[travis]: https://travis-ci.com/boxdot/openapi-schema
