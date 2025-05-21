use std::path::Path;

use io_fs::{coroutines::CreateFile, Io};
use io_vdir::ItemKind;
use log::debug;

#[derive(Debug)]
pub struct CreateCard(CreateFile);

impl CreateCard {
    pub fn new(
        collection_path: impl AsRef<Path>,
        name: impl AsRef<str>,
        contents: impl IntoIterator<Item = u8>,
    ) -> Self {
        let path = collection_path
            .as_ref()
            .join(name.as_ref())
            .with_extension(ItemKind::Vcard.as_extension());

        Self(CreateFile::new(path, contents))
    }

    pub fn resume(&mut self, io: Option<Io>) -> Result<(), Io> {
        match self.0.resume(io) {
            Ok(()) => {
                debug!("resume after creating vcf file");
                Ok(())
            }
            Err(io) => {
                debug!("break: need I/O to create vcf file");
                Err(io)
            }
        }
    }
}
