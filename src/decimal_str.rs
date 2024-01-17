use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use rust_decimal::Decimal;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

/// rust decimal serialized as a string
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
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    From,
    Into,
)]
pub struct DecimalStr(pub Decimal);

struct DecimalStrVistor;

impl<'de> Visitor<'de> for DecimalStrVistor {
    type Value = DecimalStr;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("decimal serialized as a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let res = value
            .parse::<Decimal>()
            .map_err(|e| de::Error::custom(format!("Could not parse decimal. Error: {:?}", e)))?;
        Ok(DecimalStr(res))
    }
}

impl<'de> Deserialize<'de> for DecimalStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DecimalStrVistor)
    }
}

impl Serialize for DecimalStr {
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
    pub fn decimal_str_serde_round_trip() {
        let actual: Decimal = Decimal::from_f64_retain(std::f64::consts::PI).unwrap();

        let ser = serde_json::to_string(&DecimalStr(actual)).unwrap();
        assert_eq!(ser, format!("\"{}\"", actual));

        let de: DecimalStr = serde_json::from_str(&ser).unwrap();
        assert_eq!(*de, actual);
    }
}
