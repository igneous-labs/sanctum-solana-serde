use utoipa::{
    openapi::{ObjectBuilder, RefOr, Schema, SchemaType},
    ToSchema,
};

#[cfg(feature = "b58_pubkey")]
impl<'a> ToSchema<'a> for crate::b58_pubkey::B58Pubkey {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "B58Pubkey",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("base-58 encoded solana pubkey"))
                .into(),
        )
    }
}

#[cfg(feature = "b58_signature")]
impl<'a> ToSchema<'a> for crate::b58_signature::B58Signature {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "B58Signature",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("base-58 encoded solana signature"))
                .into(),
        )
    }
}

#[cfg(feature = "b64_buffer")]
impl<'a> ToSchema<'a> for crate::b64_buffer::B64Buffer {
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

#[cfg(feature = "b64_legacy_tx")]
impl<'a> ToSchema<'a> for crate::b64_legacy_tx::B64LegacyTx {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
          "B64LegacyTx",
          ObjectBuilder::new()
              .schema_type(SchemaType::String)
              .description(Some("base-64 encoded solana legacy transaction. Can be created by calling `transaction.serialize()` in `@solana/web3.js`"))
              .into()
      )
    }
}

#[cfg(feature = "b64_versioned_tx")]
impl<'a> ToSchema<'a> for crate::b64_versioned_tx::B64VersionedTx {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
          "B64VersionedTx",
          ObjectBuilder::new()
              .schema_type(SchemaType::String)
              .description(Some("base-64 encoded solana versioned transaction. Encoded bytes can be created by calling `transaction.serialize()` in `@solana/web3.js`"))
              .into()
      )
    }
}

#[cfg(feature = "u64_str")]
impl<'a> ToSchema<'a> for crate::u64_str::U64Str {
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

#[cfg(feature = "decimal_str")]
impl<'a> ToSchema<'a> for crate::decimal_str::DecimalStr {
    fn schema() -> (&'a str, RefOr<Schema>) {
        (
            "DecimalStr",
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("rust decimal serialized as a string"))
                .into(),
        )
    }
}
