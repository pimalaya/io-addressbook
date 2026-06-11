//! Std-blocking Vdir addressbook client.
//!
//! Wraps an inner [`io_vdir::client::VdirClient`] (the filesystem root)
//! and pumps io-addressbook Vdir coroutines directly against the local
//! filesystem via [`VdirClient::run`]. One shared-API method per
//! operation builds a coroutine and runs it; the inner client stays
//! reachable through [`VdirClient::inner`].

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    vec,
    vec::Vec,
};
use std::{fs, io};

use getrandom::fill;
use io_vdir::{
    client::VdirClient as InnerVdirClient, collection::Collection, coroutine::*, path::VdirPath,
};
use log::trace;
use thiserror::Error;

use crate::{
    addressbook::{
        Addressbook, AddressbookDiff,
        vdir::{
            create::{VdirAddressbookCreate, VdirAddressbookCreateError},
            delete::{VdirAddressbookDelete, VdirAddressbookDeleteError},
            list::{VdirAddressbookList, VdirAddressbookListError},
            update::{VdirAddressbookUpdate, VdirAddressbookUpdateError},
        },
    },
    card::{
        Card,
        vdir::{
            create::{VdirCardCreate, VdirCardCreateError},
            delete::{VdirCardDelete, VdirCardDeleteError},
            get::{VdirCardGet, VdirCardGetError},
            list::{VdirCardList, VdirCardListError},
            update::{VdirCardUpdate, VdirCardUpdateError},
        },
    },
    vdir::convert::{paginate, resolve_addressbook_path},
};

/// Errors surfaced by [`VdirClient`] while running a coroutine.
///
/// One variant per shared-API Vdir coroutine, plus filesystem and
/// randomness failures from the run loop.
#[derive(Debug, Error)]
pub enum VdirClientError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Inner(#[from] io_vdir::client::VdirClientError),
    #[error("Failed to gather randomness for new card id: {0}")]
    Random(getrandom::Error),
    #[error("Invalid addressbook `{0}`")]
    InvalidAddressbook(String),
    #[error("Addressbook `{0}` not found")]
    AddressbookNotFound(String),

    #[error(transparent)]
    AddressbookCreate(#[from] VdirAddressbookCreateError),
    #[error(transparent)]
    AddressbookDelete(#[from] VdirAddressbookDeleteError),
    #[error(transparent)]
    AddressbookList(#[from] VdirAddressbookListError),
    #[error(transparent)]
    AddressbookUpdate(#[from] VdirAddressbookUpdateError),

    #[error(transparent)]
    CardCreate(#[from] VdirCardCreateError),
    #[error(transparent)]
    CardDelete(#[from] VdirCardDeleteError),
    #[error(transparent)]
    CardGet(#[from] VdirCardGetError),
    #[error(transparent)]
    CardList(#[from] VdirCardListError),
    #[error(transparent)]
    CardUpdate(#[from] VdirCardUpdateError),
}

/// Std-blocking Vdir addressbook client built on a filesystem root.
#[derive(Debug)]
pub struct VdirClient {
    pub inner: InnerVdirClient,
}

impl VdirClient {
    /// Wraps an already-built inner client.
    pub fn new(inner: InnerVdirClient) -> Self {
        Self { inner }
    }

    /// Pumps any standard-shape Vdir coroutine (`Yield = VdirYield`,
    /// `Return = Result<T, E>`) against the local filesystem until it
    /// terminates.
    pub fn run<C, T, E>(&self, mut coroutine: C) -> Result<T, VdirClientError>
    where
        C: VdirCoroutine<Yield = VdirYield, Return = Result<T, E>>,
        VdirClientError: From<E>,
    {
        let mut arg: Option<VdirReply> = None;

        loop {
            match coroutine.resume(arg.take()) {
                VdirCoroutineState::Complete(Ok(out)) => return Ok(out),
                VdirCoroutineState::Complete(Err(err)) => return Err(err.into()),
                VdirCoroutineState::Yielded(VdirYield::WantsRandom { len }) => {
                    let mut bytes = vec![0u8; len];
                    fill(&mut bytes).map_err(VdirClientError::Random)?;
                    arg = Some(VdirReply::Random(bytes));
                }
                VdirCoroutineState::Yielded(VdirYield::WantsFileExists(paths)) => {
                    let mut out = BTreeMap::new();
                    for path in paths {
                        let exists = fs::metadata(path.as_str())
                            .map(|m| m.is_file())
                            .unwrap_or(false);
                        trace!("file_exists {path}: {exists}");
                        out.insert(path, exists);
                    }
                    arg = Some(VdirReply::FileExists(out));
                }
                VdirCoroutineState::Yielded(VdirYield::WantsDirExists(paths)) => {
                    let mut out = BTreeMap::new();
                    for path in paths {
                        let exists = fs::metadata(path.as_str())
                            .map(|m| m.is_dir())
                            .unwrap_or(false);
                        trace!("dir_exists {path}: {exists}");
                        out.insert(path, exists);
                    }
                    arg = Some(VdirReply::DirExists(out));
                }
                VdirCoroutineState::Yielded(VdirYield::WantsDirRead(paths)) => {
                    let mut entries = BTreeMap::new();
                    for path in paths {
                        trace!("read_dir {path}");
                        let mut names = BTreeSet::new();
                        match fs::read_dir(path.as_str()) {
                            Ok(iter) => {
                                for entry in iter {
                                    let entry = entry?;
                                    names.insert(normalize_path(entry.path()));
                                }
                            }
                            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
                            Err(err) => return Err(err.into()),
                        }
                        entries.insert(path, names);
                    }
                    arg = Some(VdirReply::DirRead(entries));
                }
                VdirCoroutineState::Yielded(VdirYield::WantsFileRead(paths)) => {
                    let mut contents = BTreeMap::new();
                    for path in paths {
                        trace!("read_file {path}");
                        let bytes = fs::read(path.as_str())?;
                        contents.insert(path, bytes);
                    }
                    arg = Some(VdirReply::FileRead(contents));
                }
                VdirCoroutineState::Yielded(VdirYield::WantsFileCreate(files)) => {
                    for (path, bytes) in files {
                        trace!("write {path} ({} bytes)", bytes.len());
                        if let Some(parent) = std::path::Path::new(path.as_str()).parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::write(path.as_str(), &bytes)?;
                    }
                    arg = Some(VdirReply::FileCreate);
                }
                VdirCoroutineState::Yielded(VdirYield::WantsDirCreate(paths)) => {
                    for path in paths {
                        trace!("create_dir_all {path}");
                        fs::create_dir_all(path.as_str())?;
                    }
                    arg = Some(VdirReply::DirCreate);
                }
                VdirCoroutineState::Yielded(VdirYield::WantsDirRemove(paths)) => {
                    for path in paths {
                        trace!("remove_dir_all {path}");
                        fs::remove_dir_all(path.as_str())?;
                    }
                    arg = Some(VdirReply::DirRemove);
                }
                VdirCoroutineState::Yielded(VdirYield::WantsFileRemove(paths)) => {
                    for path in paths {
                        trace!("remove_file {path}");
                        fs::remove_file(path.as_str())?;
                    }
                    arg = Some(VdirReply::FileRemove);
                }
                VdirCoroutineState::Yielded(VdirYield::WantsRename(pairs)) => {
                    for (from, to) in pairs {
                        trace!("rename {from} -> {to}");
                        fs::rename(from.as_str(), to.as_str())?;
                    }
                    arg = Some(VdirReply::Rename);
                }
                VdirCoroutineState::Yielded(VdirYield::WantsCopy(pairs)) => {
                    for (from, to) in pairs {
                        trace!("copy {from} -> {to}");
                        fs::copy(from.as_str(), to.as_str())?;
                    }
                    arg = Some(VdirReply::Copy);
                }
            }
        }
    }

    /// Lists every addressbook under the configured root, sorted by
    /// name.
    pub fn list_addressbooks(&self) -> Result<Vec<Addressbook>, VdirClientError> {
        self.run(VdirAddressbookList::new(self.inner.root().clone()))
    }

    /// Creates an addressbook named `name` under the root. Returns the
    /// new addressbook id.
    pub fn create_addressbook(
        &self,
        name: &str,
        description: Option<&str>,
        color: Option<&str>,
    ) -> Result<String, VdirClientError> {
        self.run(VdirAddressbookCreate::new(
            self.inner.root().clone(),
            name,
            description,
            color,
        )?)
    }

    /// Applies `patch` to the addressbook identified by `id`, merging
    /// it against the current collection metadata.
    pub fn update_addressbook(
        &self,
        id: &str,
        patch: AddressbookDiff,
    ) -> Result<(), VdirClientError> {
        let path = resolve_addressbook_path(&self.inner, id)?;

        let collections = self.inner.list_collections()?;
        let current = collections
            .into_iter()
            .find(|c| c.id() == id)
            .ok_or_else(|| VdirClientError::AddressbookNotFound(id.to_string()))?;

        let next = Collection {
            path,
            display_name: match patch.name {
                Some(name) => Some(name),
                None => current.display_name,
            },
            description: match patch.description {
                Some(desc) => desc,
                None => current.description,
            },
            color: match patch.color {
                Some(color) => color,
                None => current.color,
            },
        };

        self.run(VdirAddressbookUpdate::new(next))
    }

    /// Recursively removes the addressbook identified by `id`.
    pub fn delete_addressbook(&self, id: &str) -> Result<(), VdirClientError> {
        let path = resolve_addressbook_path(&self.inner, id)?;
        self.run(VdirAddressbookDelete::new(path))
    }

    /// Lists cards inside `addressbook_id`, applying 1-indexed
    /// pagination.
    pub fn list_cards(
        &self,
        addressbook_id: &str,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<Card>, VdirClientError> {
        let path = resolve_addressbook_path(&self.inner, addressbook_id)?;
        let cards = self.run(VdirCardList::new(path, addressbook_id))?;
        Ok(paginate(cards, page, page_size))
    }

    /// Fetches `card_id` from `addressbook_id`.
    pub fn get_card(&self, addressbook_id: &str, card_id: &str) -> Result<Card, VdirClientError> {
        let path = resolve_addressbook_path(&self.inner, addressbook_id)?;
        self.run(VdirCardGet::new(path, addressbook_id, card_id))
    }

    /// Appends a new vCard to `addressbook_id`. Returns its assigned
    /// id.
    pub fn create_card(
        &self,
        addressbook_id: &str,
        contents: Vec<u8>,
    ) -> Result<String, VdirClientError> {
        let path = resolve_addressbook_path(&self.inner, addressbook_id)?;
        self.run(VdirCardCreate::new(path, contents)?)
    }

    /// Overwrites `card_id` inside `addressbook_id`. `if_match` is
    /// ignored: vdir has no entity-tag concept.
    pub fn update_card(
        &self,
        addressbook_id: &str,
        card_id: &str,
        contents: Vec<u8>,
        _if_match: Option<&str>,
    ) -> Result<(), VdirClientError> {
        let path = resolve_addressbook_path(&self.inner, addressbook_id)?;
        self.run(VdirCardUpdate::new(path, card_id, contents)?)
    }

    /// Permanently deletes `card_id` from `addressbook_id`.
    pub fn delete_card(&self, addressbook_id: &str, card_id: &str) -> Result<(), VdirClientError> {
        let path = resolve_addressbook_path(&self.inner, addressbook_id)?;
        self.run(VdirCardDelete::new(path, card_id))
    }
}

/// Normalizes a host [`std::path::PathBuf`] into a `/`-separated
/// [`VdirPath`].
fn normalize_path(path: std::path::PathBuf) -> VdirPath {
    let s = path.to_string_lossy().into_owned();
    #[cfg(windows)]
    let s = s.replace('\\', "/");
    VdirPath::new(s)
}
