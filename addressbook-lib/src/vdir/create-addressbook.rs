use std::{collections::HashMap, path::PathBuf};

use tracing::{debug, instrument, trace};

use crate::{
    vdir::{fs::state::Task, COLOR, DESCRIPTION, DISPLAYNAME},
    Addressbook,
};

use super::{fs, Config};

#[derive(Debug)]
pub enum Step {
    CreateDir,
    CreateFiles,
    Done,
}

#[derive(Debug)]
pub struct CreateAddressbook {
    state: fs::State,
    next_step: Step,
    addressbook_path: PathBuf,
    addressbook: Addressbook,
}

impl CreateAddressbook {
    pub fn new(config: &Config, addressbook: Addressbook) -> Self {
        let addressbook_path = config.home_dir.join(&addressbook.id);

        Self {
            state: fs::State::default(),
            next_step: Step::CreateDir,
            addressbook_path,
            addressbook,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Addressbook> {
        let Step::Done = self.next_step else {
            debug!(step = ?self.next_step, "invalid step to get output");
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
        trace!(step = ?self.next_step, state = ?self.state);

        match self.next_step {
            Step::CreateDir => {
                self.state.create_dir = Task::Pending(self.addressbook_path.clone());
                self.next_step = Step::CreateFiles;
                Some(fs::Io::CreateDir)
            }
            Step::CreateFiles => {
                if !self.state.create_dir.is_done() {
                    debug!("invalid state for step {:?}", self.next_step);
                    return None;
                };

                let mut contents = HashMap::new();

                let path = self.addressbook_path.join(DISPLAYNAME);
                let content = self.addressbook.name.as_bytes().to_vec();
                contents.insert(path, content);

                if let Some(desc) = self.addressbook.desc.as_ref() {
                    let path = self.addressbook_path.join(DESCRIPTION);
                    let content = desc.as_bytes().to_vec();
                    contents.insert(path, content);
                }

                if let Some(color) = self.addressbook.color.as_ref() {
                    let path = self.addressbook_path.join(COLOR);
                    let content = color.as_bytes().to_vec();
                    contents.insert(path, content);
                }

                self.state.create_files = Task::Pending(contents);
                self.next_step = Step::Done;
                Some(fs::Io::CreateFiles)
            }
            Step::Done => None,
        }
    }
}
