//! Addressbook domain: the shared [`Addressbook`] type and the
//! per-backend addressbook operations.

mod types;
#[cfg(feature = "vdir")]
pub mod vdir;
#[cfg(feature = "webdav")]
pub mod webdav;

#[doc(inline)]
pub use types::*;
