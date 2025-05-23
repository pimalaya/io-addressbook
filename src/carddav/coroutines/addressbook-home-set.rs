use io_http::v1_1::coroutines::Send;
use io_stream::Io;
use log::debug;
use serde::Deserialize;

use crate::carddav::{
    config::Authentication,
    response::{HrefProp, Multistatus},
    Config, Request,
};

#[derive(Debug)]
pub struct AddressbookHomeSet(Send);

impl AddressbookHomeSet {
    const BODY: &'static str = include_str!("./addressbook-home-set.xml");

    pub fn new(config: &Config, uri: impl AsRef<str>) -> Self {
        let mut request = Request::propfind(uri.as_ref(), config.http_version)
            .content_type_xml()
            .host(&config.host, config.port)
            .connection_keep_alive();

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        let body = Self::BODY.as_bytes().to_vec();

        Self(Send::new(request.body(body)))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<Option<String>, Io> {
        let response = self.0.resume(input)?;
        let body = String::from_utf8_lossy(response.body());

        if !response.status().is_success() {
            let err = format!("HTTP {}: {body}", response.status());
            return Err(Io::err(err));
        }

        let body: Response = match quick_xml::de::from_str(&body) {
            Ok(xml) => xml,
            Err(err) => {
                let err = format!("HTTP response error: XML body parsing error: {err}");
                return Err(Io::err(err));
            }
        };

        let Some(responses) = body.responses else {
            return Ok(None);
        };

        for response in responses {
            // trace!("process multistatus");

            if let Some(status) = response.status {
                if !status.is_success() {
                    debug!("multistatus response error");
                    continue;
                }
            };

            let Some(propstats) = response.propstats else {
                continue;
            };

            for propstat in propstats {
                if !propstat.status.is_success() {
                    debug!("multistatus propstat response error");
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
