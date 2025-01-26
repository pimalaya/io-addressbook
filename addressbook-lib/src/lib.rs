#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// #![doc = include_str!("../../README.md")]

#[cfg(feature = "carddav")]
pub mod carddav;
mod types;
#[cfg(feature = "vdir")]
pub mod vdir;

#[doc(inline)]
pub use self::types::*;
