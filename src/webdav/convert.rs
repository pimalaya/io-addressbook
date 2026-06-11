//! Conversions between WebDAV wire types and the shared types used by
//! [`AddressbookClientStd`], plus the `From` impl that wraps an
//! already-built [`WebdavClient`] into the unified client's WebDAV
//! variant.
//!
//! [`AddressbookClientStd`]: crate::client::AddressbookClientStd
//! [`WebdavClient`]: crate::webdav::client::WebdavClient

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use io_webdav::rfc6352::{addressbook::Addressbook as WireAddressbook, card::CardEntry};

use crate::{addressbook::Addressbook, card::Card};

#[cfg(feature = "client")]
impl From<crate::webdav::client::WebdavClient> for crate::client::AddressbookClientStd {
    fn from(client: crate::webdav::client::WebdavClient) -> Self {
        Self::Webdav(Box::new(client))
    }
}

/// Maps a WebDAV [`WireAddressbook`] to a shared [`Addressbook`].
///
/// The wire `display_name` falls back to the id when absent; `ctag`
/// stays `None` until WebDAV CTag discovery lands.
pub(crate) fn addressbook_from_wire(wire: WireAddressbook) -> Addressbook {
    let id = wire.id;
    let name = wire.display_name.clone().unwrap_or_else(|| id.clone());

    Addressbook {
        id,
        name,
        description: wire.description,
        color: wire.color,
        ctag: None,
    }
}

/// Maps a WebDAV [`CardEntry`] to a shared [`Card`].
pub(crate) fn card_from_entry(addressbook_id: &str, entry: CardEntry) -> Card {
    Card {
        id: entry.id,
        addressbook_id: addressbook_id.to_string(),
        etag: entry.etag,
        contents: entry.data,
    }
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

/// Builds a fresh [`WireAddressbook`] suitable for create/update
/// requests. Empty name falls back to the id.
pub(crate) fn wire_from_name(
    id: impl ToString,
    name: Option<&str>,
    description: Option<&str>,
    color: Option<&str>,
) -> WireAddressbook {
    let id = id.to_string();
    let display_name = name
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .or_else(|| Some(id.clone()));

    WireAddressbook {
        id,
        display_name,
        description: description.map(str::to_string),
        color: color.map(str::to_string),
    }
}

/// Validates a non-empty addressbook id, returning it owned. Returns
/// [`None`] when the id is empty.
pub(crate) fn validate_id(id: &str) -> Option<String> {
    if id.is_empty() {
        return None;
    }
    Some(id.to_string())
}

/// Generates a fresh CardDAV card id from the system entropy source.
///
/// CardDAV requires the caller to supply the resource name; the vCard
/// UID parsing path is gated behind the optional parser feature
/// upstream, so on the bare client API we always synthesize the id.
pub(crate) fn fresh_card_id() -> Result<String, getrandom::Error> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes)?;

    // NOTE: RFC 4122 4.4 stamps version 4 and variant 10xx.
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = [0u8; 36];
    let mut cursor = 0;
    for (i, byte) in bytes.iter().enumerate() {
        if matches!(i, 4 | 6 | 8 | 10) {
            out[cursor] = b'-';
            cursor += 1;
        }
        out[cursor] = HEX[(byte >> 4) as usize];
        out[cursor + 1] = HEX[(byte & 0x0f) as usize];
        cursor += 2;
    }

    Ok(String::from_utf8(out.to_vec()).expect("ASCII hex is always valid UTF-8"))
}
