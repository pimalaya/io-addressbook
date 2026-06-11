//! WebDAV card get coroutine wrapping
//! [`io_webdav::rfc6352::card::read::ReadCard`].
//!
//! # Example
//!
//! ```rust,ignore
//! let card = client.get_card("personal", "card-id")?;
//! ```

use alloc::string::{String, ToString};

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::card::read::ReadCard,
};
use log::trace;
use thiserror::Error;
use url::Url;

use crate::card::Card;

/// Errors produced by [`WebdavCardGet`].
#[derive(Debug, Error)]
pub enum WebdavCardGetError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("Invalid card id `{0}`")]
    InvalidCardId(String),
}

/// I/O-free coroutine reading a single WebDAV card by id.
///
/// On completion builds a [`Card`] from the fetched body and ETag.
pub struct WebdavCardGet {
    addressbook_id: String,
    card_id: String,
    inner: ReadCard,
}

impl WebdavCardGet {
    /// Builds the coroutine reading card `card_id` from the collection
    /// at `addressbook_path` (the addressbook `addressbook_id`),
    /// rejecting an empty card id.
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        addressbook_path: &str,
        addressbook_id: &str,
        card_id: &str,
    ) -> Result<Self, WebdavCardGetError> {
        trace!("prepare webdav card get");

        if card_id.is_empty() {
            return Err(WebdavCardGetError::InvalidCardId(String::new()));
        }

        Ok(Self {
            addressbook_id: addressbook_id.to_string(),
            card_id: card_id.to_string(),
            inner: ReadCard::new(base_url, auth, user_agent, addressbook_path, card_id),
        })
    }
}

impl WebdavCoroutine for WebdavCardGet {
    type Yield = WebdavYield;
    type Return = Result<Card, WebdavCardGetError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> WebdavCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            WebdavCoroutineState::Yielded(y) => WebdavCoroutineState::Yielded(y),
            WebdavCoroutineState::Complete(Ok(body)) => {
                let card = Card {
                    id: self.card_id.clone(),
                    addressbook_id: self.addressbook_id.clone(),
                    etag: body.etag,
                    contents: body.data,
                };
                WebdavCoroutineState::Complete(Ok(card))
            }
            WebdavCoroutineState::Complete(Err(err)) => {
                WebdavCoroutineState::Complete(Err(err.into()))
            }
        }
    }
}
