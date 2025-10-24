use std::borrow::Cow;

use http::Uri;
use secrecy::SecretString;

#[derive(Clone, Debug)]
pub struct CarddavConfig<'a> {
    /// The URI of the CardDAV server.
    pub uri: Cow<'a, Uri>,

    /// The authentication/authorization method used to communicate
    /// with the CardDAV server.
    pub auth: CarddavAuth<'a>,
}

#[derive(Clone, Debug, Default)]
pub enum CarddavAuth<'a> {
    /// The plain authentication method.
    #[default]
    Plain,

    /// The basic authentication method.
    Basic {
        username: Cow<'a, str>,
        password: Cow<'a, SecretString>,
    },

    /// The bearer authorization method.
    Bearer { token: Cow<'a, SecretString> },
}
