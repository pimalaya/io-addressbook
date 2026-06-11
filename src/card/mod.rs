//! Card domain: the shared [`Card`] type and the per-backend card operations.

mod types;
#[cfg(feature = "vdir")]
pub mod vdir;
#[cfg(feature = "webdav")]
pub mod webdav;

#[doc(inline)]
pub use types::*;
