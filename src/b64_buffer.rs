use data_encoding::BASE64;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

/// base-64 encoded byte buffer
#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    From,
    Into,
    IntoIterator,
)]
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
        let bytes = BASE64
            .decode(value.as_bytes())
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
        serializer.serialize_str(&BASE64.encode(self.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    pub fn b64_buffer_serde_round_trip() {
        let actual: Vec<u8> = vec![0, 1, 2, 3, 4, 5];

        let ser = serde_json::to_string(&B64Buffer(actual.clone())).unwrap();
        assert_eq!(ser, format!("\"{}\"", BASE64.encode(&actual)));

        let de: B64Buffer = serde_json::from_str(&ser).unwrap();
        assert_eq!(*de, actual);
    }
}
