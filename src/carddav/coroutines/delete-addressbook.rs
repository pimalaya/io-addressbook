use io_stream::Io;
use serde::Deserialize;

use crate::carddav::{response::StatusResponse, Config, Request};

use super::Send;

#[derive(Debug)]
pub struct DeleteAddressbook(Send<Response>);

impl DeleteAddressbook {
    const BODY: &'static str = "";

    pub fn new(config: &Config, id: impl AsRef<str>) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}", id.as_ref());
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
