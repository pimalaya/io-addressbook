//! Vdir addressbook create coroutine wrapping
//! [`io_vdir::collection::create::VdirCollectionCreate`].
//!
//! # Example
//!
//! ```rust,ignore
//! let id = client.create_addressbook("personal", None, None)?;
//! ```

use alloc::string::{String, ToString};

use io_vdir::{
    collection::{
        Collection,
        create::{VdirCollectionCreate, VdirCollectionCreateError, VdirCollectionCreateOptions},
    },
    coroutine::*,
    path::VdirPath,
};
use log::trace;
use thiserror::Error;

/// Errors produced by [`VdirAddressbookCreate`].
#[derive(Debug, Error)]
pub enum VdirAddressbookCreateError {
    #[error(transparent)]
    Create(#[from] VdirCollectionCreateError),
    #[error("Invalid addressbook name")]
    InvalidName,
}

/// I/O-free coroutine creating a Vdir addressbook collection.
///
/// On completion returns the new addressbook id (its directory name).
pub struct VdirAddressbookCreate {
    id: String,
    inner: VdirCollectionCreate,
}

impl VdirAddressbookCreate {
    /// Builds the coroutine creating addressbook `name` under `root`,
    /// rejecting an empty name.
    pub fn new(
        root: impl Into<VdirPath>,
        name: &str,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<Self, VdirAddressbookCreateError> {
        trace!("prepare vdir addressbook create");

        if name.is_empty() {
            return Err(VdirAddressbookCreateError::InvalidName);
        }

        let path = root.into().join(name);
        let collection = Collection {
            path,
            display_name: Some(name.to_string()),
            description: description.map(str::to_string),
            color: color.map(str::to_string),
        };

        Ok(Self {
            id: name.to_string(),
            inner: VdirCollectionCreate::new(collection, VdirCollectionCreateOptions::default()),
        })
    }
}

impl VdirCoroutine for VdirAddressbookCreate {
    type Yield = VdirYield;
    type Return = Result<String, VdirAddressbookCreateError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(Ok(())) => {
                VdirCoroutineState::Complete(Ok(self.id.clone()))
            }
            VdirCoroutineState::Complete(Err(err)) => VdirCoroutineState::Complete(Err(err.into())),
        }
    }
}
