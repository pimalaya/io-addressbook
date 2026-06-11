//! Vdir card get coroutine wrapping
//! [`io_vdir::item::get::VdirItemGet`].
//!
//! # Example
//!
//! ```rust,ignore
//! let card = client.get_card("personal", "card-id")?;
//! ```

use alloc::string::{String, ToString};

use io_vdir::{
    coroutine::*,
    item::{
        ItemKind,
        get::{VdirItemGet, VdirItemGetError, VdirItemGetOptions},
    },
    path::VdirPath,
};
use log::trace;
use thiserror::Error;

use crate::{card::Card, vdir::convert::card_from_item};

/// Errors produced by [`VdirCardGet`].
#[derive(Debug, Error)]
pub enum VdirCardGetError {
    #[error(transparent)]
    Get(#[from] VdirItemGetError),
    #[error("Invalid card id `{0}`")]
    InvalidCardId(String),
}

/// I/O-free coroutine fetching a Vdir card by its id.
///
/// On completion the located item is converted to a [`Card`]; a
/// non-Vcard item or an item with no derivable id is rejected.
pub struct VdirCardGet {
    addressbook_id: String,
    card_id: String,
    inner: VdirItemGet,
}

impl VdirCardGet {
    /// Builds the coroutine fetching card `card_id` from the
    /// addressbook `addressbook_id` located at `path`.
    pub fn new(path: impl Into<VdirPath>, addressbook_id: &str, card_id: &str) -> Self {
        trace!("prepare vdir card get");
        Self {
            addressbook_id: addressbook_id.to_string(),
            card_id: card_id.to_string(),
            inner: VdirItemGet::new(path, card_id, VdirItemGetOptions::default()),
        }
    }
}

impl VdirCoroutine for VdirCardGet {
    type Yield = VdirYield;
    type Return = Result<Card, VdirCardGetError>;

    fn resume(&mut self, arg: Option<VdirReply>) -> VdirCoroutineState<Self::Yield, Self::Return> {
        match self.inner.resume(arg) {
            VdirCoroutineState::Yielded(y) => VdirCoroutineState::Yielded(y),
            VdirCoroutineState::Complete(Ok(item)) => {
                if !matches!(item.kind, ItemKind::Vcard) {
                    let err = VdirCardGetError::InvalidCardId(self.card_id.clone());
                    return VdirCoroutineState::Complete(Err(err));
                }

                match card_from_item(&self.addressbook_id, item) {
                    Some(card) => VdirCoroutineState::Complete(Ok(card)),
                    None => {
                        let err = VdirCardGetError::InvalidCardId(self.card_id.clone());
                        VdirCoroutineState::Complete(Err(err))
                    }
                }
            }
            VdirCoroutineState::Complete(Err(err)) => VdirCoroutineState::Complete(Err(err.into())),
        }
    }
}
