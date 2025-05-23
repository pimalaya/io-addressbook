use std::marker::PhantomData;

use io_stream::Io;
use serde::Deserialize;

use crate::carddav::{config::Authentication, Config, Request};

#[derive(Debug)]
pub struct Send<T: for<'a> Deserialize<'a>> {
    phantom: PhantomData<T>,
    send: io_http::v1_1::coroutines::Send,
}

impl<T: for<'a> Deserialize<'a>> Send<T> {
    pub fn new(config: &Config, mut request: Request, body: &[u8]) -> Self {
        request = request.host(&config.host, config.port);

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        let request = request.body(body.to_vec());

        Self {
            phantom: PhantomData::default(),
            send: io_http::v1_1::coroutines::Send::new(request),
        }
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<T, Io> {
        let response = self.send.resume(input)?;
        let body = String::from_utf8_lossy(response.body());

        if !response.status().is_success() {
            let err = format!("HTTP error: {}: {body}", response.status());
            return Err(Io::err(err));
        }

        match quick_xml::de::from_str(&body) {
            Ok(xml) => Ok(xml),
            Err(err) => {
                let err = format!("HTTP error: parse XML response body error: {err}");
                return Err(Io::err(err));
            }
        }
    }
}
