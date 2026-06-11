//! WebDAV card create coroutine wrapping
//! [`io_webdav::rfc6352::card::create::CreateCard`].
//!
//! # Example
//!
//! ```rust,ignore
//! let id = client.create_card("personal", contents)?;
//! ```

use alloc::{string::String, vec::Vec};

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::card::create::CreateCard,
};
use log::trace;
use thiserror::Error;
use url::Url;

use crate::webdav::convert::fresh_card_id;

/// Errors produced by [`WebdavCardCreate`].
#[derive(Debug, Error)]
pub enum WebdavCardCreateError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("Card body is empty")]
    EmptyCardBody,
    #[error("Failed to gather randomness for new card id: {0}")]
    Random(getrandom::Error),
}

/// I/O-free coroutine creating a WebDAV card.
///
/// Synthesizes a fresh resource id; on completion returns the id the
/// server confirmed.
pub struct WebdavCardCreate {
    inner: CreateCard,
}

impl WebdavCardCreate {
    /// Builds the coroutine creating a card from `contents` inside the
    /// collection at `addressbook_path`, rejecting an empty body.
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        addressbook_path: &str,
        contents: Vec<u8>,
    ) -> Result<Self, WebdavCardCreateError> {
        trace!("prepare webdav card create");

        if contents.is_empty() {
            return Err(WebdavCardCreateError::EmptyCardBody);
        }

        let card_id = fresh_card_id().map_err(WebdavCardCreateError::Random)?;

        Ok(Self {
            inner: CreateCard::new(
                base_url,
                auth,
                user_agent,
                addressbook_path,
                &card_id,
                contents,
            ),
        })
    }
}

impl WebdavCoroutine for WebdavCardCreate {
    type Yield = WebdavYield;
    type Return = Result<String, WebdavCardCreateError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> WebdavCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            WebdavCoroutineState::Yielded(y) => WebdavCoroutineState::Yielded(y),
            WebdavCoroutineState::Complete(Ok(ok)) => WebdavCoroutineState::Complete(Ok(ok.id)),
            WebdavCoroutineState::Complete(Err(err)) => {
                WebdavCoroutineState::Complete(Err(err.into()))
            }
        }
    }
}
