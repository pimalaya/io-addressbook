use io_stream::io::StreamIo;

use crate::carddav::{config::CarddavConfig, request::Request};

use super::send::{Empty, Send, SendResult};

#[derive(Debug)]
pub struct DeleteCard(Send<Empty>);

impl DeleteCard {
    const BODY: &'static str = "";

    pub fn new(
        config: &CarddavConfig,
        addressbook_id: impl AsRef<str>,
        card_id: impl AsRef<str>,
    ) -> Self {
        let addressbook_id = addressbook_id.as_ref();
        let card_id = card_id.as_ref();
        let path = &format!("/{addressbook_id}/{card_id}.vcf");
        let request = Request::delete(config, path).content_type_xml();
        Self(Send::new(request, Self::BODY.as_bytes().to_vec()))
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<Empty> {
        self.0.resume(arg)
    }
}
