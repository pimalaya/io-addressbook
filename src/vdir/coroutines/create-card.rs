use std::path::Path;

use io_fs::io::FsIo;
use io_vdir::{
    constants::VCF,
    coroutines::create_item::{CreateItem, CreateItemError, CreateItemResult},
    item::{Item, ItemKind},
};
use thiserror::Error;

use crate::card::Card;

#[derive(Clone, Debug, Error)]
pub enum CreateCardError {
    #[error("Create card error")]
    CreateItem(#[from] CreateItemError),
}

#[derive(Clone, Debug)]
pub enum CreateCardResult {
    Ok,
    Err(CreateCardError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct CreateCard(CreateItem);

impl CreateCard {
    pub fn new(root: impl AsRef<Path>, card: Card) -> Self {
        let kind = ItemKind::Vcard(card.vcard);
        let path = root
            .as_ref()
            .join(card.addressbook_id)
            .join(card.id)
            .with_extension(VCF);

        Self(CreateItem::new(Item { path, kind }))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> CreateCardResult {
        match self.0.resume(input) {
            CreateItemResult::Ok => CreateCardResult::Ok,
            CreateItemResult::Err(err) => CreateCardResult::Err(err.into()),
            CreateItemResult::Io(io) => CreateCardResult::Io(io),
        }
    }
}
