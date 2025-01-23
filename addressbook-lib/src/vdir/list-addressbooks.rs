use std::{collections::HashMap, path::PathBuf};

use tracing::{debug, instrument, trace};

use crate::{Addressbook, Addressbooks};

use super::{fs, Config};

#[derive(Debug, Default)]
pub enum Step {
    #[default]
    ReadDir,
    ReadFiles,
    ReadMoreFiles,
    Done(Addressbooks),
}

#[derive(Debug, Default)]
pub struct ListAddressbooks {
    state: fs::State,
    step: Step,
    valid_addressbook_dirs: Vec<PathBuf>,
}

impl ListAddressbooks {
    pub fn new(config: &Config) -> Self {
        let mut this = Self::default();
        this.state.read_dir = Some((config.home_dir.clone(), None));
        this
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Addressbooks> {
        let Step::Done(addressbooks) = self.step else {
            debug!(?self.step, "invalid step to get output");
            return None;
        };

        Some(addressbooks)
    }
}

impl AsMut<fs::State> for ListAddressbooks {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for ListAddressbooks {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(step = ?self.step, state = ?self.state);

        match self.step {
            Step::Done(_) => None,
            Step::ReadDir => {
                self.step = Step::ReadFiles;
                Some(fs::Io::ReadDir)
            }
            Step::ReadFiles => {
                let Some((dir, Some(paths))) = &self.state.read_dir else {
                    debug!("invalid state for step {:?}", self.step);
                    return None;
                };

                let mut files = HashMap::default();

                for path in paths {
                    let Some(name) = path.file_name() else {
                        continue;
                    };

                    let path = dir.join(&name);

                    if !path.is_dir() {
                        continue;
                    }

                    let name_path = path.join("displayname");
                    if name_path.is_file() {
                        files.insert(name_path, None);
                    }

                    let desc_path = path.join("description");
                    if desc_path.is_file() {
                        files.insert(desc_path, None);
                    }

                    let color_path = path.join("color");
                    if color_path.is_file() {
                        files.insert(color_path, None);
                    }

                    self.valid_addressbook_dirs.push(path);
                }

                self.state.read_files = Some(files);
                self.step = Step::ReadMoreFiles;
                Some(fs::Io::ReadFiles)
            }
            Step::ReadMoreFiles => {
                let Some(files) = &mut self.state.read_files else {
                    debug!("invalid state for step {:?}", self.step);
                    return None;
                };

                if files.values().any(Option::is_none) {
                    return Some(fs::Io::ReadFiles);
                }

                let mut addressbooks = Addressbooks::default();

                for path in self.valid_addressbook_dirs.drain(..) {
                    let id = path.file_name().unwrap().to_string_lossy().to_string();

                    let mut addressbook = Addressbook {
                        id: id.clone(),
                        name: id,
                        desc: None,
                        color: None,
                    };

                    if let Some(name) = files.remove(&path.join("displayname")).flatten() {
                        addressbook.name = String::from_utf8_lossy(&name).trim().to_string();
                    }

                    if let Some(desc) = files.remove(&path.join("description")).flatten() {
                        let desc = String::from_utf8_lossy(&desc).trim().to_string();
                        addressbook.desc.replace(desc);
                    }

                    if let Some(color) = files.remove(&path.join("color")).flatten() {
                        let color = String::from_utf8_lossy(&color).trim().to_string();
                        addressbook.color.replace(color);
                    }

                    addressbooks.push(addressbook);
                }

                self.step = Step::Done(addressbooks);
                None
            }
        }
    }
}
