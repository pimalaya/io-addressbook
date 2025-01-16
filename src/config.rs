//! # Configuration
//!
//! Module dedicated to the main configuration of Neverest CLI.

use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

use crate::account::config::TomlAccountConfig;

/// The main configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TomlConfig {
    /// The configuration of all the accounts.
    pub accounts: HashMap<String, TomlAccountConfig>,
}

impl pimalaya_tui::terminal::config::TomlConfig for TomlConfig {
    type TomlAccountConfig = TomlAccountConfig;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn get_default_account_config(&self) -> Option<(String, Self::TomlAccountConfig)> {
        self.accounts.iter().find_map(|(name, account)| {
            if account.default {
                Some((name.to_owned(), account.clone()))
            } else {
                None
            }
        })
    }

    fn get_account_config(&self, name: &str) -> Option<(String, Self::TomlAccountConfig)> {
        self.accounts
            .get(name)
            .map(|account| (name.to_owned(), account.clone()))
    }
}

impl fmt::Display for TomlConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}
