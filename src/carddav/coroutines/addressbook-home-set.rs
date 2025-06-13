use http::Uri;
use io_stream::io::StreamIo;
use log::debug;
use serde::Deserialize;

use crate::carddav::{
    config::CarddavConfig,
    request::Request,
    response::{HrefProp, Multistatus},
};

use super::{
    follow_redirects::{FollowRedirects, FollowRedirectsResult},
    send::SendOk,
};

#[derive(Debug)]
pub struct AddressbookHomeSet(FollowRedirects<Multistatus<Prop>>);

impl AddressbookHomeSet {
    const BODY: &'static str = include_str!("./addressbook-home-set.xml");

    pub fn new(config: &CarddavConfig) -> Self {
        let request = Request::propfind(config, "/");
        let body = Self::BODY.as_bytes().into_iter().cloned();
        Self(FollowRedirects::new(request, body))
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> FollowRedirectsResult<Option<Uri>> {
        let ok = match self.0.resume(arg) {
            FollowRedirectsResult::Ok(ok) => ok,
            FollowRedirectsResult::Err(err) => return FollowRedirectsResult::Err(err),
            FollowRedirectsResult::Io(io) => return FollowRedirectsResult::Io(io),
            FollowRedirectsResult::Reset(uri) => return FollowRedirectsResult::Reset(uri),
        };

        let Some(responses) = ok.body.responses else {
            return FollowRedirectsResult::Ok(SendOk {
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

                return FollowRedirectsResult::Ok(SendOk {
                    request: ok.request,
                    response: ok.response,
                    keep_alive: ok.keep_alive,
                    body: propstat.prop.addressbook_home_set.uri().ok(),
                });
            }
        }

        FollowRedirectsResult::Ok(SendOk {
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
