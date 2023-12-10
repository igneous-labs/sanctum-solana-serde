use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use serde::{de, ser::Error, Deserialize, Serialize};
use solana_sdk::transaction::VersionedTransaction;

use super::b64_buffer::B64Buffer;

#[derive(Clone, Default, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into)]
pub struct B64VersionedTx(pub VersionedTransaction);

impl<'de> Deserialize<'de> for B64VersionedTx {
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

impl Serialize for B64VersionedTx {
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
