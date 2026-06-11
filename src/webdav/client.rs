//! Std-blocking WebDAV addressbook client.
//!
//! Wraps an inner [`io_webdav::client::WebdavClientStd`] (the connected
//! stream plus discovery cache) and pumps io-addressbook WebDAV
//! coroutines against it via [`WebdavClient::run`]. Each shared-API
//! method first resolves the cached CardDAV home-set (running discovery
//! on the first call), then builds and runs the matching coroutine; the
//! inner client stays reachable through [`WebdavClient::inner`].

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use std::io::{Read, Write};

use io_webdav::{client::WebdavClientStd, coroutine::*};
use thiserror::Error;
use url::Url;

use crate::{
    addressbook::{
        Addressbook, AddressbookDiff,
        webdav::{
            create::{WebdavAddressbookCreate, WebdavAddressbookCreateError},
            delete::{WebdavAddressbookDelete, WebdavAddressbookDeleteError},
            list::{WebdavAddressbookList, WebdavAddressbookListError},
            update::{WebdavAddressbookUpdate, WebdavAddressbookUpdateError},
        },
    },
    card::{
        Card,
        webdav::{
            create::{WebdavCardCreate, WebdavCardCreateError},
            delete::{WebdavCardDelete, WebdavCardDeleteError},
            get::{WebdavCardGet, WebdavCardGetError},
            list::{WebdavCardList, WebdavCardListError},
            update::{WebdavCardUpdate, WebdavCardUpdateError},
        },
    },
    webdav::convert::paginate,
};

const READ_BUFFER_SIZE: usize = 16 * 1024;

/// Errors surfaced by [`WebdavClient`] while running a coroutine.
///
/// One variant per shared-API WebDAV coroutine, plus the I/O failures
/// from the run loop and the discovery failures from resolving the
/// CardDAV home-set.
#[derive(Debug, Error)]
pub enum WebdavClientError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Discovery(#[from] io_webdav::client::WebdavClientStdError),

    #[error(transparent)]
    AddressbookCreate(#[from] WebdavAddressbookCreateError),
    #[error(transparent)]
    AddressbookDelete(#[from] WebdavAddressbookDeleteError),
    #[error(transparent)]
    AddressbookList(#[from] WebdavAddressbookListError),
    #[error(transparent)]
    AddressbookUpdate(#[from] WebdavAddressbookUpdateError),

    #[error(transparent)]
    CardCreate(#[from] WebdavCardCreateError),
    #[error(transparent)]
    CardDelete(#[from] WebdavCardDeleteError),
    #[error(transparent)]
    CardGet(#[from] WebdavCardGetError),
    #[error(transparent)]
    CardList(#[from] WebdavCardListError),
    #[error(transparent)]
    CardUpdate(#[from] WebdavCardUpdateError),
}

/// Std-blocking WebDAV addressbook client built on a connected stream.
#[derive(Debug)]
pub struct WebdavClient {
    pub inner: WebdavClientStd,
}

impl WebdavClient {
    /// Wraps an already-built inner client.
    pub fn new(inner: WebdavClientStd) -> Self {
        Self { inner }
    }

    /// Pumps any standard-shape WebDAV coroutine (`Yield =
    /// WebdavYield`, `Return = Result<T, E>`) against the inner stream
    /// until it terminates.
    pub fn run<C, T, E>(&mut self, mut coroutine: C) -> Result<T, WebdavClientError>
    where
        C: WebdavCoroutine<Yield = WebdavYield, Return = Result<T, E>>,
        WebdavClientError: From<E>,
    {
        let mut buf = [0u8; READ_BUFFER_SIZE];
        let mut arg: Option<&[u8]> = None;

        loop {
            match coroutine.resume(arg.take()) {
                WebdavCoroutineState::Complete(Ok(out)) => return Ok(out),
                WebdavCoroutineState::Complete(Err(err)) => return Err(err.into()),
                WebdavCoroutineState::Yielded(WebdavYield::WantsRead) => {
                    let n = self.inner.stream.read(&mut buf)?;
                    arg = Some(&buf[..n]);
                }
                WebdavCoroutineState::Yielded(WebdavYield::WantsWrite(bytes)) => {
                    self.inner.stream.write_all(&bytes)?;
                }
            }
        }
    }

    /// Lists every addressbook under the discovered home-set, sorted by
    /// name.
    pub fn list_addressbooks(&mut self) -> Result<Vec<Addressbook>, WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = home.path().to_string();

        let coroutine = WebdavAddressbookList::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
        );
        self.run(coroutine)
    }

    /// Creates an addressbook named `name` under the home-set. Returns
    /// the new addressbook id.
    pub fn create_addressbook(
        &mut self,
        name: &str,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<String, WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = home.path().to_string();

        let coroutine = WebdavAddressbookCreate::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            name,
            description,
            color,
        )?;
        self.run(coroutine)
    }

    /// Applies `patch` to the addressbook identified by `id`.
    pub fn update_addressbook(
        &mut self,
        id: &str,
        patch: AddressbookDiff,
    ) -> Result<(), WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = home.path().to_string();

        let coroutine = WebdavAddressbookUpdate::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            id,
            patch,
        )?;
        self.run(coroutine)
    }

    /// Deletes the addressbook identified by `id`.
    pub fn delete_addressbook(&mut self, id: &str) -> Result<(), WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = home.path().to_string();

        let coroutine = WebdavAddressbookDelete::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            id,
        )?;
        self.run(coroutine)
    }

    /// Lists cards inside `addressbook_id`, applying 1-indexed
    /// pagination.
    pub fn list_cards(
        &mut self,
        addressbook_id: &str,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Card>, WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = collection_path(&home, addressbook_id);

        let coroutine = WebdavCardList::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            addressbook_id,
        );
        let cards = self.run(coroutine)?;
        Ok(paginate(cards, page, page_size))
    }

    /// Fetches `card_id` from `addressbook_id`.
    pub fn get_card(
        &mut self,
        addressbook_id: &str,
        card_id: &str,
    ) -> Result<Card, WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = collection_path(&home, addressbook_id);

        let coroutine = WebdavCardGet::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            addressbook_id,
            card_id,
        )?;
        self.run(coroutine)
    }

    /// Appends a new vCard to `addressbook_id`. Returns its assigned id.
    pub fn create_card(
        &mut self,
        addressbook_id: &str,
        contents: Vec<u8>,
    ) -> Result<String, WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = collection_path(&home, addressbook_id);

        let coroutine = WebdavCardCreate::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            contents,
        )?;
        self.run(coroutine)
    }

    /// Overwrites `card_id` inside `addressbook_id`, gating on
    /// `if_match` when present.
    pub fn update_card(
        &mut self,
        addressbook_id: &str,
        card_id: &str,
        contents: Vec<u8>,
        if_match: Option<&str>,
    ) -> Result<(), WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = collection_path(&home, addressbook_id);

        let coroutine = WebdavCardUpdate::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            card_id,
            contents,
            if_match,
        )?;
        self.run(coroutine)
    }

    /// Permanently deletes `card_id` from `addressbook_id`.
    pub fn delete_card(
        &mut self,
        addressbook_id: &str,
        card_id: &str,
    ) -> Result<(), WebdavClientError> {
        let home = self.inner.addressbook_home_set()?;
        let path = collection_path(&home, addressbook_id);

        let coroutine = WebdavCardDelete::new(
            &self.inner.base_url,
            self.inner.auth(),
            &self.inner.user_agent,
            &path,
            card_id,
        )?;
        self.run(coroutine)
    }
}

/// Builds the collection path of `addressbook_id` under the home-set
/// URL, mirroring io_webdav's own `addressbook_path` (trim the
/// home-set trailing slash and the id's surrounding slashes).
fn collection_path(home: &Url, addressbook_id: &str) -> String {
    let base = home.path().trim_end_matches('/');
    let id = addressbook_id.trim_matches('/');
    format!("{base}/{id}")
}
