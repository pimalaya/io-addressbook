use std::path::Path;

use io_fs::io::FsIo;
use io_vdir::{
    constants::VCF,
    coroutines::update_item::{UpdateItem, UpdateItemError, UpdateItemResult},
    item::{Item, ItemKind},
};
use thiserror::Error;

use crate::card::Card;

#[derive(Clone, Debug, Error)]
pub enum UpdateCardError {
    #[error("Update card error")]
    UpdateItem(#[from] UpdateItemError),
}

#[derive(Clone, Debug)]
pub enum UpdateCardResult {
    Ok,
    Err(UpdateCardError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct UpdateCard(UpdateItem);

impl UpdateCard {
    pub fn new(root: impl AsRef<Path>, card: Card) -> Self {
        let kind = ItemKind::Vcard(card.vcard);
        let path = root
            .as_ref()
            .join(card.addressbook_id)
            .join(card.id)
            .with_extension(VCF);

        Self(UpdateItem::new(Item { path, kind }))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> UpdateCardResult {
        match self.0.resume(input) {
            UpdateItemResult::Ok => UpdateCardResult::Ok,
            UpdateItemResult::Err(err) => UpdateCardResult::Err(err.into()),
            UpdateItemResult::Io(io) => UpdateCardResult::Io(io),
        }
    }
}
