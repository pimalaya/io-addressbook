//! WebDAV addressbook list coroutine wrapping
//! [`io_webdav::rfc6352::addressbook::list::ListAddressbooks`].
//!
//! # Example
//!
//! ```rust,ignore
//! let addressbooks = client.list_addressbooks()?;
//! ```

use alloc::vec::Vec;

use io_webdav::{
    coroutine::*,
    rfc4918::{WebdavAuth, send::SendError},
    rfc6352::addressbook::list::ListAddressbooks,
};
use log::trace;
use thiserror::Error;
use url::Url;

use crate::{addressbook::Addressbook, webdav::convert::addressbook_from_wire};

/// Errors produced by [`WebdavAddressbookList`].
#[derive(Debug, Error)]
pub enum WebdavAddressbookListError {
    #[error(transparent)]
    Send(#[from] SendError),
}

/// I/O-free coroutine listing every WebDAV addressbook under the
/// home-set.
///
/// On completion maps each wire addressbook to an [`Addressbook`] and
/// sorts the result by name.
pub struct WebdavAddressbookList {
    inner: ListAddressbooks,
}

impl WebdavAddressbookList {
    /// Builds the coroutine listing addressbooks under `home_path`.
    pub fn new(base_url: &Url, auth: &WebdavAuth, user_agent: &str, home_path: &str) -> Self {
        trace!("prepare webdav addressbook list");
        Self {
            inner: ListAddressbooks::new(base_url, auth, user_agent, home_path),
        }
    }
}

impl WebdavCoroutine for WebdavAddressbookList {
    type Yield = WebdavYield;
    type Return = Result<Vec<Addressbook>, WebdavAddressbookListError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> WebdavCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            WebdavCoroutineState::Yielded(y) => WebdavCoroutineState::Yielded(y),
            WebdavCoroutineState::Complete(Ok(wires)) => {
                let mut addressbooks: Vec<Addressbook> =
                    wires.into_iter().map(addressbook_from_wire).collect();
                addressbooks.sort_by(|a, b| a.name.cmp(&b.name));
                WebdavCoroutineState::Complete(Ok(addressbooks))
            }
            WebdavCoroutineState::Complete(Err(err)) => {
                WebdavCoroutineState::Complete(Err(err.into()))
            }
        }
    }
}
