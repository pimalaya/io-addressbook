use std::path::PathBuf;

use log::{debug, instrument, trace};

use crate::{
    vdir::{
        fs::{self, IoState},
        Config, VCF,
    },
    Card,
};

#[derive(Debug)]
pub struct ReadCard {
    state: fs::State,
    card_id: String,
    card_path: PathBuf,
    done: bool,
}

impl ReadCard {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card_id: impl ToString) -> Self {
        let card_id = card_id.to_string();
        let card_path = config
            .home_dir
            .join(addressbook_id.as_ref())
            .join(&card_id)
            .with_extension(VCF);

        Self {
            state: fs::State::default(),
            card_id,
            card_path,
            done: false,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<Card> {
        if !self.done {
            debug!("invalid step to get output");
            return None;
        };

        let IoState::Done(files) = self.state.read_files else {
            debug!(state = ?self.state, "invalid state to get output");
            return None;
        };

        let (_, content) = files.into_iter().next()?;
        let content = String::from_utf8(content).ok()?;
        Card::parse(self.card_id, content)
    }
}

impl AsMut<fs::State> for ReadCard {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for ReadCard {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(done = ?self.done, state = ?self.state);

        if self.done {
            None
        } else {
            let paths = vec![self.card_path.clone()];
            self.state.read_files = IoState::Pending(paths);
            self.done = true;
            Some(fs::Io::ReadFiles)
        }
    }
}
