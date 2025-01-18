use quick_xml::DeError as Error;

use crate::{
    carddav::serde::{AddressbookProp, Multistatus},
    http::sans_io::{Request, SendReceiveFlow},
    tcp::sans_io::{Flow, Io, Read, Write},
};

#[derive(Debug)]
pub struct UpdateAddressbookFlow {
    http: SendReceiveFlow,
}

impl UpdateAddressbookFlow {
    pub fn new(
        uri: impl AsRef<str>,
        version: impl AsRef<str>,
        user: impl AsRef<str>,
        pass: impl AsRef<str>,
        name: Option<impl AsRef<str>>,
        color: Option<impl AsRef<str>>,
        desc: Option<impl AsRef<str>>,
    ) -> Self {
        let name = match name {
            Some(name) => format!("<displayname>{}</displayname>", name.as_ref()),
            None => String::new(),
        };

        let color = match color {
            Some(color) => format!(
                "<I:addressbook-color>{}</I:addressbook-color>",
                color.as_ref()
            ),
            None => String::new(),
        };

        let desc = match desc {
            Some(desc) => format!(
                "<C:addressbook-description>{}</C:addressbook-description>",
                desc.as_ref()
            ),
            None => String::new(),
        };

        let request = Request::proppatch(uri.as_ref(), version.as_ref())
            .basic_auth(user.as_ref(), pass.as_ref())
            .body(&format!(
                include_str!("./flow-addressbook-update.xml"),
                name, color, desc,
            ));

        Self {
            http: SendReceiveFlow::new(request),
        }
    }

    pub fn output(self) -> Result<Multistatus<AddressbookProp>, Error> {
        quick_xml::de::from_reader(self.http.take_body().as_slice())
    }
}

impl Flow for UpdateAddressbookFlow {}

impl Write for UpdateAddressbookFlow {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for UpdateAddressbookFlow {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for UpdateAddressbookFlow {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
