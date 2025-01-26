use serde::Deserialize;

use crate::carddav::{
    http::{Request, SendHttpRequest},
    tcp,
};

use super::{client::Authentication, response::StatusResponse, Config};

#[derive(Debug)]
pub struct DeleteAddressbook {
    http: SendHttpRequest,
}

impl DeleteAddressbook {
    const BODY: &str = "";

    pub fn new(config: &Config, id: impl AsRef<str>) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
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

impl AsMut<tcp::State> for DeleteAddressbook {
    fn as_mut(&mut self) -> &mut tcp::State {
        self.http.as_mut()
    }
}

impl Iterator for DeleteAddressbook {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
