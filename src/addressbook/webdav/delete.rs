//! WebDAV addressbook delete coroutine wrapping
//! [`io_webdav::rfc6352::addressbook::delete::DeleteAddressbook`].
//!
//! # Example
//!
//! ```rust,ignore
//! client.delete_addressbook("personal")?;
//! ```

use alloc::string::String;

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::addressbook::delete::DeleteAddressbook,
};
use log::trace;
use thiserror::Error;
use url::Url;

use crate::webdav::convert::validate_id;

/// Errors produced by [`WebdavAddressbookDelete`].
#[derive(Debug, Error)]
pub enum WebdavAddressbookDeleteError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("Invalid addressbook `{0}`")]
    InvalidAddressbook(String),
}

/// I/O-free coroutine deleting a WebDAV addressbook collection.
pub struct WebdavAddressbookDelete {
    inner: DeleteAddressbook,
}

impl WebdavAddressbookDelete {
    /// Builds the coroutine deleting addressbook `id` under
    /// `home_path`, rejecting an empty id.
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        home_path: &str,
        id: &str,
    ) -> Result<Self, WebdavAddressbookDeleteError> {
        trace!("prepare webdav addressbook delete");

        let id = validate_id(id)
            .ok_or_else(|| WebdavAddressbookDeleteError::InvalidAddressbook(String::new()))?;

        Ok(Self {
            inner: DeleteAddressbook::new(base_url, auth, user_agent, home_path, &id),
        })
    }
}

impl WebdavCoroutine for WebdavAddressbookDelete {
    type Yield = WebdavYield;
    type Return = Result<(), WebdavAddressbookDeleteError>;

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
