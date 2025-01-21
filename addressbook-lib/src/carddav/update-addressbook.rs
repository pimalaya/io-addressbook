use serde::Deserialize;
use tracing::debug;

use crate::{
    carddav::response::MkcolResponse,
    http::{Request, SendHttpRequest},
    tcp::{Flow, Io, Read, Write},
    Addressbook,
};

use super::{client::Authentication, Config};

#[derive(Debug)]
pub struct UpdateAddressbook {
    addressbook: Addressbook,
    http: SendHttpRequest,
}

impl UpdateAddressbook {
    pub fn new(config: &Config, addressbook: Addressbook) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}", addressbook.id);

        let color = match &addressbook.color {
            Some(color) => format!("<I:addressbook-color>{color}</I:addressbook-color>"),
            None => String::new(),
        };

        let desc = match &addressbook.desc {
            Some(desc) => format!("<C:addressbook-description>{desc}</C:addressbook-description>"),
            None => String::new(),
        };

        let mut request = Request::proppatch(uri, config.http_version.as_ref()).content_type_xml();

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        request = request.body(&format!(
            include_str!("./update-addressbook.xml"),
            &addressbook.name, color, desc,
        ));

        Self {
            addressbook,
            http: SendHttpRequest::new(request),
        }
    }

    pub fn output(mut self) -> Result<Addressbook, quick_xml::de::DeError> {
        let body = self.http.take_body();

        if body.is_empty() {
            return Ok(self.addressbook);
        }

        let response: Response = quick_xml::de::from_reader(body.as_slice())?;

        let Some(propstats) = response.propstats else {
            return Ok(self.addressbook);
        };

        for propstat in propstats {
            if !propstat.status.is_success() {
                debug!(?propstat, "multistatus propstat error");
                continue;
            }

            if let Some(name) = propstat.prop.displayname {
                self.addressbook.name = name
            }

            if let Some(desc) = propstat.prop.addressbook_description {
                self.addressbook.desc = Some(desc);
            }

            if let Some(color) = propstat.prop.addressbook_color {
                self.addressbook.color = Some(color);
            }
        }

        Ok(self.addressbook)
    }
}

pub type Response = MkcolResponse<Prop>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub displayname: Option<String>,
    pub addressbook_color: Option<String>,
    pub addressbook_description: Option<String>,
}

impl Flow for UpdateAddressbook {}

impl Write for UpdateAddressbook {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for UpdateAddressbook {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for UpdateAddressbook {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
