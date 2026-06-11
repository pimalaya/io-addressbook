//! Vdir addressbook list coroutine wrapping
//! [`io_vdir::collection::list::VdirCollectionList`].
//!
//! # Example
//!
//! ```rust,ignore
//! let addressbooks = client.list_addressbooks()?;
//! ```

use alloc::vec::Vec;

use io_vdir::{
    collection::list::{VdirCollectionList, VdirCollectionListError, VdirCollectionListOptions},
    coroutine::*,
    path::VdirPath,
};
use log::trace;
use thiserror::Error;

use crate::{addressbook::Addressbook, vdir::convert::addressbook_from_collection};

/// Errors produced by [`VdirAddressbookList`].
#[derive(Debug, Error)]
pub enum VdirAddressbookListError {
    #[error(transparent)]
    List(#[from] VdirCollectionListError),
}

/// I/O-free coroutine listing every Vdir addressbook under a root.
///
/// On completion maps each collection to an [`Addressbook`] and sorts
/// the result by name.
pub struct VdirAddressbookList {
    inner: VdirCollectionList,
}

impl VdirAddressbookList {
    /// Builds the coroutine listing addressbooks under `root`.
    pub fn new(root: impl Into<VdirPath>) -> Self {
        trace!("prepare vdir addressbook list");
        Self {
            inner: VdirCollectionList::new(root, VdirCollectionListOptions::default()),
        }
    }
}

impl VdirCoroutine for VdirAddressbookList {
    type Yield = VdirYield;
    type Return = Result<Vec<Addressbook>, VdirAddressbookListError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(Ok(collections)) => {
                let mut addressbooks: Vec<Addressbook> = collections
                    .into_iter()
                    .map(addressbook_from_collection)
                    .collect();
                addressbooks.sort_by(|a, b| a.name.cmp(&b.name));
                VdirCoroutineState::Complete(Ok(addressbooks))
            }
            VdirCoroutineState::Complete(Err(err)) => VdirCoroutineState::Complete(Err(err.into())),
        }
    }
}
