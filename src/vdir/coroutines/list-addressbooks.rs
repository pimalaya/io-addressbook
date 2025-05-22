use std::{collections::HashSet, path::Path};

use io_fs::Io;
use io_vdir::coroutines::ListCollections;

use crate::Addressbook;

#[derive(Debug)]
pub struct ListAddressbooks(ListCollections);

impl ListAddressbooks {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self(ListCollections::new(root))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<HashSet<Addressbook>, Io> {
        let collections = self.0.resume(input)?;
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

        Ok(addressbooks)
    }
}
