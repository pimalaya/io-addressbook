use std::{collections::HashMap, env::temp_dir, mem, path::PathBuf};

use tracing::{debug, instrument, trace};
use uuid::Uuid;

use crate::{
    vdir::{
        fs::{self, IoState},
        Config, COLOR, DESCRIPTION, DISPLAYNAME,
    },
    PartialAddressbook,
};

#[derive(Debug)]
pub enum Step {
    CreateFiles,
    MoveFiles,
    Done,
}

#[derive(Debug)]
pub struct UpdateAddressbook {
    state: fs::State,
    next_step: Step,
    addressbook_path: PathBuf,
    addressbook: PartialAddressbook,
    move_files: HashMap<PathBuf, PathBuf>,
}

impl UpdateAddressbook {
    pub fn new(config: &Config, addressbook: PartialAddressbook) -> Self {
        let addressbook_path = config.home_dir.join(&addressbook.id);

        Self {
            state: fs::State::default(),
            next_step: Step::CreateFiles,
            addressbook_path,
            addressbook,
            move_files: HashMap::new(),
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<PartialAddressbook> {
        let Step::Done = self.next_step else {
            debug!(step = ?self.next_step, "invalid step to get output");
            return None;
        };

        Some(self.addressbook)
    }
}

impl AsMut<fs::State> for UpdateAddressbook {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for UpdateAddressbook {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(step = ?self.next_step, state = ?self.state);

        match self.next_step {
            Step::CreateFiles => {
                let tmp = temp_dir();
                let mut create_files = HashMap::new();

                if let Some(name) = &self.addressbook.name {
                    let src = tmp.join(Uuid::new_v4().to_string());
                    let dest = self.addressbook_path.join(DISPLAYNAME);
                    create_files.insert(src.clone(), name.as_bytes().to_vec());
                    self.move_files.insert(src, dest);
                }

                if let Some(desc) = &self.addressbook.desc {
                    let src = tmp.join(Uuid::new_v4().to_string());
                    let dest = self.addressbook_path.join(DESCRIPTION);
                    create_files.insert(src.clone(), desc.as_bytes().to_vec());
                    self.move_files.insert(src, dest);
                }

                if let Some(color) = &self.addressbook.color {
                    let src = tmp.join(Uuid::new_v4().to_string());
                    let dest = self.addressbook_path.join(COLOR);
                    create_files.insert(src.clone(), color.as_bytes().to_vec());
                    self.move_files.insert(src, dest);
                }

                self.state.create_files = IoState::Pending(create_files);
                self.next_step = Step::MoveFiles;
                Some(fs::Io::CreateFiles)
            }
            Step::MoveFiles => {
                if !self.state.create_files.is_done() {
                    debug!("invalid state for step {:?}", self.next_step);
                    return None;
                };

                let mut move_files = HashMap::new();
                mem::swap(&mut move_files, &mut self.move_files);

                self.state.move_files = IoState::Pending(move_files);
                self.next_step = Step::Done;
                Some(fs::Io::MoveFiles)
            }
            Step::Done => None,
        }
    }
}
