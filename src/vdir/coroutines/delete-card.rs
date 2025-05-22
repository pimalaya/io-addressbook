use std::path::Path;

use io_fs::Io;
use io_vdir::{constants::VCF, coroutines::DeleteItem};

#[derive(Debug)]
pub struct DeleteCard(DeleteItem);

impl DeleteCard {
    pub fn new(
        root: impl AsRef<Path>,
        addressbook_id: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> Self {
        let path = root
            .as_ref()
            .join(addressbook_id.as_ref())
            .join(id.as_ref())
            .with_extension(VCF);

        Self(DeleteItem::new(path))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
