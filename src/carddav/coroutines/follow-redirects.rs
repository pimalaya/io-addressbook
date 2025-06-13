use std::marker::PhantomData;

use http::{StatusCode, Uri};
use io_http::v1_1::coroutines::{
    follow_redirects::{
        FollowRedirects as HttpFollowRedirects, FollowRedirectsError as HttpFollowRedirectsError,
        FollowRedirectsResult as HttpFollowRedirectsResult,
    },
    send::Send as HttpSend,
};
use io_stream::io::StreamIo;
use serde::Deserialize;
use thiserror::Error;

use crate::carddav::request::Request;

use super::send::SendOk;

#[derive(Debug)]
pub enum FollowRedirectsResult<T> {
    Ok(SendOk<T>),
    Err(FollowRedirectsError),
    Io(StreamIo),
    Reset(Uri),
}

#[derive(Debug, Error)]
pub enum FollowRedirectsError {
    #[error("HTTP response error {0}: {1}")]
    Response(StatusCode, String),
    #[error("Parse HTTP response body error")]
    ParseResponseBody(quick_xml::DeError),

    #[error(transparent)]
    FollowRedirects(#[from] HttpFollowRedirectsError),
}

#[derive(Debug)]
pub struct FollowRedirects<T: for<'a> Deserialize<'a>> {
    phantom: PhantomData<T>,
    send: HttpFollowRedirects,
}

impl<T: for<'a> Deserialize<'a>> FollowRedirects<T> {
    pub fn new(request: Request, body: impl IntoIterator<Item = u8>) -> Self {
        let request = request.body(body.into_iter().collect::<Vec<_>>());
        let send = HttpSend::new(request);

        Self {
            phantom: PhantomData::default(),
            send: HttpFollowRedirects::new(send),
        }
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> FollowRedirectsResult<T> {
        let ok = match self.send.resume(arg) {
            HttpFollowRedirectsResult::Ok(ok) => ok,
            HttpFollowRedirectsResult::Err(err) => return FollowRedirectsResult::Err(err.into()),
            HttpFollowRedirectsResult::Io(io) => return FollowRedirectsResult::Io(io),
            HttpFollowRedirectsResult::Reset(uri) => return FollowRedirectsResult::Reset(uri),
        };

        let body = String::from_utf8_lossy(ok.response.body());

        if !ok.response.status().is_success() {
            let status = ok.response.status();
            let body = body.to_string();
            let err = FollowRedirectsError::Response(status, body);
            return FollowRedirectsResult::Err(err);
        }

        let body = match quick_xml::de::from_str(&body) {
            Ok(xml) => xml,
            Err(err) => {
                let err = FollowRedirectsError::ParseResponseBody(err);
                return FollowRedirectsResult::Err(err);
            }
        };

        FollowRedirectsResult::Ok(SendOk {
            request: ok.request,
            response: ok.response,
            keep_alive: ok.keep_alive,
            body,
        })
    }
}
