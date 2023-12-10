use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use serde::{de, ser::Error, Deserialize, Serialize};
use solana_sdk::transaction::Transaction;

use super::b64_buffer::B64Buffer;

/// base-64 encoded solana legacy transaction
#[derive(Clone, Default, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into)]
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

#[cfg(test)]
mod tests {
    use solana_program::{pubkey::Pubkey, system_instruction};

    pub use super::*;

    #[test]
    pub fn b64_legacy_tx_serde_round_trip() {
        let payer = Pubkey::new_unique();
        let actual = Transaction::new_with_payer(
            &[system_instruction::transfer(&payer, &payer, 69)],
            Some(&payer),
        );

        let ser = serde_json::to_string(&B64LegacyTx(actual.clone())).unwrap();
        assert!(ser.starts_with('"'));
        assert!(ser.ends_with('"'));

        let de: B64LegacyTx = serde_json::from_str(&ser).unwrap();
        assert_eq!(*de, actual);
    }
}
