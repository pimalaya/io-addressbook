use std::{collections::HashMap, path::PathBuf};

use tracing::{debug, instrument, trace};

use crate::{
    vdir::{
        fs::{self, state::Task},
        Config, VCF,
    },
    Card,
};

#[derive(Debug)]
pub struct CreateCard {
    state: fs::State,
    card_path: PathBuf,
    card: Card,
    done: bool,
}

impl CreateCard {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card: Card) -> Self {
        let card_path = config
            .home_dir
            .join(addressbook_id.as_ref())
            .join(&card.id)
            .with_extension(VCF);

        Self {
            state: fs::State::default(),
            card_path,
            card,
            done: false,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Card> {
        if !self.done {
            debug!("invalid step to get output");
            return None;
        };

        if !self.state.create_files.is_done() {
            debug!(state = ?self.state, "invalid state to get output");
            return None;
        }

        Some(self.card)
    }
}

impl AsMut<fs::State> for CreateCard {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for CreateCard {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(done = ?self.done, state = ?self.state);

        if self.done {
            None
        } else {
            let content = self.card.to_string().as_bytes().to_vec();
            let contents = HashMap::from_iter(Some((self.card_path.clone(), content)));
            self.state.create_files = Task::Pending(contents);
            self.done = true;
            Some(fs::Io::CreateFiles)
        }
    }
}
