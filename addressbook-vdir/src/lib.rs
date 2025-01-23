use std::{
    fs::{read, read_dir},
    io::{Error, ErrorKind, Result},
};

use addressbook::vdir::fs;
use tracing::{debug, trace};

#[derive(Debug, Default)]
pub struct Connector;

impl Connector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute<F: AsMut<fs::State>>(&mut self, flow: &mut F, io: fs::Io) -> Result<()> {
        let state = flow.as_mut();

        match io {
            fs::Io::ReadDir => self.read_dir(state),
            fs::Io::ReadFiles => self.read_files(state),
        }
    }

    fn read_dir(&self, state: &mut fs::State) -> Result<()> {
        let Some(dir) = state.get_read_dir_path() else {
            let err = Error::new(ErrorKind::NotFound, "read dir state not found");
            return Err(err);
        };

        let mut paths = Vec::new();

        for entry in read_dir(dir)? {
            trace!(?entry, "process directory entry");

            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    debug!(?err, "ignore invalid directory entry: {err}");
                    continue;
                }
            };

            paths.push(entry.path())
        }

        state.set_read_dir_entry_paths(paths);
        Ok(())
    }

    fn read_files(&self, state: &mut fs::State) -> Result<()> {
        let Some(paths) = state.get_read_file_paths() else {
            let err = Error::new(ErrorKind::NotFound, "read file state not found");
            return Err(err);
        };

        let mut contents = vec![];

        for path in paths {
            let content = read(path)?;
            contents.push((path.to_owned(), content));
        }

        state.set_read_file_contents(contents);
        Ok(())
    }
}
