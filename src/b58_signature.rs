use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use solana_sdk::{bs58, signature::Signature};

/// base-58 encoded solana signature string
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
pub struct B58Signature(pub Signature);

struct B58SignatureVistor;

impl<'de> Visitor<'de> for B58SignatureVistor {
    type Value = B58Signature;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("base-58 encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = bs58::decode(value)
            .into_vec()
            .map_err(|e| de::Error::custom(format!("invalid base-58 string. Error: {:?}", e)))?;
        let bytes_arr = <[u8; 64]>::try_from(<&[u8]>::clone(&&bytes[..]))
            .map_err(|e| de::Error::custom(format!("Not 512-bit long. Error: {:?}", e)))?;
        Ok(B58Signature(Signature::try_from(bytes_arr).unwrap()))
    }
}

impl<'de> Deserialize<'de> for B58Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(B58SignatureVistor)
    }
}

impl Serialize for B58Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&bs58::encode(self.0).into_string())
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    pub fn b58_signature_serde_round_trip() {
        let actual = Signature::new_unique();

        let ser = serde_json::to_string(&B58Signature(actual)).unwrap();
        assert_eq!(ser, format!("\"{}\"", actual));

        let de: B58Signature = serde_json::from_str(&ser).unwrap();
        assert_eq!(*de, actual);
    }
}
