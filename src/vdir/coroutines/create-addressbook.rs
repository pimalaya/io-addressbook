use std::path::Path;

use io_fs::io::FsIo;
use io_vdir::{
    collection::Collection,
    coroutines::create_collection::{
        CreateCollection, CreateCollectionError, CreateCollectionResult,
    },
};
use thiserror::Error;

use crate::addressbook::Addressbook;

#[derive(Clone, Debug, Error)]
pub enum CreateAddressbookError {
    #[error("Create addressbook error")]
    CreateItem(#[from] CreateCollectionError),
}

#[derive(Clone, Debug)]
pub enum CreateAddressbookResult {
    Ok,
    Err(CreateAddressbookError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct CreateAddressbook(CreateCollection);

impl CreateAddressbook {
    pub fn new(root: impl AsRef<Path>, addressbook: Addressbook) -> Self {
        Self(CreateCollection::new(Collection {
            path: root.as_ref().join(addressbook.id),
            display_name: addressbook.display_name,
            description: addressbook.description,
            color: addressbook.color,
        }))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> CreateAddressbookResult {
        match self.0.resume(input) {
            CreateCollectionResult::Ok => CreateAddressbookResult::Ok,
            CreateCollectionResult::Err(err) => CreateAddressbookResult::Err(err.into()),
            CreateCollectionResult::Io(io) => CreateAddressbookResult::Io(io),
        }
    }
}
