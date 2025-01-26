use std::path::PathBuf;

use tracing::{debug, instrument, trace};

use crate::vdir::{
    fs::{self, state::Task},
    Config,
};

#[derive(Debug)]
pub struct DeleteAddressbook {
    state: fs::State,
    addressbook_path: PathBuf,
    done: bool,
}

impl DeleteAddressbook {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>) -> Self {
        let addressbook_path = config.home_dir.join(addressbook_id.as_ref());

        Self {
            state: fs::State::default(),
            addressbook_path,
            done: false,
        }
    }

    #[instrument(skip_all)]
    pub fn output(self) -> Option<()> {
        if !self.done {
            debug!("invalid step to get output");
            return None;
        };

        Some(())
    }
}

impl AsMut<fs::State> for DeleteAddressbook {
    fn as_mut(&mut self) -> &mut fs::State {
        &mut self.state
    }
}

impl Iterator for DeleteAddressbook {
    type Item = fs::Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        trace!(done = ?self.done, state = ?self.state);

        if self.done {
            None
        } else {
            self.state.remove_dir = Task::Pending(self.addressbook_path.clone());
            self.done = true;
            Some(fs::Io::RemoveDir)
        }
    }
}
