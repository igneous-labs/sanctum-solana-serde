use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{de, ser::Error, Deserialize, Serialize};
use solana_sdk::transaction::Transaction;
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

use super::B64Buffer;

#[derive(Clone, AsRef, AsMut, Deref, DerefMut)]
pub struct B64LegacyTx(pub Transaction);

impl<'de> Deserialize<'de> for B64LegacyTx {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buf = B64Buffer::deserialize(deserializer)?;
        let tx = bincode::deserialize(&buf).map_err(|e| {
            de::Error::custom(format!("Could not bincode deserialize. Error: {:?}", e))
        })?;
        Ok(Self(tx))
    }
}

impl Serialize for B64LegacyTx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let buf = bincode::serialize(self.as_ref()).map_err(|e| {
            S::Error::custom(format!("Could not bincode serialize. Error: {:?}", e))
        })?;
        B64Buffer::serialize(&B64Buffer(buf), serializer)
    }
}

impl<'a> ToSchema<'a> for B64LegacyTx {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "B64LegacyTx",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("base-64 encoded legacy transaction. Can be created by calling `transaction.serialize()` in `@solana/web3.js`"))
                .into()
        )
    }
}
