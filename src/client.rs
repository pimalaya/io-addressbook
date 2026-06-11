//! Std-blocking unified addressbook client.
//!
//! [`AddressbookClientStd`] is an enum over the single registered backend: a
//! value is exactly one of the compiled-in per-backend clients ([`VdirClient`],
//! [`WebdavClient`]). Unlike io-email's multi-backend `EmailClientStd` struct,
//! an addressbook account speaks one protocol at a time, so the unified client
//! is an enum rather than a bag of optional slots; dispatch is a plain `match`
//! with no priority order.
//!
//! Build one via the per-backend `From` impls (e.g.
//! `AddressbookClientStd::from(VdirClient::new(inner))`) or by naming the
//! variant directly.
//!
//! [`VdirClient`]: crate::vdir::client::VdirClient
//! [`WebdavClient`]: crate::webdav::client::WebdavClient

#[cfg(feature = "webdav")]
use alloc::boxed::Box;
use alloc::{string::String, vec::Vec};

use thiserror::Error;

use crate::{
    addressbook::{Addressbook, AddressbookDiff},
    card::Card,
};

/// Errors surfaced by [`AddressbookClientStd`].
///
/// Each variant flattens the registered backend's error type via
/// `#[from]`, so the `?` operator works across the dispatch boundary.
#[derive(Debug, Error)]
pub enum AddressbookClientStdError {
    #[cfg(feature = "vdir")]
    #[error(transparent)]
    Vdir(#[from] crate::vdir::client::VdirClientError),
    #[cfg(feature = "webdav")]
    #[error(transparent)]
    Webdav(#[from] crate::webdav::client::WebdavClientError),
}

/// Std-blocking unified addressbook client.
///
/// One variant per compiled-in backend; a value always holds exactly
/// one. Each shared-API method dispatches to the active backend's
/// matching method.
#[derive(Debug)]
pub enum AddressbookClientStd {
    #[cfg(feature = "vdir")]
    Vdir(crate::vdir::client::VdirClient),
    // NOTE: boxed because the WebDAV client (boxed stream, base URL, discovery
    // caches) dwarfs the vdir client (a filesystem root).
    #[cfg(feature = "webdav")]
    Webdav(Box<crate::webdav::client::WebdavClient>),
}

impl AddressbookClientStd {
    /// Lists every addressbook available to the active account.
    pub fn list_addressbooks(&mut self) -> Result<Vec<Addressbook>, AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.list_addressbooks()?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.list_addressbooks()?),
        }
    }

    /// Creates an addressbook named `name`, optionally carrying a
    /// description and a color. Returns the backend-assigned id.
    pub fn create_addressbook(
        &mut self,
        name: &str,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<String, AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.create_addressbook(name, description, color)?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.create_addressbook(name, description, color)?),
        }
    }

    /// Applies a partial update to the addressbook identified by `id`.
    /// Fields left as `None` in `patch` are preserved.
    pub fn update_addressbook(
        &mut self,
        id: &str,
        patch: AddressbookDiff,
    ) -> Result<(), AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.update_addressbook(id, patch)?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.update_addressbook(id, patch)?),
        }
    }

    /// Deletes the addressbook identified by `id` and every card it
    /// contains.
    pub fn delete_addressbook(&mut self, id: &str) -> Result<(), AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.delete_addressbook(id)?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.delete_addressbook(id)?),
        }
    }

    /// Lists cards inside `addressbook_id`. `page` is 1-indexed; pass
    /// `None` to default to page 1. `page_size = None` returns the full
    /// window.
    pub fn list_cards(
        &mut self,
        addressbook_id: &str,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Card>, AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.list_cards(addressbook_id, page, page_size)?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.list_cards(addressbook_id, page, page_size)?),
        }
    }

    /// Fetches the card `card_id` from `addressbook_id`.
    pub fn get_card(
        &mut self,
        addressbook_id: &str,
        card_id: &str,
    ) -> Result<Card, AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.get_card(addressbook_id, card_id)?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.get_card(addressbook_id, card_id)?),
        }
    }

    /// Appends a raw vCard to `addressbook_id`. Returns the identifier
    /// the backend assigned to the stored card.
    pub fn create_card(
        &mut self,
        addressbook_id: &str,
        contents: Vec<u8>,
    ) -> Result<String, AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.create_card(addressbook_id, contents)?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.create_card(addressbook_id, contents)?),
        }
    }

    /// Replaces the bytes of `card_id` inside `addressbook_id`.
    ///
    /// `if_match` is the backend-specific entity tag to gate the update
    /// on; pass `None` to overwrite unconditionally. Backends without
    /// ETag support (vdir) ignore it.
    pub fn update_card(
        &mut self,
        addressbook_id: &str,
        card_id: &str,
        contents: Vec<u8>,
        if_match: Option<&str>,
    ) -> Result<(), AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => {
                Ok(client.update_card(addressbook_id, card_id, contents, if_match)?)
            }
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => {
                Ok(client.update_card(addressbook_id, card_id, contents, if_match)?)
            }
        }
    }

    /// Permanently deletes `card_id` from `addressbook_id`.
    pub fn delete_card(
        &mut self,
        addressbook_id: &str,
        card_id: &str,
    ) -> Result<(), AddressbookClientStdError> {
        match self {
            #[cfg(feature = "vdir")]
            Self::Vdir(client) => Ok(client.delete_card(addressbook_id, card_id)?),
            #[cfg(feature = "webdav")]
            Self::Webdav(client) => Ok(client.delete_card(addressbook_id, card_id)?),
        }
    }
}
