//! Card shared across all protocols.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "parser")]
use core::str::from_utf8;

#[cfg(feature = "parser")]
use calcard::{Entry, icalendar::ICalendarComponentType, vcard::VCard};
#[cfg(feature = "parser")]
use thiserror::Error;

/// Errors surfaced when parsing raw card bytes as a vCard.
#[cfg(feature = "parser")]
#[derive(Clone, Debug, Error)]
pub enum ParseCardError {
    #[error("Card contents are not valid UTF-8")]
    InvalidUtf8,
    #[error("Card contents parsed as iCalendar, not vCard")]
    NotAVcard,
    #[error("Invalid vCard line: {0}")]
    InvalidLine(String),
    #[error("Unexpected vCard EOF")]
    UnexpectedEof,
    #[error("Too many vCard components")]
    TooManyComponents,
    #[error("Unexpected vCard component end: expected {0:?} got {1:?}")]
    UnexpectedComponentEnd(ICalendarComponentType, ICalendarComponentType),
    #[error("Unterminated vCard component: {0}")]
    UnterminatedComponent(alloc::borrow::Cow<'static, str>),
    #[error("Unknown vCard parse error")]
    Unknown,
}

/// A single card inside an addressbook.
///
/// Strict least-common-denominator shape: contents stay raw bytes; the
/// optional `parser` feature exposes calcard-backed helpers on top.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct Card {
    /// Card identifier (file stem for vdir, last URL segment for
    /// CardDAV).
    pub id: String,

    /// Parent addressbook identifier.
    pub addressbook_id: String,

    /// Entity tag (RFC 9110 §8.8.3, without surrounding quotes) when
    /// the backend exposes it; vdir surfaces `None`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub etag: Option<String>,

    /// Raw vCard bytes.
    pub contents: Vec<u8>,
}

impl Card {
    /// Returns the raw card bytes.
    pub fn contents(&self) -> &[u8] {
        &self.contents
    }

    /// Parses the bytes as a vCard.
    #[cfg(feature = "parser")]
    pub fn as_vcard(&self) -> Result<VCard, ParseCardError> {
        let s = from_utf8(&self.contents).map_err(|_| ParseCardError::InvalidUtf8)?;
        parse_vcard(s)
    }
}

#[cfg(feature = "parser")]
fn parse_vcard(s: &str) -> Result<VCard, ParseCardError> {
    match VCard::parse(s) {
        Ok(vcard) => Ok(vcard),
        Err(Entry::VCard(vcard)) => Ok(vcard),
        Err(Entry::ICalendar(_)) => Err(ParseCardError::NotAVcard),
        Err(Entry::InvalidLine(line)) => Err(ParseCardError::InvalidLine(line)),
        Err(Entry::Eof) => Err(ParseCardError::UnexpectedEof),
        Err(Entry::TooManyComponents) => Err(ParseCardError::TooManyComponents),
        Err(Entry::UnexpectedComponentEnd { expected, found }) => {
            Err(ParseCardError::UnexpectedComponentEnd(expected, found))
        }
        Err(Entry::UnterminatedComponent(component)) => {
            Err(ParseCardError::UnterminatedComponent(component))
        }
        Err(_) => Err(ParseCardError::Unknown),
    }
}
