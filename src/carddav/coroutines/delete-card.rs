use io_stream::io::StreamIo;
use serde::Deserialize;

use crate::carddav::{config::CarddavConfig, request::Request, response::StatusResponse};

use super::{Send, SendResult};

#[derive(Debug)]
pub struct DeleteCard(Send<Response>);

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
        Self(Send::new(request, Self::BODY.as_bytes()))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> SendResult<bool> {
        let body = match self.0.resume(arg) {
            SendResult::Ok(body) => body,
            SendResult::Err(err) => return SendResult::Err(err),
            SendResult::Io(io) => return SendResult::Io(io),
            SendResult::Redirect(res) => return SendResult::Redirect(res),
        };

        SendResult::Ok(body.response.status.is_success())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub response: StatusResponse,
}
