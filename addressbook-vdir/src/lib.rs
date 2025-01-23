use std::{
    fs::{create_dir, read, read_dir, write},
    io::{Error, ErrorKind, Result},
};

use addressbook::vdir::fs;
use tracing::{debug, instrument, trace};

#[derive(Debug, Default)]
pub struct Connector;

impl Connector {
    pub fn new() -> Self {
        Self::default()
    }

    #[instrument(skip_all)]
    pub fn execute<F: AsMut<fs::State>>(&mut self, flow: &mut F, io: fs::Io) -> Result<()> {
        let state = flow.as_mut();

        match io {
            fs::Io::CreateDir => self.create_dir(state),
            fs::Io::CreateFiles => self.create_files(state),
            fs::Io::ReadDir => self.read_dir(state),
            fs::Io::ReadFiles => self.read_files(state),
        }
    }

    #[instrument(skip_all)]
    fn create_dir(&self, state: &mut fs::State) -> Result<()> {
        let Some(path) = state.get_create_dir_path() else {
            let err = Error::new(ErrorKind::NotFound, "create dir state not found");
            return Err(err);
        };

        create_dir(path)?;

        state.set_create_dir_done();
        Ok(())
    }

    #[instrument(skip_all)]
    fn create_files(&self, state: &mut fs::State) -> Result<()> {
        let Some(contents) = state.get_create_file_contents() else {
            let err = Error::new(ErrorKind::NotFound, "create files state not found");
            return Err(err);
        };

        let mut created_paths = Vec::new();

        for (path, content) in contents {
            trace!(?path, "create file");
            write(path, content)?;
            created_paths.push(path.to_owned());
        }

        state.set_create_files_done(created_paths);
        Ok(())
    }

    #[instrument(skip_all)]
    fn read_dir(&self, state: &mut fs::State) -> Result<()> {
        let Some(dir) = state.get_read_dir_path() else {
            let err = Error::new(ErrorKind::NotFound, "read dir state not found");
            return Err(err);
        };

        let mut paths = Vec::new();

        for entry in read_dir(dir)? {
            trace!(?entry, "read directory");

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

    #[instrument(skip_all)]
    fn read_files(&self, state: &mut fs::State) -> Result<()> {
        let Some(paths) = state.get_read_file_paths() else {
            let err = Error::new(ErrorKind::NotFound, "read file state not found");
            return Err(err);
        };

        let mut contents = Vec::new();

        for path in paths {
            trace!(?path, "read file");
            let content = read(path)?;
            contents.push((path.to_owned(), content));
        }

        state.set_read_file_contents(contents);
        Ok(())
    }
}
