#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Io {
    CreateDir,
    CreateFiles,
    ReadDir,
    ReadFiles,
}
