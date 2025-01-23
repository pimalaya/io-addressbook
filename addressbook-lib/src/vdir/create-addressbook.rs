use std::collections::HashMap;

use tracing::{debug, instrument, trace};

use crate::{vdir::fs::CreateFileState, Addressbook};

use super::{fs, Config};

#[derive(Debug, Default)]
pub enum Step {
    #[default]
    CreateDir,
    CreateFiles,
    CreateMoreFiles,
    Done,
}

#[derive(Debug, Default)]
pub struct CreateAddressbook {
    state: fs::State,
    step: Step,
    addressbook: Addressbook,
}

impl CreateAddressbook {
    pub fn new(config: &Config, addressbook: Addressbook) -> Self {
        let mut state = fs::State::default();
        let addressbook_path = config.home_dir.join(&addressbook.id);
        state.create_dir = Some((addressbook_path, false));

        Self {
            state,
            step: Step::default(),
            addressbook,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Addressbook> {
        let Step::Done = self.step else {
            debug!(step = ?self.step, "invalid step to get output");
            return None;
        };

        Some(self.addressbook)
    }
}

impl AsMut<fs::State> for CreateAddressbook {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for CreateAddressbook {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(step = ?self.step, state = ?self.state);

        match self.step {
            Step::Done => None,
            Step::CreateDir => {
                self.step = Step::CreateFiles;
                Some(fs::Io::CreateDir)
            }
            Step::CreateFiles => {
                let Some((path, true)) = &self.state.create_dir else {
                    debug!("invalid state for step {:?}", self.step);
                    return None;
                };

                let mut files = HashMap::default();

                let name = self.addressbook.name.as_bytes().to_vec();
                files.insert(path.join("displayname"), CreateFileState::Write(name));

                if let Some(desc) = self.addressbook.desc.as_ref() {
                    let desc = CreateFileState::Write(desc.as_bytes().to_vec());
                    files.insert(path.join("description"), desc);
                }

                if let Some(color) = self.addressbook.color.as_ref() {
                    let color = CreateFileState::Write(color.as_bytes().to_vec());
                    files.insert(path.join("color"), color);
                }

                self.state.create_files = Some(files);
                self.step = Step::CreateMoreFiles;
                Some(fs::Io::CreateFiles)
            }
            Step::CreateMoreFiles => {
                let Some(files) = &mut self.state.create_files else {
                    debug!("invalid state for step {:?}", self.step);
                    return None;
                };

                if files.values().any(CreateFileState::needs_write) {
                    return Some(fs::Io::CreateFiles);
                }

                self.step = Step::Done;
                None
            }
        }
    }
}
