use std::str::FromStr;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use solana_program::pubkey::Pubkey;

/// base-58 encoded solana pubkey string
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRef,
    AsMut,
    Deref,
    DerefMut,
    From,
    Into,
)]
pub struct B58Pubkey(pub Pubkey);

struct B58PubkeyVistor;

impl<'de> Visitor<'de> for B58PubkeyVistor {
    type Value = B58Pubkey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("base-58 encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let pk = Pubkey::from_str(value)
            .map_err(|e| de::Error::custom(format!("Pubkey from_str error: {:?}", e)))?;
        Ok(B58Pubkey(pk))
    }
}

impl<'de> Deserialize<'de> for B58Pubkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(B58PubkeyVistor)
    }
}

impl Serialize for B58Pubkey {
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
    pub fn b58_pubkey_serde_round_trip() {
        let actual = Pubkey::new_unique();

        let ser = serde_json::to_string(&B58Pubkey(actual)).unwrap();
        assert_eq!(ser, format!("\"{}\"", actual));

        let de: B58Pubkey = serde_json::from_str(&ser).unwrap();
        assert_eq!(*de, actual);
    }
}
