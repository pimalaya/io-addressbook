use std::mem;

use memchr::memmem;
use tracing::trace;

use crate::carddav::{
    http::{Request, CR, CRLF, LF},
    tcp,
};

const CRLF_CRLF: [u8; 4] = [CR, LF, CR, LF];
const CONTENT_LENGTH: &[u8] = b"Content-Length";

#[derive(Clone, Debug)]
enum Step {
    SerializeHttpRequest,
    SendHttpRequest,
    ReceiveHttpResponse,
}

#[derive(Debug)]
pub struct SendHttpRequest {
    state: tcp::IoState,
    step: Option<Step>,

    request: Request,
    response_bytes: Vec<u8>,
    response_body_start: usize,
    response_body_length: usize,
}

impl SendHttpRequest {
    pub fn new(request: Request) -> Self {
        Self {
            step: Some(Step::SerializeHttpRequest),
            state: tcp::IoState::new(),

            request,
            response_bytes: vec![],
            response_body_start: 0,
            response_body_length: 0,
        }
    }

    pub fn response(&self) -> &[u8] {
        &self.response_bytes
    }

    pub fn headers(&self) -> &[u8] {
        &self.response_bytes[..self.response_body_start]
    }

    pub fn body(&self) -> &[u8] {
        &self.response_bytes[self.response_body_start..]
    }

    pub fn take_response(self) -> Vec<u8> {
        self.response_bytes
    }

    pub fn take_headers(mut self) -> Vec<u8> {
        self.response_bytes
            .drain(..self.response_body_start)
            .collect()
    }

    pub fn take_body(mut self) -> Vec<u8> {
        self.response_bytes
            .drain(self.response_body_start..)
            .collect()
    }
}

impl AsMut<tcp::IoState> for SendHttpRequest {
    fn as_mut(&mut self) -> &mut tcp::IoState {
        &mut self.state
    }
}

impl Iterator for SendHttpRequest {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.step.take() {
                None => return None,
                Some(Step::SerializeHttpRequest) => {
                    self.step = Some(Step::SendHttpRequest);
                    let mut request = Request::default();
                    mem::swap(&mut request, &mut self.request);
                    self.state.write_buffer = request.into();
                    trace!(request = ?String::from_utf8_lossy(&self.state.write_buffer), "send full");
                    return Some(tcp::Io::Write);
                }
                Some(Step::SendHttpRequest) => {
                    self.step = Some(Step::ReceiveHttpResponse);
                    return Some(tcp::Io::Read);
                }
                Some(Step::ReceiveHttpResponse) => {
                    if self.state.read_bytes_count == 0 {
                        return None;
                    }

                    let bytes = &self.state.read_buffer[..self.state.read_bytes_count];
                    self.response_bytes.extend(bytes);

                    let i = self.state.read_bytes_count;
                    let n = self.state.read_buffer.len();
                    trace!(response = ?String::from_utf8_lossy(bytes), "receive chunk {i}/{n}");

                    if self.response_body_start == 0 {
                        let body_start = memmem::find(&self.response_bytes, &CRLF_CRLF);

                        if let Some(n) = body_start {
                            self.response_body_start = n + 4;
                        }
                    }

                    if self.response_body_start > 0 && self.response_body_length == 0 {
                        let mut content_length_start = None;

                        for crlf in memmem::find_iter(&self.response_bytes, &CRLF) {
                            if let Some(start) = content_length_start {
                                let length = &self.response_bytes[start..crlf];
                                let length = String::from_utf8_lossy(length);
                                self.response_body_length = length.trim().parse().unwrap();
                                break;
                            }

                            // take bytes after the found CRLF
                            let crlf = crlf + CRLF.len();
                            let bytes = &self.response_bytes[crlf..];

                            // break if length of bytes after CRLF is
                            // smaller than `Content-Length: 0`
                            let colon_space_digit = 3;
                            if bytes.len() < CONTENT_LENGTH.len() + colon_space_digit {
                                break;
                            }

                            // search for another CRLF if header does
                            // not match Content-Type
                            if !bytes[..CONTENT_LENGTH.len()].eq_ignore_ascii_case(CONTENT_LENGTH) {
                                continue;
                            }

                            content_length_start = Some(crlf + CONTENT_LENGTH.len() + 1);
                        }
                    }

                    if self.response_body_start > 0 && self.response_body_length > 0 {
                        let body_bytes = &self.response_bytes[self.response_body_start..];
                        if body_bytes.len() >= self.response_body_length {
                            return None;
                        }
                    }

                    self.step = Some(Step::ReceiveHttpResponse);
                    return Some(tcp::Io::Read);
                }
            }
        }
    }
}
