use std::path::PathBuf;

use super::{Config, ListAddressbooks};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new(home_dir: impl Into<PathBuf>) -> Self {
        Self {
            config: Config {
                home_dir: home_dir.into(),
            },
        }
    }

    #[cfg(debug_assertions)]
    pub fn new_from_envs() -> Self {
        let home_dir = std::env::var("DIR").expect("missing DIR env var");
        Self::new(home_dir)
    }

    pub fn list_addressbooks(&self) -> ListAddressbooks {
        ListAddressbooks::new(&self.config)
    }
}
