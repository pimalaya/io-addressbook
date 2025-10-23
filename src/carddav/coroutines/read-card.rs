use calcard::vcard::VCard;
use io_stream::io::StreamIo;

use crate::{
    card::Card,
    carddav::{config::CarddavConfig, request::Request},
};

use super::send::{Empty, Send, SendError, SendOk, SendResult};

#[derive(Debug)]
pub struct ReadCard {
    addressbook_id: Option<String>,
    id: Option<String>,
    send: Send<Empty>,
}

impl ReadCard {
    const BODY: &'static str = "";

    pub fn new(
        config: &CarddavConfig,
        addressbook_id: impl AsRef<str>,
        card_id: impl AsRef<str>,
    ) -> Self {
        let addressbook_id = addressbook_id.as_ref().to_owned();
        let card_id = card_id.as_ref().to_owned();
        let path = &format!("/{addressbook_id}/{card_id}.vcf");
        let request = Request::get(config, path);
        let send = Send::new(request, Self::BODY.as_bytes().to_vec());

        Self {
            id: Some(card_id),
            addressbook_id: Some(addressbook_id),
            send,
        }
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<Card> {
        let ok = match self.send.resume(arg) {
            SendResult::Ok(ok) => ok,
            SendResult::Err(err) => return SendResult::Err(err),
            SendResult::Io(io) => return SendResult::Io(io),
        };

        let content = String::from_utf8_lossy(ok.response.body());
        let vcard = match VCard::parse(content) {
            Ok(vcard) => vcard,
            Err(err) => return SendResult::Err(SendError::ParseVcardResponseBody(err)),
        };

        let card = Card {
            id: self.id.take().unwrap(),
            addressbook_id: self.addressbook_id.take().unwrap(),
            vcard,
        };

        SendResult::Ok(SendOk {
            request: ok.request,
            response: ok.response,
            keep_alive: ok.keep_alive,
            body: card,
        })
    }
}
