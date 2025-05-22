use std::path::Path;

use io_fs::Io;
use io_vdir::coroutines::DeleteCollection;

#[derive(Debug)]
pub struct DeleteAddressbook(DeleteCollection);

impl DeleteAddressbook {
    pub fn new(root: impl AsRef<Path>, id: impl AsRef<str>) -> Self {
        Self(DeleteCollection::new(root.as_ref().join(id.as_ref())))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
