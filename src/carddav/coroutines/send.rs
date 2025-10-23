use std::marker::PhantomData;

use http::StatusCode;
use io_http::v1_1::coroutines::send::{SendHttp, SendHttpError, SendHttpResult};
use io_stream::io::StreamIo;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

use crate::carddav::request::Request;

#[derive(Debug)]
pub struct SendOk<T> {
    pub request: http::request::Request<Vec<u8>>,
    pub response: http::response::Response<Vec<u8>>,
    pub keep_alive: bool,
    pub body: T,
}

#[derive(Debug, Error)]
pub enum SendError {
    #[error("HTTP response error {0}: {1}")]
    Response(StatusCode, String),
    #[error("Parse HTTP response XML body error")]
    ParseXmlResponseBody(quick_xml::DeError),
    #[error("Parse HTTP response vCard body error")]
    ParseVcardResponseBody(calcard::Entry),

    #[error(transparent)]
    Send(#[from] SendHttpError),
}

/// Send result returned by the coroutine's resume function.
#[derive(Debug)]
pub enum SendResult<T> {
    /// The coroutine has successfully terminated its execution.
    Ok(SendOk<T>),
    /// The coroutine encountered an error.
    Err(SendError),
    /// The coroutine wants stream I/O.
    Io(StreamIo),
}

#[derive(Debug)]
pub struct Send<T: for<'a> Deserialize<'a>> {
    phantom: PhantomData<T>,
    send: SendHttp,
}

impl<T: for<'a> Deserialize<'a>> Send<T> {
    pub fn new(request: Request, body: impl IntoIterator<Item = u8>) -> Self {
        let request = request.body(body.into_iter().collect::<Vec<_>>());

        Self {
            phantom: PhantomData::default(),
            send: SendHttp::new(request),
        }
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<T> {
        let ok = match self.send.resume(arg) {
            SendHttpResult::Ok(ok) => ok,
            SendHttpResult::Err(err) => return SendResult::Err(err.into()),
            SendHttpResult::Io(io) => return SendResult::Io(io),
        };

        let body = String::from_utf8_lossy(ok.response.body());

        if !ok.response.status().is_success() {
            let status = ok.response.status();
            let body = body.to_string();
            return SendResult::Err(SendError::Response(status, body));
        }

        let body = match quick_xml::de::from_str(&body) {
            Ok(xml) => xml,
            Err(err) => return SendResult::Err(SendError::ParseXmlResponseBody(err)),
        };

        SendResult::Ok(SendOk {
            request: ok.request,
            response: ok.response,
            keep_alive: ok.keep_alive,
            body,
        })
    }
}

#[derive(Debug)]
pub struct Empty;

impl<'de> Deserialize<'de> for Empty {
    fn deserialize<D: Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        Ok(Empty)
    }
}
