//! Vdir card update coroutine wrapping
//! [`io_vdir::item::store::VdirItemStore`] in overwrite mode.
//!
//! # Example
//!
//! ```rust,ignore
//! client.update_card("personal", "card-id", contents, None)?;
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

/// Errors produced by [`VdirCardUpdate`].
#[derive(Debug, Error)]
pub enum VdirCardUpdateError {
    #[error(transparent)]
    Store(#[from] VdirItemStoreError),
    #[error("Empty card body")]
    EmptyBody,
}

/// I/O-free coroutine overwriting an existing Vdir card's contents.
pub struct VdirCardUpdate {
    inner: VdirItemStore,
}

impl VdirCardUpdate {
    /// Builds the coroutine overwriting card `card_id` under the
    /// addressbook at `path`, rejecting empty contents.
    pub fn new(
        path: impl Into<VdirPath>,
        card_id: &str,
        contents: Vec<u8>,
    ) -> Result<Self, VdirCardUpdateError> {
        trace!("prepare vdir card update");

        if contents.is_empty() {
            return Err(VdirCardUpdateError::EmptyBody);
        }

        let id: String = card_id.into();
        Ok(Self {
            inner: VdirItemStore::new(
                path,
                Some(id),
                ItemKind::Vcard,
                contents,
                VdirItemStoreOptions::default(),
            ),
        })
    }
}

impl VdirCoroutine for VdirCardUpdate {
    type Yield = VdirYield;
    type Return = Result<(), VdirCardUpdateError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(Ok(_)) => VdirCoroutineState::Complete(Ok(())),
            VdirCoroutineState::Complete(Err(err)) => VdirCoroutineState::Complete(Err(err.into())),
        }
    }
}
