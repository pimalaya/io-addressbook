//! # Account configuration
//!
//! Module dedicated to account configuration.

use serde::{Deserialize, Serialize};

use crate::contact::{AccountConfig, CardDavConfig};

/// The account configuration.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TomlAccountConfig {
    /// The defaultness of the current account.
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub backend: Backend,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Backend {
    #[default]
    None,
    CardDav(CardDavConfig),
}

impl Into<(AccountConfig, Backend)> for TomlAccountConfig {
    fn into(self) -> (AccountConfig, Backend) {
        let TomlAccountConfig { default, backend } = self;
        let config = AccountConfig { default };
        (config, backend)
    }
}
