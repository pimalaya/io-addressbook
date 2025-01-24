use std::{
    fs::{create_dir, read, read_dir, remove_dir_all, remove_file, rename, write},
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
            fs::Io::ReadDir => self.read_dir(state),
            fs::Io::RemoveDir => self.remove_dir(state),
            fs::Io::CreateFiles => self.create_files(state),
            fs::Io::ReadFiles => self.read_files(state),
            fs::Io::MoveFiles => self.move_files(state),
            fs::Io::RemoveFiles => self.remove_files(state),
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
    fn remove_dir(&self, state: &mut fs::State) -> Result<()> {
        let Some(path) = state.get_remove_dir_path() else {
            let err = Error::new(ErrorKind::NotFound, "remove dir state not found");
            return Err(err);
        };

        remove_dir_all(path)?;

        state.set_remove_dir_done();
        Ok(())
    }

    #[instrument(skip_all)]
    fn create_files(&self, state: &mut fs::State) -> Result<()> {
        let Some(contents) = state.get_create_file_contents() else {
            let err = Error::new(ErrorKind::NotFound, "create files state not found");
            return Err(err);
        };

        for (path, content) in contents {
            trace!(?path, "create file");
            write(path, content)?;
        }

        state.set_create_files_done();
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

    #[instrument(skip_all)]
    fn move_files(&self, state: &mut fs::State) -> Result<()> {
        let Some(paths) = state.get_move_file_paths() else {
            let err = Error::new(ErrorKind::NotFound, "move files state not found");
            return Err(err);
        };

        for (src, dest) in paths {
            trace!(?src, ?dest, "move file");
            rename(src, dest)?;
        }

        state.set_move_files_done();
        Ok(())
    }

    #[instrument(skip_all)]
    fn remove_files(&self, state: &mut fs::State) -> Result<()> {
        let Some(paths) = state.get_remove_file_paths() else {
            let err = Error::new(ErrorKind::NotFound, "remove files state not found");
            return Err(err);
        };

        for path in paths {
            trace!(?path, "remove file");
            remove_file(path)?;
        }

        state.set_remove_files_done();
        Ok(())
    }
}
