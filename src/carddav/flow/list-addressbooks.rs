use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{
        config::Authentication,
        http::{Request, SendHttpRequest},
        response::Multistatus,
        tcp, Config,
    },
    Addressbook, Addressbooks,
};

#[derive(Debug)]
pub struct ListAddressbooks {
    http: SendHttpRequest,
}

impl ListAddressbooks {
    const BODY: &'static str = include_str!("./list-addressbooks.xml");

    pub fn new(config: &Config) -> Self {
        let mut request = Request::propfind(&config.home_uri, config.http_version.as_ref())
            .content_type_xml()
            .depth("1");

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            http: SendHttpRequest::new(request.body(Self::BODY)),
        }
    }

    pub fn output(self) -> Result<Addressbooks, quick_xml::de::DeError> {
        let body = self.http.take_body();
        let response: Response = quick_xml::de::from_reader(body.as_slice())?;
        let mut addressbooks = Addressbooks::default();

        let Some(responses) = response.responses else {
            return Ok(addressbooks);
        };

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

            let mut is_addressbook = None;
            let mut addressbook = Addressbook::default();
            addressbook.id = response
                .href
                .value
                .trim_end_matches('/')
                .rsplit('/')
                .next()
                .unwrap() // SAFETY: addressbooks belong to principal
                .to_owned();

            for propstat in propstats {
                if let Some(false) = is_addressbook {
                    break;
                }

                if !propstat.status.is_success() {
                    debug!(?propstat, "multistatus propstat response error");
                    continue;
                }

                if let Some(rtype) = propstat.prop.resourcetype {
                    if rtype.addressbook.is_some() {
                        is_addressbook.replace(true);
                    }
                }

                if let Some(name) = propstat.prop.displayname {
                    addressbook.name = name
                }

                if let Some(desc) = propstat.prop.addressbook_description {
                    addressbook.desc = Some(desc);
                }

                if let Some(color) = propstat.prop.addressbook_color {
                    addressbook.color = Some(color);
                }
            }

            if let Some(true) = is_addressbook {
                addressbooks.push(addressbook);
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

impl AsMut<tcp::State> for ListAddressbooks {
    fn as_mut(&mut self) -> &mut tcp::State {
        self.http.as_mut()
    }
}

impl Iterator for ListAddressbooks {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
