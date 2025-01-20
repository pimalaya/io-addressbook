use serde::Deserialize;

use crate::{
    http::{Request, SendHttpRequest},
    tcp::{Flow, Io, Read, Write},
};

use super::{client::Authentication, response::StatusResponse, Config};

#[derive(Debug)]
pub struct DeleteAddressbook {
    http: SendHttpRequest,
}

impl DeleteAddressbook {
    const BODY: &str = "";

    pub fn new(config: &Config, id: impl AsRef<str>) -> Self {
        let base_uri = config.addressbook_home_set_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}", id.as_ref());

        let mut request = Request::delete(uri, config.http_version.as_ref()).content_type_xml();

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            http: SendHttpRequest::new(request.body(Self::BODY)),
        }
    }

    pub fn output(self) -> Result<bool, quick_xml::de::DeError> {
        let body = self.http.take_body();
        let response: Response = quick_xml::de::from_reader(body.as_slice())?;

        Ok(response.response.status.is_success())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub response: StatusResponse,
}

impl Flow for DeleteAddressbook {}

impl Write for DeleteAddressbook {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for DeleteAddressbook {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for DeleteAddressbook {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
