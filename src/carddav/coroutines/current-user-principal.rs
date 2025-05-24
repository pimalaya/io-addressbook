use io_stream::Io;
use log::debug;
use serde::Deserialize;

use crate::carddav::{
    response::{HrefProp, Multistatus},
    Config, Request,
};

use super::Send;

#[derive(Debug)]
pub struct CurrentUserPrincipal(Send<Multistatus<Prop>>);

impl CurrentUserPrincipal {
    const BODY: &'static str = include_str!("./current-user-principal.xml");

    pub fn new(config: &Config, uri: impl AsRef<str>) -> Self {
        let request = Request::propfind(uri.as_ref(), config.http_version).content_type_xml();
        Self(Send::new(config, request, Self::BODY.as_bytes()))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> Result<Option<String>, Io> {
        let body = self.0.resume(arg)?;

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

                return Ok(Some(propstat.prop.current_user_principal.href.value));
            }
        }

        Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub current_user_principal: HrefProp,
}
