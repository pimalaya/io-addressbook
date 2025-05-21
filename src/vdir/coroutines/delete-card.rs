use std::path::Path;

use io_fs::{coroutines::RemoveFile, Io};
use log::debug;

#[derive(Debug)]
pub struct DeleteCard(RemoveFile);

impl DeleteCard {
    pub fn new(card: impl AsRef<Path>) -> Self {
        Self(RemoveFile::new(card.as_ref()))
    }

    pub fn resume(&mut self, io: Option<Io>) -> Result<(), Io> {
        match self.0.resume(io) {
            Ok(()) => {
                debug!("resume after deleting vcf file");
                Ok(())
            }
            Err(io) => {
                debug!("break: need I/O to delete vcf file");
                Err(io)
            }
        }
    }
}
