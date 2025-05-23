use std::collections::HashSet;

use io_http::v1_1::coroutines::Send;
use io_stream::Io;
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{config::Authentication, response::Multistatus, Config, Request},
    Addressbook,
};

#[derive(Debug)]
pub struct ListAddressbooks(Send);

impl ListAddressbooks {
    const BODY: &'static str = include_str!("./list-addressbooks.xml");

    pub fn new(config: &Config) -> Self {
        let mut request = Request::propfind(&config.home_uri, config.http_version)
            .content_type_xml()
            .host(&config.host, config.port)
            .connection_keep_alive()
            .depth(1);

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        let body = Self::BODY.as_bytes().to_vec();

        Self(Send::new(request.body(body)))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<HashSet<Addressbook>, Io> {
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

        let mut addressbooks = HashSet::new();

        let Some(responses) = body.responses else {
            return Ok(addressbooks);
        };

        for response in responses {
            trace!("process multistatus");

            if let Some(status) = response.status {
                if !status.is_success() {
                    debug!("multistatus response error");
                    continue;
                }
            };

            let Some(propstats) = response.propstats else {
                continue;
            };

            let mut is_addressbook = None;

            let mut addressbook = Addressbook {
                id: response
                    .href
                    .value
                    .trim_end_matches('/')
                    .rsplit('/')
                    .next()
                    .unwrap() // SAFETY: addressbooks belong to principal
                    .to_owned(),
                display_name: None,
                description: None,
                color: None,
            };

            for propstat in propstats {
                if let Some(false) = is_addressbook {
                    break;
                }

                if !propstat.status.is_success() {
                    debug!("multistatus propstat response error");
                    continue;
                }

                if let Some(rtype) = propstat.prop.resourcetype {
                    if rtype.addressbook.is_some() {
                        is_addressbook.replace(true);
                    }
                }

                if let Some(name) = propstat.prop.displayname {
                    if !name.trim().is_empty() {
                        addressbook.display_name = Some(name);
                    }
                }

                if let Some(desc) = propstat.prop.addressbook_description {
                    if !desc.trim().is_empty() {
                        addressbook.description = Some(desc);
                    }
                }

                if let Some(color) = propstat.prop.addressbook_color {
                    if !color.trim().is_empty() {
                        addressbook.color = Some(color);
                    }
                }
            }

            if let Some(true) = is_addressbook {
                addressbooks.insert(addressbook);
            }
        }

        Ok(addressbooks)
    }
}

pub type Response = Multistatus<Prop>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub resourcetype: Option<ResourceType>,
    pub displayname: Option<String>,
    pub addressbook_color: Option<String>,
    pub addressbook_description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceType {
    pub addressbook: Option<()>,
}
