use serde::Deserialize;
use tracing::debug;

use crate::{
    carddav::response::MkcolResponse,
    http::{Request, SendHttpRequest},
    tcp, PartialAddressbook,
};

use super::{client::Authentication, Config};

#[derive(Debug)]
pub struct UpdateAddressbook {
    addressbook: PartialAddressbook,
    http: SendHttpRequest,
}

impl UpdateAddressbook {
    pub fn new(config: &Config, addressbook: PartialAddressbook) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}", addressbook.id);

        let name = match &addressbook.name {
            Some(name) => format!("<displayname>{name}</displayname>"),
            None => String::new(),
        };

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
            name, color, desc,
        ));

        Self {
            addressbook,
            http: SendHttpRequest::new(request),
        }
    }

    pub fn output(mut self) -> Result<PartialAddressbook, quick_xml::de::DeError> {
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

            self.addressbook.name = propstat.prop.displayname;
            self.addressbook.desc = propstat.prop.addressbook_description;
            self.addressbook.color = propstat.prop.addressbook_color;
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

impl AsMut<tcp::State> for UpdateAddressbook {
    fn as_mut(&mut self) -> &mut tcp::State {
        self.http.as_mut()
    }
}

impl Iterator for UpdateAddressbook {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
