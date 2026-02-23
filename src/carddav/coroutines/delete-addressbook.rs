use io_stream::io::StreamIo;

use crate::carddav::{config::CarddavConfig, request::Request};

use super::send::{Empty, Send, SendResult};

#[derive(Debug)]
pub struct DeleteAddressbook(Send<Empty>);

impl DeleteAddressbook {
    const BODY: &'static str = "";

    pub fn new(config: &CarddavConfig, id: impl AsRef<str>) -> Self {
        let request = Request::delete(config, id).content_type_xml();
        Self(Send::new(request, Self::BODY.as_bytes().to_vec()))
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<Empty> {
        self.0.resume(arg)
    }
}
