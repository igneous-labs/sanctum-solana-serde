#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "utoipa")]
#[cfg_attr(docsrs, doc(cfg(feature = "utoipa")))]
pub mod utoipa_schema;

#[cfg(feature = "b58_pubkey")]
#[cfg_attr(docsrs, doc(cfg(feature = "b58_pubkey")))]
pub mod b58_pubkey;

#[cfg(feature = "b58_signature")]
#[cfg_attr(docsrs, doc(cfg(feature = "b58_signature")))]
pub mod b58_signature;

#[cfg(feature = "b64_buffer")]
#[cfg_attr(docsrs, doc(cfg(feature = "b64_buffer")))]
pub mod b64_buffer;

#[cfg(feature = "b64_legacy_tx")]
#[cfg_attr(docsrs, doc(cfg(feature = "b64_legacy_tx")))]
pub mod b64_legacy_tx;

#[cfg(feature = "b64_versioned_tx")]
#[cfg_attr(docsrs, doc(cfg(feature = "b64_versioned_tx")))]
pub mod b64_versioned_tx;

#[cfg(feature = "decimal_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "decimal_str")))]
pub mod decimal_str;

#[cfg(feature = "u64_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "u64_str")))]
pub mod u64_str;
