use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

use calcard::{icalendar::ICalendarComponentType, Entry};
use serde::{Serialize, Serializer};
use thiserror::Error;
use uuid::Uuid;

pub use calcard::vcard::*;

#[derive(Clone, Debug, Error)]
pub enum ParseCardError {
    #[error("Invalid vCard format: parsed iCal instead")]
    InvalidFormat,
    #[error("Invalid vCard line: {0}")]
    InvalidLine(String),
    #[error("Unexpected vCard EOF")]
    UnexpectedEof,
    #[error("Too many vCard components")]
    TooManyComponents,
    #[error("Unexpected vCard component end: expected {0:?} got {1:?}")]
    UnexpectedComponentEnd(ICalendarComponentType, ICalendarComponentType),
    #[error("Unterminated vCard component: {0}")]
    UnterminatedComponent(Cow<'static, str>),
    #[error("Unknown vCard error: {0:?}")]
    Unknown(Entry),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Card {
    pub id: String,
    pub addressbook_id: String,
    #[serde(serialize_with = "Card::serialize_vcard")]
    pub vcard: VCard,
}

impl Card {
    pub fn new_uuid() -> Uuid {
        Uuid::new_v4()
    }

    pub fn serialize_vcard<S: Serializer>(vcard: &VCard, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&vcard.to_string())
    }

    pub fn parse(contents: impl AsRef<str>) -> Result<VCard, ParseCardError> {
        match VCard::parse(contents) {
            Ok(vcard) => Ok(vcard),
            Err(Entry::VCard(vcard)) => Ok(vcard),
            Err(Entry::ICalendar(_)) => Err(ParseCardError::InvalidFormat),
            Err(Entry::InvalidLine(line)) => Err(ParseCardError::InvalidLine(line)),
            Err(Entry::Eof) => Err(ParseCardError::UnexpectedEof),
            Err(Entry::TooManyComponents) => Err(ParseCardError::TooManyComponents),
            Err(Entry::UnexpectedComponentEnd { expected, found }) => {
                Err(ParseCardError::UnexpectedComponentEnd(expected, found))
            }
            Err(Entry::UnterminatedComponent(component)) => {
                Err(ParseCardError::UnterminatedComponent(component))
            }
            Err(err) => Err(ParseCardError::Unknown(err)),
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
