//! WebDAV card list coroutine wrapping
//! [`io_webdav::rfc6352::card::list::ListCards`].
//!
//! # Example
//!
//! ```rust,ignore
//! let cards = client.list_cards("personal", None, None)?;
//! ```

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::card::list::ListCards,
};
use log::trace;
use thiserror::Error;
use url::Url;

use crate::{card::Card, webdav::convert::card_from_entry};

/// Errors produced by [`WebdavCardList`].
#[derive(Debug, Error)]
pub enum WebdavCardListError {
    #[error(transparent)]
    Send(#[from] SendError),
}

/// I/O-free coroutine listing every card inside a WebDAV addressbook
/// collection.
///
/// On completion maps each wire entry to a [`Card`] and sorts the
/// result by id. Pagination is applied by the client, not here.
pub struct WebdavCardList {
    addressbook_id: String,
    inner: ListCards,
}

impl WebdavCardList {
    /// Builds the coroutine listing cards in the collection at
    /// `addressbook_path` (the addressbook `addressbook_id`).
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        addressbook_path: &str,
        addressbook_id: &str,
    ) -> Self {
        trace!("prepare webdav card list");
        Self {
            addressbook_id: addressbook_id.to_string(),
            inner: ListCards::new(base_url, auth, user_agent, addressbook_path),
        }
    }
}

impl WebdavCoroutine for WebdavCardList {
    type Yield = WebdavYield;
    type Return = Result<Vec<Card>, WebdavCardListError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> WebdavCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            WebdavCoroutineState::Yielded(y) => WebdavCoroutineState::Yielded(y),
            WebdavCoroutineState::Complete(Ok(entries)) => {
                let mut cards: Vec<Card> = entries
                    .into_iter()
                    .map(|entry| card_from_entry(&self.addressbook_id, entry))
                    .collect();
                cards.sort_by(|a, b| a.id.cmp(&b.id));
                WebdavCoroutineState::Complete(Ok(cards))
            }
            WebdavCoroutineState::Complete(Err(err)) => {
                WebdavCoroutineState::Complete(Err(err.into()))
            }
        }
    }
}
