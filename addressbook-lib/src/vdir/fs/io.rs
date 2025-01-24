#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Io {
    CreateDir,
    ReadDir,
    RemoveDir,

    CreateFiles,
    ReadFiles,
    MoveFiles,
    RemoveFiles,
}
