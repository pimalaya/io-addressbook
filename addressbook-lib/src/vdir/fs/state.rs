use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub(crate) read_dir: Option<(PathBuf, Option<Vec<PathBuf>>)>,
    pub(crate) read_files: Option<HashMap<PathBuf, Option<Vec<u8>>>>,
}

impl State {
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
