//! WebDAV card delete coroutine wrapping
//! [`io_webdav::rfc6352::card::delete::DeleteCard`].
//!
//! # Example
//!
//! ```rust,ignore
//! client.delete_card("personal", "card-id")?;
//! ```

use alloc::string::String;

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::card::delete::DeleteCard,
};
use log::trace;
use thiserror::Error;
use url::Url;

/// Errors produced by [`WebdavCardDelete`].
#[derive(Debug, Error)]
pub enum WebdavCardDeleteError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("Invalid card id `{0}`")]
    InvalidCardId(String),
}

/// I/O-free coroutine deleting a single WebDAV card.
pub struct WebdavCardDelete {
    inner: DeleteCard,
}

impl WebdavCardDelete {
    /// Builds the coroutine deleting card `card_id` from the collection
    /// at `addressbook_path`, rejecting an empty card id.
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        addressbook_path: &str,
        card_id: &str,
    ) -> Result<Self, WebdavCardDeleteError> {
        trace!("prepare webdav card delete");

        if card_id.is_empty() {
            return Err(WebdavCardDeleteError::InvalidCardId(String::new()));
        }

        Ok(Self {
            inner: DeleteCard::new(base_url, auth, user_agent, addressbook_path, card_id, None),
        })
    }
}

impl WebdavCoroutine for WebdavCardDelete {
    type Yield = WebdavYield;
    type Return = Result<(), WebdavCardDeleteError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> WebdavCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            WebdavCoroutineState::Yielded(y) => WebdavCoroutineState::Yielded(y),
            WebdavCoroutineState::Complete(Ok(_)) => WebdavCoroutineState::Complete(Ok(())),
            WebdavCoroutineState::Complete(Err(err)) => {
                WebdavCoroutineState::Complete(Err(err.into()))
            }
        }
    }
}
