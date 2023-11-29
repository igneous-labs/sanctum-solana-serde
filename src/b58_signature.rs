use derive_more::{AsRef, Deref};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use solana_sdk::{bs58, signature::Signature};
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

#[derive(Clone, Copy, Debug, AsRef, Deref)]
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

impl<'a> ToSchema<'a> for B58Signature {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "B58Pubkey",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("base-58 encoded signature"))
                .into(),
        )
    }
}
