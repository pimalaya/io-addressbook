use std::{collections::HashSet, path::Path};

use io_fs::io::FsIo;
use io_vdir::coroutines::list_collections::{
    ListCollections, ListCollectionsError, ListCollectionsResult,
};
use thiserror::Error;

use crate::addressbook::Addressbook;

#[derive(Clone, Debug, Error)]
pub enum ListAddressbooksError {
    #[error("List addressbooks error")]
    ListCollections(#[from] ListCollectionsError),
}

#[derive(Clone, Debug)]
pub enum ListAddressbooksResult {
    Ok(HashSet<Addressbook>),
    Err(ListAddressbooksError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct ListAddressbooks(ListCollections);

impl ListAddressbooks {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self(ListCollections::new(root))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> ListAddressbooksResult {
        let collections = loop {
            match self.0.resume(input) {
                ListCollectionsResult::Ok(collections) => break collections,
                ListCollectionsResult::Err(err) => return ListAddressbooksResult::Err(err.into()),
                ListCollectionsResult::Io(io) => return ListAddressbooksResult::Io(io),
            }
        };

        let mut addressbooks = HashSet::new();

        for collection in collections {
            let Some(id) = collection.path.file_stem() else {
                continue;
            };

            addressbooks.insert(Addressbook {
                id: id.to_string_lossy().to_string(),
                display_name: collection.display_name,
                description: collection.description,
                color: collection.color,
            });
        }

        ListAddressbooksResult::Ok(addressbooks)
    }
}
