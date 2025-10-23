use std::path::Path;

use io_fs::io::FsIo;
use io_vdir::{
    collection::Collection,
    coroutines::update_collection::{
        UpdateCollection, UpdateCollectionError, UpdateCollectionResult,
    },
};
use thiserror::Error;

use crate::addressbook::Addressbook;

#[derive(Clone, Debug, Error)]
pub enum UpdateAddressbookError {
    #[error("Update addressbook error")]
    UpdateCollection(#[from] UpdateCollectionError),
}

#[derive(Clone, Debug)]
pub enum UpdateAddressbookResult {
    Ok,
    Err(UpdateAddressbookError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct UpdateAddressbook(UpdateCollection);

impl UpdateAddressbook {
    pub fn new(root: impl AsRef<Path>, addressbook: Addressbook) -> Self {
        Self(UpdateCollection::new(Collection {
            path: root.as_ref().join(addressbook.id),
            display_name: addressbook.display_name,
            description: addressbook.description,
            color: addressbook.color,
        }))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> UpdateAddressbookResult {
        match self.0.resume(input) {
            UpdateCollectionResult::Ok => UpdateAddressbookResult::Ok,
            UpdateCollectionResult::Err(err) => UpdateAddressbookResult::Err(err.into()),
            UpdateCollectionResult::Io(io) => UpdateAddressbookResult::Io(io),
        }
    }
}
