use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

/// The main configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    /// The configuration of all the accounts.
    pub accounts: HashMap<String, AccountConfig>,
}

/// The account configuration.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct AccountConfig {
    /// The defaultness of the current account.
    #[serde(default)]
    pub default: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CardDavConfig {
    /// The CardDAV server hostname.
    #[serde(alias = "host")]
    pub hostname: String,

    /// The CardDAV server host port.
    pub port: u16,

    /// The addressbooks root url.
    ///
    /// Also known as the addressbook home set, it represents the
    /// common base URL to all addressbooks registered on the CardDAV
    /// server by the user being authenticated in this account.
    ///
    /// See [`CardDavConfig::auth`].
    pub url: String,

    /// The HTTP version to use when communicating with the CardDAV
    /// server.
    ///
    /// Supported versions: 1.0, 1.1
    #[serde(default)]
    pub http_version: HttpVersion,

    /// The CardDAV server authentication configuration.
    ///
    /// Authentication can be done using password or OAuth 2.0.
    #[serde(default, alias = "auth")]
    pub authentication: Authentication,

    /// The CardDAV server authentication configuration.
    ///
    /// Authentication can be done using password or OAuth 2.0.
    #[serde(default)]
    pub encryption: Encryption,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum HttpVersion {
    #[serde(rename = "1.0")]
    Http1_0,
    #[default]
    #[serde(rename = "1.1")]
    Http1_1,
}

impl AsRef<str> for HttpVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::Http1_0 => "1.0",
            Self::Http1_1 => "1.1",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Authentication {
    #[default]
    None,
    Basic(BasicAuthenticationConfig),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Encryption {
    None,
    Rustls(Rustls),
}

impl Default for Encryption {
    fn default() -> Self {
        Self::Rustls(Rustls::default())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rustls {
    #[serde(default)]
    provider: RustlsProvider,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RustlsProvider {
    #[default]
    AwsLc,
    Ring,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BasicAuthenticationConfig {
    #[serde(alias = "user", alias = "login")]
    pub username: String,
    #[serde(alias = "pass")]
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Addressbook {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Addressbooks(Vec<Addressbook>);

impl fmt::Display for Addressbooks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Deref for Addressbooks {
    type Target = Vec<Addressbook>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Addressbooks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Card {
    pub id: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Cards(Vec<Card>);

impl fmt::Display for Cards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Deref for Cards {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
