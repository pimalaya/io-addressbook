use std::path::PathBuf;

use tracing::{debug, instrument, trace};

use crate::{
    vdir::{
        fs::{self, IoState},
        Config,
    },
    Card, Cards,
};

#[derive(Debug)]
pub enum Step {
    ReadDir,
    ReadFiles,
    ParseCards,
    Done(Cards),
}

#[derive(Debug)]
pub struct ListCards {
    state: fs::State,
    next_step: Step,
    addressbook_path: PathBuf,
}

impl ListCards {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>) -> Self {
        let addressbook_path = config.home_dir.join(addressbook_id.as_ref());

        Self {
            state: fs::State::default(),
            next_step: Step::ReadDir,
            addressbook_path,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Cards> {
        let Step::Done(cards) = self.next_step else {
            debug!(?self.next_step, "invalid step to get output");
            return None;
        };

        Some(cards)
    }
}

impl AsMut<fs::State> for ListCards {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for ListCards {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(step = ?self.next_step, state = ?self.state);

        match self.next_step {
            Step::ReadDir => {
                self.state.read_dir = IoState::Pending(self.addressbook_path.clone());
                self.next_step = Step::ReadFiles;
                Some(fs::Io::ReadDir)
            }
            Step::ReadFiles => {
                let IoState::Done(paths) = &mut self.state.read_dir else {
                    debug!("invalid state for step {:?}", self.next_step);
                    return None;
                };

                paths.retain(|path| {
                    let Some(ext) = path.extension() else {
                        return false;
                    };

                    let Some("vcf") = ext.to_str() else {
                        return false;
                    };

                    let Some(name) = path.file_name() else {
                        return false;
                    };

                    let path = self.addressbook_path.join(name);

                    if !path.is_file() {
                        return false;
                    }

                    true
                });

                self.state.read_files = IoState::Pending(paths.drain(..).collect());
                self.next_step = Step::ParseCards;
                Some(fs::Io::ReadFiles)
            }
            Step::ParseCards => {
                let IoState::Done(contents) = &mut self.state.read_files else {
                    debug!("invalid state for step {:?}", self.next_step);
                    return None;
                };

                let mut cards = Cards::default();

                for (path, content) in contents {
                    let id = path.file_stem().unwrap().to_string_lossy().to_string();
                    let content = String::from_utf8_lossy(content);

                    let Some(card) = Card::parse(id, content) else {
                        continue;
                    };

                    cards.push(card);
                }

                self.next_step = Step::Done(cards);
                None
            }
            Step::Done(_) => None,
        }
    }
}
