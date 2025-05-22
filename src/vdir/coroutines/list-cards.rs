use std::{collections::HashSet, path::Path};

use io_fs::Io;
use io_vdir::{coroutines::ListItems, ItemKind};

use crate::Card;

#[derive(Debug)]
pub struct ListCards(ListItems);

impl ListCards {
    pub fn new(root: impl AsRef<Path>, addressbook_id: impl AsRef<str>) -> Self {
        Self(ListItems::new(root.as_ref().join(addressbook_id.as_ref())))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<HashSet<Card>, Io> {
        let items = self.0.resume(input)?;
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

        Ok(cards)
    }
}
