use secrecy::SecretString;

use super::CurrentUserPrincipal;

#[derive(Clone, Debug, Default)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_basic_authentication(&mut self, user: impl ToString, pass: impl Into<SecretString>) {
        self.config.authentication = Authentication::Basic(user.to_string(), pass.into());
    }

    pub fn with_basic_authentication(
        mut self,
        user: impl ToString,
        pass: impl Into<SecretString>,
    ) -> Self {
        self.set_basic_authentication(user, pass);
        self
    }

    pub fn current_user_principal(&self) -> CurrentUserPrincipal {
        CurrentUserPrincipal::new(&self.config)
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    /// The CardDAV server hostname.
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
    pub root_url: String,

    /// The HTTP version to use when communicating with the CardDAV
    /// server.
    ///
    /// Supported versions: 1.0, 1.1
    pub http_version: HttpVersion,

    /// The CardDAV server authentication configuration.
    ///
    /// Authentication can be done using password or OAuth 2.0.
    pub authentication: Authentication,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hostname: String::from("localhost"),
            port: 8001,
            root_url: String::from("/"),
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
