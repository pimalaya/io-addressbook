//! # Account configuration
//!
//! Module dedicated to account configuration.

use serde::{Deserialize, Serialize};

/// The account configuration.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TomlAccountConfig {
    /// The defaultness of the current account.
    pub default: Option<bool>,
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

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum HttpVersion {
    #[serde(rename = "1.0")]
    Http1_0,
    #[default]
    #[serde(rename = "1.1")]
    Http1_1,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CardDavConfig {
    /// The CardDAV server host name.
    #[serde(alias = "host")]
    pub hostname: String,

    /// The CardDAV server host port.
    pub port: u16,

    /// The HTTP version used to communicate with the CardDAV server.
    ///
    /// Supported versions: 1.0, 1.1
    pub http_version: Option<HttpVersion>,

    /// The CardDAV server authentication configuration.
    ///
    /// Authentication can be done using password or OAuth 2.0.
    /// See [CardDavAuthConfig].
    pub auth: Option<CardDavAuthConfig>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum CardDavAuthConfig {
    #[default]
    None,
    Basic(BasicAuthenticationConfig),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub struct BasicAuthenticationConfig {
    #[serde(alias = "user", alias = "login")]
    pub username: String,
    #[serde(alias = "pass")]
    pub password: String,
}
