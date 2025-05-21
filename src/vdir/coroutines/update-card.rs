use std::path::{Path, PathBuf};

use io_fs::{
    coroutines::{CreateFile, Rename},
    Io,
};
use io_vdir::coroutines::UpdateItem;
use log::debug;

use crate::Card;

#[derive(Debug)]
pub struct UpdateCard(UpdateItem);

impl UpdateCard {
    pub fn new(card: &Card, contents: impl IntoIterator<Item = u8>) -> Self {
        let path: &Path = card.as_ref();
        let path_tmp = path.with_extension(format!("{}.tmp", card.extension()));
        let flow = CreateFile::new(&path_tmp, contents);
        let state = State::CreateTemporaryCard(flow);

        Self {
            path,
            path_tmp,
            state,
        }
    }

    pub fn resume(&mut self, io: Option<Io>) -> Result<(), Io> {
        match self.0.resume(io) {
            Ok(()) => {
                debug!("resume after updating vcf file");
                Ok(())
            }
            Err(io) => {
                debug!("break: need I/O to update vcf file");
                Err(io)
            }
        }
    }
}
