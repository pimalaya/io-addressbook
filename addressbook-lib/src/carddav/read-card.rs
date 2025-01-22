use std::string::FromUtf8Error;

use thiserror::Error;

use crate::{
    http::{Request, SendHttpRequest},
    tcp::{Flow, Io, Read, Write},
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

impl Flow for ReadCard {}

impl Write for ReadCard {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for ReadCard {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for ReadCard {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
