use std::{collections::HashMap, path::PathBuf};

use crate::{Addressbook, Addressbooks};

use super::{fs, Config};

#[derive(Debug)]
pub struct ListAddressbooks {
    state: fs::State,
    valid_addressbook_dirs: Vec<PathBuf>,
    addressbooks: Option<Addressbooks>,
}

impl ListAddressbooks {
    pub fn new(config: &Config) -> Self {
        let mut state = fs::State::default();
        state.read_dir = Some((config.home_dir.clone(), None));

        Self {
            state,
            valid_addressbook_dirs: Default::default(),
            addressbooks: Default::default(),
        }
    }

    pub fn output(mut self) -> Option<Addressbooks> {
        self.addressbooks.take()
    }
}

impl AsMut<fs::State> for ListAddressbooks {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for ListAddressbooks {
    type Item = fs::Io;

    fn next(&mut self) -> Option<Self::Item> {
        println!("state: {self:?}");

        match &mut self.state {
            fs::State {
                read_dir: Some((_, None)),
                read_files: None,
            } => {
                return Some(fs::Io::ReadDir);
            }
            fs::State {
                read_dir: Some((dir, Some(paths))),
                read_files: None,
            } => {
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
                Some(fs::Io::ReadDir)
            }
            fs::State {
                read_dir: Some(_),
                read_files: Some(files),
            } if files.values().any(Option::is_none) => {
                return Some(fs::Io::ReadFiles);
            }
            fs::State {
                read_dir: Some(_),
                read_files: Some(files),
            } => {
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

                self.addressbooks.replace(addressbooks);
                None
            }
            _ => None,
        }
    }
}
