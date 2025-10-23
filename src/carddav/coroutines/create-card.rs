use io_stream::io::StreamIo;

use crate::{
    card::Card,
    carddav::{config::CarddavConfig, request::Request},
};

use super::send::{Empty, Send, SendResult};

#[derive(Debug)]
pub struct CreateCard(Send<Empty>);

impl CreateCard {
    pub fn new(config: &CarddavConfig, card: Card) -> Self {
        let path = format!("/{}/{}.vcf", card.addressbook_id, card.id);
        let request = Request::put(config, path).content_type_vcard();
        let body = card.to_string().into_bytes();
        Self(Send::new(request, body))
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<Empty> {
        self.0.resume(arg)
    }
}
