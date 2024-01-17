use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use serde::{de, ser::Error, Deserialize, Serialize};
use solana_sdk::transaction::VersionedTransaction;

use super::b64_buffer::B64Buffer;

/// base-64 encoded solana versioned transaction
#[derive(Clone, Debug, Default, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into)]
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

#[cfg(test)]
mod tests {
    use data_encoding::BASE64;
    use solana_program::{
        hash::Hash,
        message::{self, VersionedMessage},
        pubkey::Pubkey,
        system_instruction,
    };

    pub use super::*;

    #[test]
    pub fn b64_versioned_tx_serde_round_trip() {
        let payer = Pubkey::new_unique();
        let actual = VersionedTransaction {
            message: VersionedMessage::V0(
                message::v0::Message::try_compile(
                    &payer,
                    &[system_instruction::transfer(&payer, &payer, 69)],
                    &[],
                    Hash::default(),
                )
                .unwrap(),
            ),
            signatures: vec![],
        };

        let ser = serde_json::to_string(&B64VersionedTx(actual.clone())).unwrap();
        assert!(ser.starts_with('"'));
        assert!(ser.ends_with('"'));
        // ensure valid base64
        BASE64.decode(ser[1..ser.len() - 1].as_bytes()).unwrap();

        let de: B64VersionedTx = serde_json::from_str(&ser).unwrap();
        assert_eq!(*de, actual);
    }
}
