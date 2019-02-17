use serde::Serialize;
use std::collections::BTreeMap;

mod de;

pub type Schemas = BTreeMap<String, openapi::v3_0::Schema>;
pub use de::Error;

pub fn to_schema<T: Serialize + Default>() -> Result<Schemas, Error> {
    let t = T::default();
    t.serialize(de::SchemaSerializer::default())
}
