use std::path::{Path, PathBuf};

use io_fs::io::FsIo;
use io_vdir::{
    constants::VCF,
    coroutines::read_item::{ReadItem, ReadItemError, ReadItemResult},
    item::ItemKind,
};
use thiserror::Error;

use crate::card::Card;

#[derive(Clone, Debug, Error)]
pub enum ReadCardError {
    #[error("Read card error")]
    ReadItem(#[from] ReadItemError),
    #[error("Invalid card path {0}")]
    InvalidAddressbookPath(PathBuf),
    #[error("Invalid addressbook id at {0}")]
    InvalidAddressbookId(PathBuf),
    #[error("Invalid card id at {0}")]
    InvalidCardId(PathBuf),
    #[error("Invalid card at {0}")]
    InvalidCard(PathBuf),
}

#[derive(Clone, Debug)]
pub enum ReadCardResult {
    Ok(Card),
    Err(ReadCardError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct ReadCard(ReadItem);

impl ReadCard {
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

        Self(ReadItem::new(path))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> ReadCardResult {
        let item = loop {
            match self.0.resume(input) {
                ReadItemResult::Ok(item) => break item,
                ReadItemResult::Err(err) => return ReadCardResult::Err(err.into()),
                ReadItemResult::Io(io) => return ReadCardResult::Io(io),
            }
        };

        let p = &item.path;

        let Some(parent) = p.parent() else {
            return ReadCardResult::Err(ReadCardError::InvalidAddressbookPath(p.to_owned()));
        };

        let Some(addressbook_id) = parent.file_stem() else {
            return ReadCardResult::Err(ReadCardError::InvalidAddressbookId(p.to_owned()));
        };

        let Some(id) = p.file_stem() else {
            return ReadCardResult::Err(ReadCardError::InvalidCardId(p.to_owned()));
        };

        let ItemKind::Vcard(vcard) = item.kind else {
            return ReadCardResult::Err(ReadCardError::InvalidCard(p.to_owned()));
        };

        let card = Card {
            id: id.to_string_lossy().to_string(),
            addressbook_id: addressbook_id.to_string_lossy().to_string(),
            vcard,
        };

        ReadCardResult::Ok(card)
    }
}
