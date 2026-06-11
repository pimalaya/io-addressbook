//! Vdir addressbook delete coroutine wrapping
//! [`io_vdir::collection::delete::VdirCollectionDelete`].
//!
//! # Example
//!
//! ```rust,ignore
//! client.delete_addressbook("personal")?;
//! ```

use io_vdir::{
    collection::delete::{
        VdirCollectionDelete, VdirCollectionDeleteError, VdirCollectionDeleteOptions,
    },
    coroutine::*,
    path::VdirPath,
};
use log::trace;
use thiserror::Error;

/// Errors produced by [`VdirAddressbookDelete`].
#[derive(Debug, Error)]
pub enum VdirAddressbookDeleteError {
    #[error(transparent)]
    Delete(#[from] VdirCollectionDeleteError),
}

/// I/O-free coroutine recursively removing a Vdir addressbook
/// collection.
pub struct VdirAddressbookDelete {
    inner: VdirCollectionDelete,
}

impl VdirAddressbookDelete {
    /// Builds the coroutine removing the addressbook rooted at `path`.
    pub fn new(path: impl Into<VdirPath>) -> Self {
        trace!("prepare vdir addressbook delete");
        Self {
            inner: VdirCollectionDelete::new(path, VdirCollectionDeleteOptions::default()),
        }
    }
}

impl VdirCoroutine for VdirAddressbookDelete {
    type Yield = VdirYield;
    type Return = Result<(), VdirAddressbookDeleteError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(r) => VdirCoroutineState::Complete(r.map_err(Into::into)),
        }
    }
}
