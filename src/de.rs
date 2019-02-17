use crate::Schemas;

use openapi::v3_0::Schema;
use serde::ser::{self, Impossible, Serialize, SerializeStruct, SerializeTupleStruct, Serializer};

use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum Error {
    Message(String),
    ExpectedStruct,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::ExpectedStruct => "expected struct",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(std::error::Error::description(self))
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

#[derive(Debug, Default)]
pub struct SchemaSerializer(Schemas);

impl Serializer for SchemaSerializer {
    type Ok = Schemas;
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Struct;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Struct;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!(); // TODO
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!(); // TODO
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Struct::new(name, self.0))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::ExpectedStruct)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Struct::new(name, self.0))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::ExpectedStruct)
    }
}

pub struct Struct {
    name: &'static str,
    schemas: Schemas,
    new_schema: Option<Schema>,
}

impl Struct {
    fn new(name: &'static str, schemas: Schemas) -> Self {
        // check if we already constructed a schema for this type
        let new_schema = if !schemas.contains_key(name) {
            Some(Schema::default())
        } else {
            None
        };

        Self {
            name,
            schemas,
            new_schema,
        }
    }
}

impl SerializeStruct for Struct {
    type Ok = Schemas;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if let Some(new_schema) = self.new_schema {
            self.schemas.insert(self.name.into(), new_schema);
        }
        Ok(self.schemas)
    }
}

impl SerializeTupleStruct for Struct {
    type Ok = Schemas;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if let Some(new_schema) = self.new_schema {
            self.schemas.insert(self.name.into(), new_schema);
        }
        Ok(self.schemas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    /// A tag for a pet
    #[derive(Debug, Clone, Serialize, Default)]
    struct Tag {
        id: Option<i64>,
        name: Option<String>,
    }

    #[test]
    fn test_flat_struct_schema() {
        let value = Tag::default();
        let schemas = value.serialize(SchemaSerializer::default()).unwrap();
        assert_eq!(schemas.len(), 1);
        assert!(schemas.contains_key("Tag"));
    }
}
