use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use calcard::vcard::{VCard, VCardEntry};
use io_vdir::constants::VCF;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Card {
    pub addressbook_path: PathBuf,
    pub name: String,
    pub vcard: VCard,
}

impl Card {
    pub fn path(&self) -> PathBuf {
        self.addressbook_path
            .join(&self.name)
            .with_extension(self.extension())
    }

    pub fn extension(&self) -> &'static str {
        VCF
    }

    pub fn entries(&self) -> impl Iterator<Item = &VCardEntry> {
        self.vcard.entries.iter()
    }
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path().hash(state);
    }
}
