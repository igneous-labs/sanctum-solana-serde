use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

/// unsigned 64-bit integer serialized as a string
#[derive(
    Clone,
    Copy,
    Default,
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
)]
pub struct U64Str(pub u64);

struct U64StrVistor;

impl<'de> Visitor<'de> for U64StrVistor {
    type Value = U64Str;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("unsigned 64-bit integer serialized as a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let res = value
            .parse::<u64>()
            .map_err(|e| de::Error::custom(format!("Could not parse u64. Error: {:?}", e)))?;
        Ok(U64Str(res))
    }
}

impl<'de> Deserialize<'de> for U64Str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(U64StrVistor)
    }
}

impl Serialize for U64Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    pub fn u64_str_serde_roundtrip() {
        let actual = 2134324u64;

        let ser = serde_json::to_string(&U64Str(actual)).unwrap();
        assert_eq!(ser, format!("\"{}\"", actual));

        let de: U64Str = serde_json::from_str(&ser).unwrap();
        assert_eq!(*de, actual);
    }
}
