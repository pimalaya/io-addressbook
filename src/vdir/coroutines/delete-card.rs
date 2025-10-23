use std::path::Path;

use io_fs::io::FsIo;
use io_vdir::{
    constants::VCF,
    coroutines::delete_item::{DeleteItem, DeleteItemError, DeleteItemResult},
};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum DeleteCardError {
    #[error("Delete card error")]
    DeleteItem(#[from] DeleteItemError),
}

#[derive(Clone, Debug)]
pub enum DeleteCardResult {
    Ok,
    Err(DeleteCardError),
    Io(FsIo),
}

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

    pub fn resume(&mut self, input: Option<FsIo>) -> DeleteCardResult {
        match self.0.resume(input) {
            DeleteItemResult::Ok => DeleteCardResult::Ok,
            DeleteItemResult::Err(err) => DeleteCardResult::Err(err.into()),
            DeleteItemResult::Io(io) => DeleteCardResult::Io(io),
        }
    }
}
