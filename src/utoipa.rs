#![cfg(feature = "utoipa")]

use super::{B58Pubkey, B58Signature, B64Buffer, B64LegacyTx, B64VersionedTx, DecimalStr, U64Str};
use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

impl<'a> ToSchema<'a> for B58Pubkey {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "B58Pubkey",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("base-58 encoded pubkey"))
                .into(),
        )
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

impl<'a> ToSchema<'a> for B64Buffer {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "B64Buffer",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("base-64 encoded byte buffer"))
                .into(),
        )
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

impl<'a> ToSchema<'a> for B64VersionedTx {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
          "B64VersionedTx",
          ObjectBuilder::new()
              .schema_type(SchemaType::String)
              .description(Some("base-64 encoded versioned transaction. Encoded bytes can be created by calling `transaction.serialize()` in `@solana/web3.js`"))
              .into()
      )
    }
}

impl<'a> ToSchema<'a> for U64Str {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "U64Str",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("unsigned 64-bit integer serialized as a string"))
                .into(),
        )
    }
}

impl<'a> ToSchema<'a> for DecimalStr {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "DecimalStr",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("decimal serialized as a string"))
                .into(),
        )
    }
}
