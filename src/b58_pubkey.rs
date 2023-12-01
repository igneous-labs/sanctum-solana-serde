use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use solana_program::pubkey::Pubkey;

#[derive(Default, Clone, Copy, AsRef, AsMut, Deref, DerefMut)]
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
        let bytes = bs58::decode(value)
            .into_vec()
            .map_err(|e| de::Error::custom(format!("invalid base-58 string. Error: {:?}", e)))?;
        let bytes_arr = <[u8; 32]>::try_from(<&[u8]>::clone(&&bytes[..]))
            .map_err(|e| de::Error::custom(format!("Not 256-bit long. Error: {:?}", e)))?;
        Ok(B58Pubkey(Pubkey::new_from_array(bytes_arr)))
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
        serializer.serialize_str(&bs58::encode(self.as_ref()).into_string())
    }
}
