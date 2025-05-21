use secrecy::{ExposeSecret, SecretString};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    /// The CardDAV server hostname.
    pub hostname: String,

    /// The CardDAV server host port.
    pub port: u16,

    pub home_uri: String,

    /// The HTTP version to use when communicating with the CardDAV
    /// server.
    ///
    /// Supported versions: 1.0, 1.1
    pub http_version: HttpVersion,

    /// The CardDAV server authentication configuration.
    ///
    /// Authentication can be done using password or OAuth 2.0.
    pub authentication: Authentication,
    // pub encryption: Encryption,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hostname: String::from("localhost"),
            port: 8001,
            home_uri: String::from("/"),
            http_version: HttpVersion::default(),
            authentication: Authentication::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum HttpVersion {
    V1_0,
    #[default]
    V1_1,
}

impl AsRef<str> for HttpVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::V1_0 => "1.0",
            Self::V1_1 => "1.1",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum Authentication {
    #[default]
    None,
    Basic(String, SecretString),
}

impl Eq for Authentication {}

impl PartialEq for Authentication {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::None, Self::None) => true,
            (Self::Basic(user1, pass1), Self::Basic(user2, pass2)) => {
                user1 == user2 && pass1.expose_secret() == pass2.expose_secret()
            }
            _ => false,
        }
    }
}
