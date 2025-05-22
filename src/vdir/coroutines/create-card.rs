use std::path::Path;

use io_fs::Io;
use io_vdir::{constants::VCF, coroutines::CreateItem, Item, ItemKind};
use log::debug;

use crate::Card;

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
