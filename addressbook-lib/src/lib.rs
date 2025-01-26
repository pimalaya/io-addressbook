#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// #![doc = include_str!("../../README.md")]

#[cfg(feature = "carddav")]
pub mod carddav;
#[cfg(feature = "vdir")]
pub mod vdir;

mod types;

#[doc(inline)]
pub use self::types::*;
