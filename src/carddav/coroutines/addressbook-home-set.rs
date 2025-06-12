use http::Uri;
use io_stream::io::StreamIo;
use log::debug;
use serde::Deserialize;

use crate::carddav::{
    config::CarddavConfig,
    request::Request,
    response::{HrefProp, Multistatus},
};

use super::send::{Send, SendOk, SendResult};

#[derive(Debug)]
pub struct AddressbookHomeSet(Send<Multistatus<Prop>>);

impl AddressbookHomeSet {
    const BODY: &'static str = include_str!("./addressbook-home-set.xml");

    pub fn new(config: &CarddavConfig) -> Self {
        let request = Request::propfind(config, "/");
        let body = Self::BODY.as_bytes().into_iter().cloned();
        Self(Send::new(request, body))
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<SendOk<Option<Uri>>> {
        let ok = match self.0.resume(arg) {
            SendResult::Ok(ok) => ok,
            SendResult::Err(err) => return SendResult::Err(err),
            SendResult::Io(io) => return SendResult::Io(io),
            SendResult::Reset(uri) => return SendResult::Reset(uri),
        };

        let Some(responses) = ok.body.responses else {
            return SendResult::Ok(SendOk {
                request: ok.request,
                response: ok.response,
                keep_alive: ok.keep_alive,
                body: None,
            });
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

                return SendResult::Ok(SendOk {
                    request: ok.request,
                    response: ok.response,
                    keep_alive: ok.keep_alive,
                    body: propstat.prop.addressbook_home_set.uri().ok(),
                });
            }
        }

        SendResult::Ok(SendOk {
            request: ok.request,
            response: ok.response,
            keep_alive: ok.keep_alive,
            body: None,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub addressbook_home_set: HrefProp,
}
