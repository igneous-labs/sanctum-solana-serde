use base64::{engine::general_purpose, Engine};
use derive_more::{AsMut, AsRef, Deref, DerefMut, IntoIterator};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

#[derive(Clone, AsRef, AsMut, Deref, DerefMut, IntoIterator)]
pub struct B64Buffer(pub Vec<u8>);

struct B64BufferVistor;

impl<'de> Visitor<'de> for B64BufferVistor {
    type Value = B64Buffer;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("base-64 encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = general_purpose::STANDARD
            .decode(value)
            .map_err(|e| de::Error::custom(format!("invalid base-64 string. Error: {:?}", e)))?;
        Ok(B64Buffer(bytes))
    }
}

impl<'de> Deserialize<'de> for B64Buffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(B64BufferVistor)
    }
}

impl Serialize for B64Buffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&general_purpose::STANDARD.encode(self.as_ref()))
    }
}

impl<'a> ToSchema<'a> for B64Buffer {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "B64Buffer",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("base-64 encoded byte buffer"))
                .into(),
        )
    }
}
