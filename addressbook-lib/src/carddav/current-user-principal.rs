use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, trace};

use crate::{
    http::{Request, SendReceiveFlow},
    tcp::{Flow, Io, Read, Write},
};

use super::{
    client::{Authentication, Config},
    response::{HrefProp, Multistatus},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot parse current user principal response body")]
    ParseCurrentUserPrincipalResponseError(#[source] quick_xml::de::DeError),
    #[error("cannot find missing or empty current user principal responses")]
    FindCurrentUserPrincipalResponseError,
}

#[derive(Debug)]
pub struct CurrentUserPrincipal {
    http: SendReceiveFlow,
}

impl CurrentUserPrincipal {
    const BODY: &'static str = r#"
        <propfind xmlns="DAV:">
            <prop>
                <current-user-principal />
            </prop>
        </propfind>
    "#;

    pub fn new(config: &Config) -> Self {
        let mut request =
            Request::propfind(&config.root_url, config.http_version.as_ref()).content_type_xml();

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
            .map_err(Error::ParseCurrentUserPrincipalResponseError)?
            .responses
            .ok_or(Error::FindCurrentUserPrincipalResponseError)?;

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

                return Ok(Some(propstat.prop.current_user_principal.href.value));
            }
        }

        Ok(None)
    }
}

pub type Response = Multistatus<Prop>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub current_user_principal: HrefProp,
}

impl Flow for CurrentUserPrincipal {}

impl Write for CurrentUserPrincipal {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for CurrentUserPrincipal {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for CurrentUserPrincipal {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
