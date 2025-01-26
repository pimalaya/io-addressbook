use std::string::FromUtf8Error;

use thiserror::Error;

use crate::{
    carddav::{
        http::{Request, SendHttpRequest},
        tcp,
    },
    Card,
};

use super::{client::Authentication, Config};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DecodeUtf8Error(#[from] FromUtf8Error),
    #[error("cannot parse vCard")]
    ParseError,
}

#[derive(Debug)]
pub struct ReadCard {
    id: String,
    http: SendHttpRequest,
}

impl ReadCard {
    const BODY: &'static str = "";

    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card_id: impl ToString) -> Self {
        let addressbook_id = addressbook_id.as_ref();
        let card_id = card_id.to_string();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}/{card_id}.vcf");

        let mut request = Request::get(uri.as_ref(), config.http_version.as_ref());

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            id: card_id,
            http: SendHttpRequest::new(request.body(Self::BODY)),
        }
    }

    pub fn output(self) -> Result<Card, Error> {
        let content = String::from_utf8(self.http.take_body())?;
        Card::parse(self.id, content).ok_or(Error::ParseError)
    }
}

impl AsMut<tcp::State> for ReadCard {
    fn as_mut(&mut self) -> &mut tcp::State {
        self.http.as_mut()
    }
}

impl Iterator for ReadCard {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
