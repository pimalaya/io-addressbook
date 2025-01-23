use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub home_dir: PathBuf,
}
