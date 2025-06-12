use calcard::vcard::VCard;
use io_http::v1_1::coroutines::Send;
use io_stream::io::StreamIo;

use crate::{
    carddav::{config::CarddavConfig, request::Request},
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

    pub fn new(
        config: &CarddavConfig,
        addressbook_id: impl ToString,
        card_id: impl ToString,
    ) -> Self {
        let addressbook_id = addressbook_id.to_string();
        let card_id = card_id.to_string();
        let path = &format!("/{addressbook_id}/{card_id}.vcf");
        let request = Request::get(config, path);
        let send = Send::new(request.body(Self::BODY.as_bytes().to_vec()));

        Self {
            id: Some(card_id),
            addressbook_id: Some(addressbook_id),
            send,
        }
    }

    pub fn resume(&mut self, arg: Option<Io>) -> Result<Card, Io> {
        let (_, response) = self.send.resume(arg)?;
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
