use std::{mem, path::PathBuf};

use tracing::{debug, instrument, trace};

use crate::{
    vdir::{
        fs::{self, IoState},
        Config, COLOR, DESCRIPTION, DISPLAYNAME,
    },
    Addressbook, Addressbooks,
};

#[derive(Debug)]
pub enum Step {
    ReadDir,
    ReadFiles,
    ReadFilesDone,
    Done(Addressbooks),
}

#[derive(Debug)]
pub struct ListAddressbooks {
    state: fs::State,
    next_step: Step,
    home_dir: PathBuf,
    valid_addressbook_dirs: Vec<PathBuf>,
}

impl ListAddressbooks {
    pub fn new(config: &Config) -> Self {
        Self {
            state: fs::State::default(),
            next_step: Step::ReadDir,
            home_dir: config.home_dir.clone(),
            valid_addressbook_dirs: Vec::new(),
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Addressbooks> {
        let Step::Done(addressbooks) = self.next_step else {
            debug!(?self.next_step, "invalid step to get output");
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
        trace!(step = ?self.next_step, state = ?self.state);

        match self.next_step {
            Step::ReadDir => {
                self.state.read_dir = IoState::Pending(self.home_dir.clone());
                self.next_step = Step::ReadFiles;
                Some(fs::Io::ReadDir)
            }
            Step::ReadFiles => {
                let IoState::Done(paths) = &mut self.state.read_dir else {
                    debug!("invalid state for step {:?}", self.next_step);
                    return None;
                };

                paths.retain(|path| {
                    let Some(name) = path.file_name() else {
                        return false;
                    };

                    let path = self.home_dir.join(&name);

                    if !path.is_dir() {
                        return false;
                    }

                    true
                });

                mem::swap(&mut self.valid_addressbook_dirs, paths);
                let mut paths = Vec::new();

                for dir in &self.valid_addressbook_dirs {
                    let name_path = dir.join(DISPLAYNAME);

                    if name_path.is_file() {
                        paths.push(name_path);
                    }

                    let desc_path = dir.join(DESCRIPTION);

                    if desc_path.is_file() {
                        paths.push(desc_path);
                    }

                    let color_path = dir.join(COLOR);

                    if color_path.is_file() {
                        paths.push(color_path);
                    }
                }

                self.state.read_files = IoState::Pending(paths);
                self.next_step = Step::ReadFilesDone;
                Some(fs::Io::ReadFiles)
            }
            Step::ReadFilesDone => {
                let IoState::Done(contents) = &mut self.state.read_files else {
                    debug!("invalid state for step {:?}", self.next_step);
                    return None;
                };

                let mut addressbooks = Addressbooks::default();

                for path in self.valid_addressbook_dirs.drain(..) {
                    let id = path.file_name().unwrap().to_string_lossy().to_string();

                    let mut addressbook = Addressbook {
                        id: id.clone(),
                        name: id,
                        desc: None,
                        color: None,
                    };

                    if let Some(name) = contents.remove(&path.join(DISPLAYNAME)) {
                        let name = String::from_utf8_lossy(&name);
                        addressbook.name = name.to_string();
                    }

                    if let Some(desc) = contents.remove(&path.join(DESCRIPTION)) {
                        let desc = String::from_utf8_lossy(&desc);

                        if desc.trim().is_empty() {
                            addressbook.desc = None
                        } else {
                            addressbook.desc.replace(desc.to_string());
                        }
                    }

                    if let Some(color) = contents.remove(&path.join(COLOR)) {
                        let color = String::from_utf8_lossy(&color);

                        if color.trim().is_empty() {
                            addressbook.color = None
                        } else {
                            addressbook.color.replace(color.to_string());
                        }
                    }

                    addressbooks.push(addressbook);
                }

                self.next_step = Step::Done(addressbooks);
                None
            }
            Step::Done(_) => None,
        }
    }
}
