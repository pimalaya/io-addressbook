use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, trace};

use crate::{
    http::{Request, SendReceiveFlow},
    tcp::{Flow, Io, Read, Write},
};

use super::{
    response::{HrefProp, Multistatus},
    Authentication, Config,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot parse addressbook home set response body")]
    ParseAddressbookHomeSetResponseError(#[source] quick_xml::de::DeError),
    #[error("cannot find missing or empty addressbook home set responses")]
    FindAddressbookHomeSetResponseError,
}

#[derive(Debug)]
pub struct AddressbookHomeSet {
    http: SendReceiveFlow,
}

impl AddressbookHomeSet {
    const BODY: &str = r#"
        <propfind xmlns="DAV:" xmlns:C="urn:ietf:params:xml:ns:carddav">
            <prop>
                <C:addressbook-home-set />
            </prop>
        </propfind>
    "#;

    pub fn new(config: &Config, url: impl AsRef<str>) -> Self {
        let mut request =
            Request::propfind(url.as_ref(), config.http_version.as_ref()).content_type_xml();

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            http: SendReceiveFlow::new(request.body(Self::BODY)),
        }
    }

    pub fn output(self) -> Result<Option<String>, Error> {
        let body = self.http.take_body();

        let response: Result<Response, quick_xml::de::DeError> =
            quick_xml::de::from_reader(body.as_slice());

        let responses = response
            .map_err(Error::ParseAddressbookHomeSetResponseError)?
            .responses
            .ok_or(Error::FindAddressbookHomeSetResponseError)?;

        for response in responses {
            trace!(?response, "process multistatus");

            if let Some(status) = response.status {
                if !status.is_success() {
                    debug!(?status, "multistatus response error");
                    continue;
                }
            };

            let Some(propstats) = response.propstats else {
                continue;
            };

            for propstat in propstats {
                if !propstat.status.is_success() {
                    debug!(?propstat.status, "multistatus propstat response error");
                    continue;
                }

                return Ok(Some(propstat.prop.addressbook_home_set.href.value));
            }
        }

        Ok(None)
    }
}

pub type Response = Multistatus<Prop>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub addressbook_home_set: HrefProp,
}

impl Flow for AddressbookHomeSet {}

impl Write for AddressbookHomeSet {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for AddressbookHomeSet {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for AddressbookHomeSet {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
