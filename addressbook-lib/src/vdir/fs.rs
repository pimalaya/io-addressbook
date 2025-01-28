use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Io {
    CreateDir,
    CreateFiles,
    ReadDir,
    ReadFiles,
    MoveFiles,
    RemoveDir,
    RemoveFiles,
}

#[derive(Debug, Default)]
pub struct State {
    pub create_dir: IoState<PathBuf, ()>,
    pub create_files: IoState<HashMap<PathBuf, Vec<u8>>, ()>,
    pub read_dir: IoState<PathBuf, Vec<PathBuf>>,
    pub read_files: IoState<Vec<PathBuf>, HashMap<PathBuf, Vec<u8>>>,
    pub move_files: IoState<HashMap<PathBuf, PathBuf>, ()>,
    pub remove_dir: IoState<PathBuf, ()>,
    pub remove_files: IoState<Vec<PathBuf>, ()>,
}

impl State {
    pub fn get_create_dir_path(&self) -> Option<&Path> {
        Some(self.create_dir.pending()?.as_ref())
    }

    pub fn set_create_dir_done(&mut self) -> Option<()> {
        self.create_dir.done()
    }

    pub fn get_read_dir_path(&self) -> Option<&Path> {
        Some(self.read_dir.pending()?.as_ref())
    }

    pub fn set_read_dir_entry_paths(
        &mut self,
        paths: impl IntoIterator<Item = impl Into<PathBuf>>,
    ) -> Option<()> {
        let output = paths.into_iter().map(Into::into).collect();
        self.read_dir.done_with(output)
    }

    pub fn get_create_file_contents(&self) -> Option<impl Iterator<Item = (&Path, &[u8])>> {
        let input = self
            .create_files
            .pending()?
            .iter()
            .map(|(path, content)| (path.as_ref(), content.as_ref()));

        Some(input)
    }

    pub fn get_remove_dir_path(&self) -> Option<&Path> {
        Some(self.remove_dir.pending()?.as_ref())
    }

    pub fn set_remove_dir_done(&mut self) -> Option<()> {
        self.remove_dir.done()
    }

    pub fn set_create_files_done(&mut self) -> Option<()> {
        self.create_files.done()
    }

    pub fn get_read_file_paths(&self) -> Option<impl Iterator<Item = &Path>> {
        Some(self.read_files.pending()?.iter().map(PathBuf::as_path))
    }

    pub fn set_read_file_contents(
        &mut self,
        files: impl IntoIterator<Item = (impl Into<PathBuf>, impl IntoIterator<Item = u8>)>,
    ) -> Option<()> {
        let output = files
            .into_iter()
            .map(|(path, content)| (path.into(), content.into_iter().collect()))
            .collect();

        self.read_files.done_with(output)
    }

    pub fn get_move_file_paths(&self) -> Option<impl Iterator<Item = (&Path, &Path)>> {
        let input = self
            .move_files
            .pending()?
            .iter()
            .map(|(from, to)| (from.as_ref(), to.as_ref()));

        Some(input)
    }

    pub fn set_move_files_done(&mut self) -> Option<()> {
        self.move_files.done()
    }

    pub fn get_remove_file_paths(&self) -> Option<impl Iterator<Item = &Path>> {
        Some(self.remove_files.pending()?.iter().map(AsRef::as_ref))
    }

    pub fn set_remove_files_done(&mut self) -> Option<()> {
        self.remove_files.done()
    }
}

#[derive(Debug, Default)]
pub enum IoState<I, O> {
    #[default]
    Idle,
    Pending(I),
    Done(O),
}

impl<I> IoState<I, ()> {
    pub fn done(&mut self) -> Option<()> {
        self.done_with(())
    }
}

impl<I, O> IoState<I, O> {
    pub fn pending(&self) -> Option<&I> {
        match self {
            IoState::Idle | IoState::Done(_) => None,
            IoState::Pending(input) => Some(input),
        }
    }

    pub fn is_done(&self) -> bool {
        match self {
            IoState::Idle | IoState::Pending(_) => false,
            IoState::Done(_) => true,
        }
    }

    pub fn done_with(&mut self, output: O) -> Option<()> {
        match self {
            IoState::Idle | IoState::Done(_) => None,
            IoState::Pending(_) => {
                *self = Self::Done(output);
                Some(())
            }
        }
    }
}
