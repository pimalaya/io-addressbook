//! Vdir addressbook update coroutine wrapping
//! [`io_vdir::collection::update::VdirCollectionUpdate`].
//!
//! The diff merge against the current collection happens in the client
//! method (which lists collections first); this coroutine only writes
//! the already-merged [`Collection`].
//!
//! # Example
//!
//! ```rust,ignore
//! client.update_addressbook("personal", patch)?;
//! ```

use io_vdir::{
    collection::{
        Collection,
        update::{VdirCollectionUpdate, VdirCollectionUpdateError, VdirCollectionUpdateOptions},
    },
    coroutine::*,
};
use log::trace;
use thiserror::Error;

/// Errors produced by [`VdirAddressbookUpdate`].
#[derive(Debug, Error)]
pub enum VdirAddressbookUpdateError {
    #[error(transparent)]
    Update(#[from] VdirCollectionUpdateError),
}

/// I/O-free coroutine rewriting a Vdir addressbook's metadata.
pub struct VdirAddressbookUpdate {
    inner: VdirCollectionUpdate,
}

impl VdirAddressbookUpdate {
    /// Builds the coroutine writing the already-merged `collection`
    /// metadata to disk.
    pub fn new(collection: Collection) -> Self {
        trace!("prepare vdir addressbook update");
        Self {
            inner: VdirCollectionUpdate::new(collection, VdirCollectionUpdateOptions::default()),
        }
    }
}

impl VdirCoroutine for VdirAddressbookUpdate {
    type Yield = VdirYield;
    type Return = Result<(), VdirAddressbookUpdateError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(r) => VdirCoroutineState::Complete(r.map_err(Into::into)),
        }
    }
}
