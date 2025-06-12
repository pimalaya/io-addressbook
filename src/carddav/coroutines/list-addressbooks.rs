use std::collections::HashSet;

use io_stream::io::StreamIo;
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{config::CarddavConfig, request::Request, response::Multistatus},
    Addressbook,
};

use super::send::{Send, SendOk, SendResult};

#[derive(Debug)]
pub struct ListAddressbooks(Send<Multistatus<Prop>>);

impl ListAddressbooks {
    const BODY: &'static str = include_str!("./list-addressbooks.xml");

    pub fn new(config: &CarddavConfig) -> Self {
        let request = Request::propfind(config, "").depth(1);
        let body = Self::BODY.as_bytes().into_iter().cloned();
        Self(Send::new(request, body))
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<SendOk<HashSet<Addressbook>>> {
        let ok = match self.0.resume(arg) {
            SendResult::Ok(ok) => ok,
            SendResult::Err(err) => return SendResult::Err(err),
            SendResult::Io(io) => return SendResult::Io(io),
            SendResult::Reset(uri) => return SendResult::Reset(uri),
        };

        let mut addressbooks = HashSet::new();

        let Some(responses) = ok.body.responses else {
            return SendResult::Ok(SendOk {
                request: ok.request,
                response: ok.response,
                keep_alive: ok.keep_alive,
                body: addressbooks,
            });
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

        SendResult::Ok(SendOk {
            request: ok.request,
            response: ok.response,
            keep_alive: ok.keep_alive,
            body: addressbooks,
        })
    }
}

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
