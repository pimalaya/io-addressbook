use std::path::Path;

use io_fs::Io;
use io_vdir::{coroutines::CreateCollection, Collection};

use crate::Addressbook;

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

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
