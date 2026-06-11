//! Vdir card list coroutine wrapping
//! [`io_vdir::item::list::VdirItemList`].
//!
//! Filters to Vcard items and sorts by id. Pagination is applied by
//! the client method, not here.
//!
//! # Example
//!
//! ```rust,ignore
//! let cards = client.list_cards("personal", None, None)?;
//! ```

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use io_vdir::{
    coroutine::*,
    item::{
        ItemKind,
        list::{VdirItemList, VdirItemListError, VdirItemListOptions},
    },
    path::VdirPath,
};
use log::trace;
use thiserror::Error;

use crate::{card::Card, vdir::convert::card_from_item};

/// Errors produced by [`VdirCardList`].
#[derive(Debug, Error)]
pub enum VdirCardListError {
    #[error(transparent)]
    List(#[from] VdirItemListError),
}

/// I/O-free coroutine listing every Vcard in a Vdir addressbook.
///
/// On completion keeps only Vcard items, maps each to a [`Card`], and
/// sorts the result by id.
pub struct VdirCardList {
    addressbook_id: String,
    inner: VdirItemList,
}

impl VdirCardList {
    /// Builds the coroutine listing cards of addressbook
    /// `addressbook_id` located at `path`.
    pub fn new(path: impl Into<VdirPath>, addressbook_id: &str) -> Self {
        trace!("prepare vdir card list");
        Self {
            addressbook_id: addressbook_id.to_string(),
            inner: VdirItemList::new(path, VdirItemListOptions::default()),
        }
    }
}

impl VdirCoroutine for VdirCardList {
    type Yield = VdirYield;
    type Return = Result<Vec<Card>, VdirCardListError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(Ok(items)) => {
                let mut cards: Vec<Card> = items
                    .into_iter()
                    .filter(|item| matches!(item.kind, ItemKind::Vcard))
                    .filter_map(|item| card_from_item(&self.addressbook_id, item))
                    .collect();
                cards.sort_by(|a, b| a.id.cmp(&b.id));
                VdirCoroutineState::Complete(Ok(cards))
            }
            VdirCoroutineState::Complete(Err(err)) => VdirCoroutineState::Complete(Err(err.into())),
        }
    }
}
