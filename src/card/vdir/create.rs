//! Vdir card create coroutine wrapping
//! [`io_vdir::item::store::VdirItemStore`] with a generated id.
//!
//! # Example
//!
//! ```rust,ignore
//! let id = client.create_card("personal", contents)?;
//! ```

use alloc::{string::String, vec::Vec};

use io_vdir::{
    coroutine::*,
    item::{
        ItemKind,
        store::{VdirItemStore, VdirItemStoreError, VdirItemStoreOptions},
    },
    path::VdirPath,
};
use log::trace;
use thiserror::Error;

/// Errors produced by [`VdirCardCreate`].
#[derive(Debug, Error)]
pub enum VdirCardCreateError {
    #[error(transparent)]
    Store(#[from] VdirItemStoreError),
    #[error("Empty card body")]
    EmptyBody,
}

/// I/O-free coroutine writing a new Vcard item under a collection.
///
/// The id is minted by the inner store coroutine; on completion the
/// generated card id is returned.
pub struct VdirCardCreate {
    inner: VdirItemStore,
}

impl VdirCardCreate {
    /// Builds the coroutine storing `contents` as a fresh Vcard item
    /// under the addressbook at `path`, rejecting empty contents.
    pub fn new(path: impl Into<VdirPath>, contents: Vec<u8>) -> Result<Self, VdirCardCreateError> {
        trace!("prepare vdir card create");

        if contents.is_empty() {
            return Err(VdirCardCreateError::EmptyBody);
        }

        Ok(Self {
            inner: VdirItemStore::new(
                path,
                None,
                ItemKind::Vcard,
                contents,
                VdirItemStoreOptions::default(),
            ),
        })
    }
}

impl VdirCoroutine for VdirCardCreate {
    type Yield = VdirYield;
    type Return = Result<String, VdirCardCreateError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(Ok(out)) => VdirCoroutineState::Complete(Ok(out.id)),
            VdirCoroutineState::Complete(Err(err)) => VdirCoroutineState::Complete(Err(err.into())),
        }
    }
}
