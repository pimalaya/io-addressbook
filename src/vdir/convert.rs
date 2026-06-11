//! Conversions between vdir wire types and the shared types used by
//! [`AddressbookClientStd`], plus the `From` impl that wraps an
//! already-built [`VdirClient`] into a fresh unified client with vdir
//! as the only registered backend.
//!
//! [`AddressbookClientStd`]: crate::client::AddressbookClientStd
//! [`VdirClient`]: io_vdir::client::VdirClient

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use io_vdir::{client::VdirClient, collection::Collection, item::Item, path::VdirPath};

#[cfg(feature = "client")]
use crate::vdir::client::VdirClientError;
use crate::{addressbook::Addressbook, card::Card};

#[cfg(feature = "client")]
impl From<crate::vdir::client::VdirClient> for crate::client::AddressbookClientStd {
    fn from(client: crate::vdir::client::VdirClient) -> Self {
        Self::Vdir(client)
    }
}

/// Maps a vdir [`Collection`] to a shared [`Addressbook`].
///
/// Uses the collection's final path segment as the id; the
/// `display_name` falls back to the id when absent. `ctag` stays
/// `None` since vdir has no collection-state token.
pub(crate) fn addressbook_from_collection(collection: Collection) -> Addressbook {
    let id = collection.id().to_string();
    let name = collection
        .display_name
        .clone()
        .unwrap_or_else(|| id.clone());

    Addressbook {
        id,
        name,
        description: collection.description,
        color: collection.color,
        ctag: None,
    }
}

/// Builds the on-disk path of `addressbook_id` under `root`.
///
/// Forwards to [`VdirPath::join`]; performs no filesystem check.
pub(crate) fn addressbook_path(root: &VdirPath, addressbook_id: &str) -> VdirPath {
    root.join(addressbook_id)
}

/// Maps a vdir [`Item`] to a shared [`Card`].
///
/// The card id is the item's file stem; non-vCard items are filtered
/// out by the caller via [`io_vdir::item::ItemKind`]. ETag is `None`
/// because vdir has no entity tag concept.
pub(crate) fn card_from_item(addressbook_id: &str, item: Item) -> Option<Card> {
    let id = item.id()?.to_string();

    Some(Card {
        id,
        addressbook_id: addressbook_id.to_string(),
        etag: None,
        contents: item.contents,
    })
}

/// 1-indexed pagination on an in-memory list. `page_size = None`
/// returns the full slice; `page_size = 0` or a page past the end
/// returns an empty vector.
pub(crate) fn paginate<T>(items: Vec<T>, page: Option<u32>, page_size: Option<u32>) -> Vec<T> {
    let Some(size) = page_size else {
        return items;
    };

    if size == 0 {
        return Vec::new();
    }

    let page = page.unwrap_or(1).max(1);
    let skip = ((page - 1) as usize).saturating_mul(size as usize);

    if skip >= items.len() {
        return Vec::new();
    }

    items.into_iter().skip(skip).take(size as usize).collect()
}

/// Resolves `addressbook_id` against the inner client root, returning
/// a [`VdirClientError::InvalidAddressbook`] when the id is empty.
#[cfg(feature = "client")]
pub(crate) fn resolve_addressbook_path(
    client: &VdirClient,
    addressbook_id: &str,
) -> Result<VdirPath, VdirClientError> {
    if addressbook_id.is_empty() {
        return Err(VdirClientError::InvalidAddressbook(String::new()));
    }
    Ok(addressbook_path(client.root(), addressbook_id))
}
