use crate::{
    http::{Request, SendHttpRequest},
    tcp::{Flow, Io, Read, Write},
};

use super::{client::Authentication, Config};

#[derive(Debug)]
pub struct DeleteCard {
    http: SendHttpRequest,
}

impl DeleteCard {
    const BODY: &'static str = "";

    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card_id: impl AsRef<str>) -> Self {
        let base_uri = config.addressbook_home_set_uri.trim_end_matches('/');
        let uri = &format!(
            "{base_uri}/{}/{}.vcf",
            addressbook_id.as_ref(),
            card_id.as_ref()
        );
        let mut request = Request::delete(uri.as_ref(), config.http_version.as_ref());

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            http: SendHttpRequest::new(request.body(Self::BODY)),
        }
    }
}

impl Flow for DeleteCard {}

impl Write for DeleteCard {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for DeleteCard {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for DeleteCard {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
