//! Vdir card delete coroutine wrapping
//! [`io_vdir::item::delete::VdirItemDelete`].
//!
//! # Example
//!
//! ```rust,ignore
//! client.delete_card("personal", "card-id")?;
//! ```

use alloc::string::ToString;

use io_vdir::{
    coroutine::*,
    item::delete::{VdirItemDelete, VdirItemDeleteError, VdirItemDeleteOptions},
    path::VdirPath,
};
use log::trace;
use thiserror::Error;

/// Errors produced by [`VdirCardDelete`].
#[derive(Debug, Error)]
pub enum VdirCardDeleteError {
    #[error(transparent)]
    Delete(#[from] VdirItemDeleteError),
}

/// I/O-free coroutine locating then removing a Vdir card by its id.
pub struct VdirCardDelete {
    inner: VdirItemDelete,
}

impl VdirCardDelete {
    /// Builds the coroutine deleting card `card_id` from the
    /// addressbook at `path`.
    pub fn new(path: impl Into<VdirPath>, card_id: impl ToString) -> Self {
        trace!("prepare vdir card delete");
        Self {
            inner: VdirItemDelete::new(path, card_id, VdirItemDeleteOptions::default()),
        }
    }
}

impl VdirCoroutine for VdirCardDelete {
    type Yield = VdirYield;
    type Return = Result<(), VdirCardDeleteError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(r) => VdirCoroutineState::Complete(r.map_err(Into::into)),
        }
    }
}
