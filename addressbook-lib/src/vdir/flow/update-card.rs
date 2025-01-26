use std::{collections::HashMap, env::temp_dir, path::PathBuf};

use tracing::{debug, instrument, trace};
use uuid::Uuid;

use crate::{
    vdir::{
        fs::{self, state::Task},
        Config, VCF,
    },
    Card,
};

#[derive(Debug)]
pub enum Step {
    CreateFiles,
    MoveFiles,
    Done,
}

#[derive(Debug)]
pub struct UpdateCard {
    state: fs::State,
    next_step: Step,
    card_path: PathBuf,
    card: Card,
}

impl UpdateCard {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card: Card) -> Self {
        let card_path = config
            .home_dir
            .join(addressbook_id.as_ref())
            .join(&card.id)
            .with_extension(VCF);

        Self {
            state: fs::State::default(),
            next_step: Step::CreateFiles,
            card_path,
            card,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Card> {
        let Step::Done = self.next_step else {
            debug!("invalid step to get output");
            return None;
        };

        if !self.state.move_files.is_done() {
            debug!(state = ?self.state, "invalid state to get output");
            return None;
        }

        Some(self.card)
    }
}

impl AsMut<fs::State> for UpdateCard {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for UpdateCard {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(step = ?self.next_step, state = ?self.state);

        match self.next_step {
            Step::CreateFiles => {
                let content = self.card.to_string().as_bytes().to_vec();
                let path = temp_dir().join(Uuid::new_v4().to_string());

                let create_files = HashMap::from_iter(Some((path.clone(), content)));
                self.state.create_files = Task::Pending(create_files);

                let move_files = HashMap::from_iter(Some((path, self.card_path.clone())));
                self.state.move_files = Task::Pending(move_files);

                self.next_step = Step::MoveFiles;
                Some(fs::Io::CreateFiles)
            }
            Step::MoveFiles => {
                if !self.state.create_files.is_done() {
                    debug!("invalid state for step {:?}", self.next_step);
                    return None;
                };

                self.next_step = Step::Done;
                Some(fs::Io::MoveFiles)
            }
            Step::Done => None,
        }
    }
}
