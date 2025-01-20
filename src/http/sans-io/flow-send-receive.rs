use std::mem;

use memchr::memmem;
use tracing::trace;

use crate::{
    http::sans_io::CRLF,
    tcp::sans_io::{Flow, Io, Read, Write},
};

use super::{Request, State, CR, LF};

const CRLF_CRLF: [u8; 4] = [CR, LF, CR, LF];
const CONTENT_LENGTH: &[u8] = b"Content-Length";

#[derive(Debug)]
pub struct SendReceiveFlow {
    state: Option<State>,

    write_buffer: Vec<u8>,
    wrote_bytes_count: usize,

    read_buffer: Vec<u8>,
    read_bytes_count: usize,

    request: Request,
    response_bytes: Vec<u8>,
    response_body_start: usize,
    response_body_length: usize,
}

impl SendReceiveFlow {
    pub fn new(request: Request) -> Self {
        Self {
            state: Some(State::SerializeHttpRequest),

            write_buffer: vec![],
            wrote_bytes_count: 0,

            read_buffer: vec![0; 512],
            read_bytes_count: 0,

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

impl Flow for SendReceiveFlow {}

impl Write for SendReceiveFlow {
    fn get_buffer(&mut self) -> &[u8] {
        &self.write_buffer
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.wrote_bytes_count = count;
    }
}

impl Read for SendReceiveFlow {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.read_buffer
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.read_bytes_count = count;
    }
}

impl Iterator for SendReceiveFlow {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state.take() {
                None => return None,
                Some(State::SerializeHttpRequest) => {
                    self.state = Some(State::SendHttpRequest);
                    let mut request = Request::default();
                    mem::swap(&mut request, &mut self.request);
                    self.write_buffer = request.into();
                    trace!(request = ?String::from_utf8_lossy(&self.write_buffer), "send full");
                    return Some(Io::Write);
                }
                Some(State::SendHttpRequest) => {
                    self.state = Some(State::ReceiveHttpResponse);
                    return Some(Io::Read);
                }
                Some(State::ReceiveHttpResponse) => {
                    if self.read_bytes_count == 0 {
                        return None;
                    }

                    let bytes = &self.read_buffer[..self.read_bytes_count];
                    self.response_bytes.extend(bytes);

                    let i = self.read_bytes_count;
                    let n = self.read_buffer.len();
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

                    self.state = Some(State::ReceiveHttpResponse);
                    return Some(Io::Read);
                }
            }
        }
    }
}
