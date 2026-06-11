//! WebDAV addressbook create coroutine wrapping
//! [`io_webdav::rfc6352::addressbook::create::CreateAddressbook`].
//!
//! # Example
//!
//! ```rust,ignore
//! let id = client.create_addressbook("personal", None, None)?;
//! ```

use alloc::string::String;

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::addressbook::create::CreateAddressbook,
};
use log::trace;
use thiserror::Error;
use url::Url;

use crate::webdav::convert::{validate_id, wire_from_name};

/// Errors produced by [`WebdavAddressbookCreate`].
#[derive(Debug, Error)]
pub enum WebdavAddressbookCreateError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("Invalid addressbook `{0}`")]
    InvalidAddressbook(String),
}

/// I/O-free coroutine creating a WebDAV addressbook collection.
///
/// On completion returns the new addressbook id (its URL segment).
pub struct WebdavAddressbookCreate {
    id: String,
    inner: CreateAddressbook,
}

impl WebdavAddressbookCreate {
    /// Builds the coroutine creating addressbook `name` under
    /// `home_path`, rejecting an empty name.
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        home_path: &str,
        name: &str,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<Self, WebdavAddressbookCreateError> {
        trace!("prepare webdav addressbook create");

        let id = validate_id(name)
            .ok_or_else(|| WebdavAddressbookCreateError::InvalidAddressbook(String::new()))?;
        let wire = wire_from_name(&id, Some(name), description, color);

        Ok(Self {
            id,
            inner: CreateAddressbook::new(base_url, auth, user_agent, home_path, &wire),
        })
    }
}

impl WebdavCoroutine for WebdavAddressbookCreate {
    type Yield = WebdavYield;
    type Return = Result<String, WebdavAddressbookCreateError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> WebdavCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            WebdavCoroutineState::Yielded(y) => WebdavCoroutineState::Yielded(y),
            WebdavCoroutineState::Complete(Ok(())) => {
                WebdavCoroutineState::Complete(Ok(self.id.clone()))
            }
            WebdavCoroutineState::Complete(Err(err)) => {
                WebdavCoroutineState::Complete(Err(err.into()))
            }
        }
    }
}
