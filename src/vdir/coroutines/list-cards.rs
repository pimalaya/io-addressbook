use std::{collections::HashSet, path::Path};

use io_fs::io::FsIo;
use io_vdir::{
    coroutines::list_items::{ListItems, ListItemsError, ListItemsResult},
    item::ItemKind,
};
use thiserror::Error;

use crate::card::Card;

#[derive(Clone, Debug, Error)]
pub enum ListCardsError {
    #[error("List cards error")]
    ListItems(#[from] ListItemsError),
}

#[derive(Clone, Debug)]
pub enum ListCardsResult {
    Ok(HashSet<Card>),
    Err(ListItemsError),
    Io(FsIo),
}

#[derive(Debug)]
pub struct ListCards(ListItems);

impl ListCards {
    pub fn new(root: impl AsRef<Path>, addressbook_id: impl AsRef<str>) -> Self {
        Self(ListItems::new(root.as_ref().join(addressbook_id.as_ref())))
    }

    pub fn resume(&mut self, input: Option<FsIo>) -> ListCardsResult {
        let items = loop {
            match self.0.resume(input) {
                ListItemsResult::Ok(items) => break items,
                ListItemsResult::Err(err) => return ListCardsResult::Err(err.into()),
                ListItemsResult::Io(io) => return ListCardsResult::Io(io),
            }
        };

        let mut cards = HashSet::new();

        for item in items {
            let Some(parent) = item.path.parent() else {
                continue;
            };

            let Some(addressbook_id) = parent.file_stem() else {
                continue;
            };

            let Some(id) = item.path.file_stem() else {
                continue;
            };

            let ItemKind::Vcard(vcard) = item.kind else {
                continue;
            };

            cards.insert(Card {
                id: id.to_string_lossy().to_string(),
                addressbook_id: addressbook_id.to_string_lossy().to_string(),
                vcard,
            });
        }

        ListCardsResult::Ok(cards)
    }
}
