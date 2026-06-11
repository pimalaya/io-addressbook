//! WebDAV addressbook update coroutine wrapping
//! [`io_webdav::rfc6352::addressbook::update::UpdateAddressbook`].
//!
//! # Example
//!
//! ```rust,ignore
//! client.update_addressbook("personal", patch)?;
//! ```

use alloc::string::String;

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::addressbook::{Addressbook as WireAddressbook, update::UpdateAddressbook},
};
use log::trace;
use thiserror::Error;
use url::Url;

use crate::{addressbook::AddressbookDiff, webdav::convert::validate_id};

/// Errors produced by [`WebdavAddressbookUpdate`].
#[derive(Debug, Error)]
pub enum WebdavAddressbookUpdateError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("Invalid addressbook `{0}`")]
    InvalidAddressbook(String),
}

/// I/O-free coroutine updating a WebDAV addressbook collection's
/// properties from an [`AddressbookDiff`].
pub struct WebdavAddressbookUpdate {
    inner: UpdateAddressbook,
}

impl WebdavAddressbookUpdate {
    /// Builds the coroutine applying `patch` to addressbook `id` under
    /// `home_path`, rejecting an empty id.
    pub fn new(
        base_url: &Url,
        auth: &WebdavAuth,
        user_agent: &str,
        home_path: &str,
        id: &str,
        patch: AddressbookDiff,
    ) -> Result<Self, WebdavAddressbookUpdateError> {
        trace!("prepare webdav addressbook update");

        let id = validate_id(id)
            .ok_or_else(|| WebdavAddressbookUpdateError::InvalidAddressbook(String::new()))?;
        let wire = WireAddressbook {
            id,
            display_name: patch.name,
            description: patch.description.unwrap_or(None),
            color: patch.color.unwrap_or(None),
        };

        Ok(Self {
            inner: UpdateAddressbook::new(base_url, auth, user_agent, home_path, &wire),
        })
    }
}

impl WebdavCoroutine for WebdavAddressbookUpdate {
    type Yield = WebdavYield;
    type Return = Result<(), WebdavAddressbookUpdateError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> WebdavCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            WebdavCoroutineState::Yielded(y) => WebdavCoroutineState::Yielded(y),
            WebdavCoroutineState::Complete(r) => {
                WebdavCoroutineState::Complete(r.map_err(Into::into))
            }
        }
    }
}
