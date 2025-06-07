use io_stream::Io;
use serde::Deserialize;

use crate::carddav::{response::StatusResponse, Config, Request};

use super::Send;

#[derive(Debug)]
pub struct DeleteCard(Send<Response>);

impl DeleteCard {
    const BODY: &'static str = "";

    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card_id: impl AsRef<str>) -> Self {
        let addressbook_id = addressbook_id.as_ref();
        let card_id = card_id.as_ref();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}/{card_id}.vcf");
        let request = Request::delete(uri, config.http_version).content_type_xml();
        Self(Send::new(config, request, Self::BODY.as_bytes()))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> Result<bool, Io> {
        let body = self.0.resume(arg)?;
        Ok(body.response.status.is_success())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub response: StatusResponse,
}
