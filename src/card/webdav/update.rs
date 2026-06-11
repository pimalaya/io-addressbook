//! WebDAV card update coroutine wrapping
//! [`io_webdav::rfc6352::card::update::UpdateCard`].
//!
//! # Example
//!
//! ```rust,ignore
//! client.update_card("personal", "card-id", contents, None)?;
//! ```

use alloc::{string::String, vec::Vec};

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::card::update::UpdateCard,
};
use log::trace;
use thiserror::Error;
use url::Url;

/// Errors produced by [`WebdavCardUpdate`].
#[derive(Debug, Error)]
pub enum WebdavCardUpdateError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("Invalid card id `{0}`")]
    InvalidCardId(String),
    #[error("Card body is empty")]
    EmptyCardBody,
}

/// I/O-free coroutine overwriting an existing WebDAV card.
pub struct WebdavCardUpdate {
    inner: UpdateCard,
}

impl WebdavCardUpdate {
    /// Builds the coroutine overwriting card `card_id` in the
    /// collection at `addressbook_path` with `contents`, gating the
    /// write on `if_match` when present. Rejects an empty id or body.
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        addressbook_path: &str,
        card_id: &str,
        contents: Vec<u8>,
        if_match: Option<&str>,
    ) -> Result<Self, WebdavCardUpdateError> {
        trace!("prepare webdav card update");

        if card_id.is_empty() {
            return Err(WebdavCardUpdateError::InvalidCardId(String::new()));
        }

        if contents.is_empty() {
            return Err(WebdavCardUpdateError::EmptyCardBody);
        }

        Ok(Self {
            inner: UpdateCard::new(
                base_url,
                auth,
                user_agent,
                addressbook_path,
                card_id,
                contents,
                if_match,
            ),
        })
    }
}

impl WebdavCoroutine for WebdavCardUpdate {
    type Yield = WebdavYield;
    type Return = Result<(), WebdavCardUpdateError>;

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
