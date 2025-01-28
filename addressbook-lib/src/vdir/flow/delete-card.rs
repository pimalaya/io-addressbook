use std::path::PathBuf;

use tracing::{debug, instrument, trace};

use crate::vdir::{
    fs::{self, IoState},
    Config, VCF,
};

#[derive(Debug)]
pub struct DeleteCard {
    state: fs::State,
    card_path: PathBuf,
    done: bool,
}

impl DeleteCard {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card_id: impl AsRef<str>) -> Self {
        let card_path = config
            .home_dir
            .join(addressbook_id.as_ref())
            .join(card_id.as_ref())
            .with_extension(VCF);

        Self {
            state: fs::State::default(),
            card_path,
            done: false,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<PathBuf> {
        if !self.done {
            debug!("invalid step to get output");
            return None;
        };

        if !self.state.remove_files.is_done() {
            debug!(state = ?self.state, "invalid state to get output");
            return None;
        }

        Some(self.card_path)
    }
}

impl AsMut<fs::State> for DeleteCard {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for DeleteCard {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(done = ?self.done, state = ?self.state);

        if self.done {
            None
        } else {
            self.state.remove_files = IoState::Pending(vec![self.card_path.clone()]);
            self.done = true;
            Some(fs::Io::RemoveFiles)
        }
    }
}
