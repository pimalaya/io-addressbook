#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

extern crate alloc;
#[cfg(feature = "client")]
extern crate std;

pub mod addressbook;
pub mod card;
#[cfg(feature = "client")]
#[cfg(any(feature = "vdir", feature = "webdav"))]
pub mod client;
#[cfg(feature = "vdir")]
pub mod vdir;
#[cfg(feature = "webdav")]
pub mod webdav;
