use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum CreateFileState {
    Write(Vec<u8>),
    Done,
}

impl CreateFileState {
    pub fn needs_write(&self) -> bool {
        match self {
            Self::Write(_) => true,
            Self::Done => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub(crate) create_dir: Option<(PathBuf, bool)>,
    pub(crate) create_files: Option<HashMap<PathBuf, CreateFileState>>,
    pub(crate) read_dir: Option<(PathBuf, Option<Vec<PathBuf>>)>,
    pub(crate) read_files: Option<HashMap<PathBuf, Option<Vec<u8>>>>,
}

impl State {
    pub fn get_create_dir_path(&self) -> Option<&Path> {
        Some(self.create_dir.as_ref()?.0.as_ref())
    }

    pub fn set_create_dir_done(&mut self) -> Option<()> {
        self.create_dir.as_mut()?.1 = true;
        Some(())
    }

    pub fn get_create_file_contents(&self) -> Option<impl Iterator<Item = (&Path, &[u8])>> {
        let contents = self
            .create_files
            .as_ref()?
            .iter()
            .filter_map(|(path, state)| match state {
                CreateFileState::Write(content) => Some((path.as_ref(), content.as_ref())),
                CreateFileState::Done => None,
            });

        Some(contents)
    }

    pub fn set_create_file_done(&mut self, path: impl Into<PathBuf>) -> Option<()> {
        let files = self.create_files.as_mut()?;
        files.insert(path.into(), CreateFileState::Done);
        Some(())
    }

    pub fn set_create_files_done(
        &mut self,
        paths: impl IntoIterator<Item = impl Into<PathBuf>>,
    ) -> Option<()> {
        let prev_files = self.create_files.as_mut()?;
        let next_files = paths
            .into_iter()
            .map(|path| (path.into(), CreateFileState::Done));
        prev_files.extend(next_files);
        Some(())
    }

    pub fn get_read_dir_path(&self) -> Option<&Path> {
        Some(self.read_dir.as_ref()?.0.as_ref())
    }

    pub fn set_read_dir_entry_paths(
        &mut self,
        paths: impl IntoIterator<Item = impl Into<PathBuf>>,
    ) -> Option<()> {
        let paths = paths.into_iter().map(Into::into);
        self.read_dir.as_mut()?.1.replace(paths.collect());
        Some(())
    }

    pub fn get_read_file_paths(&self) -> Option<impl Iterator<Item = &Path>> {
        Some(self.read_files.as_ref()?.keys().map(PathBuf::as_path))
    }

    pub fn set_read_file_content(
        &mut self,
        path: impl Into<PathBuf>,
        content: impl IntoIterator<Item = u8>,
    ) -> Option<()> {
        let files = self.read_files.as_mut()?;
        files.insert(path.into(), Some(content.into_iter().collect()));
        Some(())
    }

    pub fn set_read_file_contents(
        &mut self,
        files: impl IntoIterator<Item = (impl Into<PathBuf>, impl IntoIterator<Item = u8>)>,
    ) -> Option<()> {
        let prev_files = self.read_files.as_mut()?;
        let next_files = files
            .into_iter()
            .map(|(path, content)| (path.into(), Some(content.into_iter().collect())));
        prev_files.extend(next_files);
        Some(())
    }
}
