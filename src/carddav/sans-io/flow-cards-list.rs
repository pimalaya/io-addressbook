use quick_xml::DeError as Error;

use crate::{
    carddav::serde::{AddressDataProp, Multistatus},
    http::sans_io::{Request, SendReceiveFlow},
    tcp::sans_io::{Flow, Io, Read, Write},
};

#[derive(Debug)]
pub struct ListCardsFlow {
    http: SendReceiveFlow,
}

impl ListCardsFlow {
    const BODY: &str = r#"
        <c:addressbook-query xmlns="DAV:" xmlns:c="urn:ietf:params:xml:ns:carddav">
            <prop>
                <getetag />
            </prop>
        </c:addressbook-query>
    "#;

    pub fn new(
        uri: impl AsRef<str>,
        version: impl AsRef<str>,
        user: impl AsRef<str>,
        pass: impl AsRef<str>,
    ) -> Self {
        let request = Request::report(uri.as_ref(), version.as_ref())
            .basic_auth(user.as_ref(), pass.as_ref())
            .depth("1")
            .body(Self::BODY);

        Self {
            http: SendReceiveFlow::new(request),
        }
    }

    pub fn output(self) -> Result<Multistatus<AddressDataProp>, Error> {
        quick_xml::de::from_reader(self.http.take_body().as_slice())
    }
}

impl Flow for ListCardsFlow {}

impl Write for ListCardsFlow {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for ListCardsFlow {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for ListCardsFlow {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
