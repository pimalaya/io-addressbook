use std::path::Path;

use io_fs::Io;
use io_vdir::{constants::VCF, coroutines::ReadItem, ItemKind};

use crate::Card;

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

    pub fn resume(&mut self, input: Option<Io>) -> Result<Card, Io> {
        let item = self.0.resume(input)?;

        let Some(parent) = item.path.parent() else {
            return Err(Io::error("invalid item addressbook path"));
        };

        let Some(addressbook_id) = parent.file_stem() else {
            return Err(Io::error("invalid item addressbook id"));
        };

        let Some(id) = item.path.file_stem() else {
            return Err(Io::error("invalid item id"));
        };

        let ItemKind::Vcard(vcard) = item.kind else {
            return Err(Io::error("invalid vcard"));
        };

        let card = Card {
            id: id.to_string_lossy().to_string(),
            addressbook_id: addressbook_id.to_string_lossy().to_string(),
            vcard,
        };

        Ok(card)
    }
}
