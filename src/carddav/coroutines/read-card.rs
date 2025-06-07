use calcard::vcard::VCard;
use io_http::v1_1::coroutines::Send;
use io_stream::Io;

use crate::{
    carddav::{Config, Request},
    Card,
};

#[derive(Debug)]
pub struct ReadCard {
    addressbook_id: Option<String>,
    id: Option<String>,
    send: Send,
}

impl ReadCard {
    const BODY: &'static str = "";

    pub fn new(config: &Config, addressbook_id: impl ToString, card_id: impl ToString) -> Self {
        let addressbook_id = addressbook_id.to_string();
        let card_id = card_id.to_string();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}/{card_id}.vcf");

        let request = Request::get(uri.as_ref(), config.http_version)
            .host(&config.host, config.port)
            .authorization(&config.auth);

        let send = Send::new(request.body(Self::BODY.as_bytes().to_vec()));

        Self {
            id: Some(card_id),
            addressbook_id: Some(addressbook_id),
            send,
        }
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<Card, Io> {
        let response = self.send.resume(input)?;
        let content = String::from_utf8(response.into_body()).unwrap();
        let vcard = VCard::parse(content).unwrap();
        let card = Card {
            id: self.id.take().unwrap(),
            addressbook_id: self.addressbook_id.take().unwrap(),
            vcard,
        };

        Ok(card)
    }
}
