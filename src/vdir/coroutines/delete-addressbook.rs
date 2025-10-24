use std::path::Path;

use io_fs::io::FsIo;
use io_vdir::coroutines::delete_collection::{
    DeleteCollection, DeleteCollectionError, DeleteCollectionResult,
};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum DeleteAddressbookError {
    #[error("Delete addressbook error")]
    DeleteItem(#[from] DeleteCollectionError),
}

#[derive(Clone, Debug)]
pub enum DeleteAddressbookResult {
    Ok,
    Err(DeleteAddressbookError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct DeleteAddressbook(DeleteCollection);

impl DeleteAddressbook {
    pub fn new(root: impl AsRef<Path>, id: impl AsRef<str>) -> Self {
        Self(DeleteCollection::new(root.as_ref().join(id.as_ref())))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> DeleteAddressbookResult {
        match self.0.resume(input) {
            DeleteCollectionResult::Ok => DeleteAddressbookResult::Ok,
            DeleteCollectionResult::Err(err) => DeleteAddressbookResult::Err(err.into()),
            DeleteCollectionResult::Io(io) => DeleteAddressbookResult::Io(io),
        }
    }
}
