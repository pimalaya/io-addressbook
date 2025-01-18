use std::string::FromUtf8Error;

use crate::{
    http::sans_io::{Request, SendReceiveFlow},
    tcp::sans_io::{Flow, Io, Read, Write},
};

#[derive(Debug)]
pub struct UpdateCardFlow {
    http: SendReceiveFlow,
}

impl UpdateCardFlow {
    pub fn new(
        uri: impl AsRef<str>,
        version: impl AsRef<str>,
        user: impl AsRef<str>,
        pass: impl AsRef<str>,
        vcf: impl AsRef<str>,
    ) -> Self {
        let request = Request::put(uri.as_ref(), version.as_ref())
            .content_type_vcard()
            .basic_auth(user.as_ref(), pass.as_ref())
            .body(vcf.as_ref());

        Self {
            http: SendReceiveFlow::new(request),
        }
    }

    pub fn output(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.http.take_body())
    }
}

impl Flow for UpdateCardFlow {}

impl Write for UpdateCardFlow {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for UpdateCardFlow {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for UpdateCardFlow {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
