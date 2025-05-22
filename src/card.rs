use std::hash::{Hash, Hasher};

use calcard::vcard::{VCard, VCardEntry};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Card {
    pub id: String,
    pub addressbook_id: String,
    pub vcard: VCard,
}

impl Card {
    pub fn new(addressbook_id: impl ToString, vcard: VCard) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            addressbook_id: addressbook_id.to_string(),
            vcard,
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = &VCardEntry> {
        self.vcard.entries.iter()
    }
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.addressbook_id.hash(state);
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        self.vcard.to_string()
    }
}
