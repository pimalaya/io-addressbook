use std::collections::HashSet;

use io_fs::Io;
use io_vdir::{coroutines::ListItems, ItemKind};

use crate::{Addressbook, Card};

#[derive(Debug)]
pub struct ListCards(ListItems);

impl ListCards {
    pub fn new(addressbook: &Addressbook) -> Self {
        Self(ListItems::new(addressbook))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<HashSet<Card>, Io> {
        let items = self.0.resume(input)?;
        let mut cards = HashSet::new();

        for item in items {
            let ItemKind::Vcard(vcard) = item.kind else {
                continue;
            };

            cards.insert(Card {
                addressbook_path: item.collection_path,
                name: item.name,
                vcard,
            });
        }

        Ok(cards)
    }
}
