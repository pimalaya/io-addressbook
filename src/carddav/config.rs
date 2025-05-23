use http::Version;
use secrecy::SecretString;

#[derive(Clone, Debug)]
pub struct Config {
    /// The CardDAV server hostname.
    pub host: String,

    /// The CardDAV server host port.
    pub port: u16,

    pub home_uri: String,

    /// The HTTP version to use when communicating with the CardDAV
    /// server.
    pub http_version: Version,

    /// The CardDAV server authentication configuration.
    ///
    /// Authentication can be done using password or OAuth 2.0.
    pub authentication: Authentication,
    // pub encryption: Encryption,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 8001,
            home_uri: String::from("/"),
            http_version: Version::default(),
            authentication: Authentication::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum Authentication {
    #[default]
    None,
    Basic(String, SecretString),
}
