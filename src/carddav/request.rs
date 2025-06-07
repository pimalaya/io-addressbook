use base64::{prelude::BASE64_STANDARD, Engine};
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE, HOST},
    Method, Version,
};
use secrecy::ExposeSecret;

use super::config::Auth;

#[derive(Debug, Default)]
pub struct Request {
    builder: http::request::Builder,
}

impl Request {
    pub fn new(method: Method, uri: &str, version: Version) -> Self {
        let builder = http::Request::builder()
            .method(method)
            .version(version)
            .uri(uri);

        Self { builder }
    }

    pub fn delete(uri: &str, version: Version) -> Self {
        Self::new(Method::DELETE, uri, version)
    }

    pub fn get(uri: &str, version: Version) -> Self {
        Self::new(Method::GET, uri, version)
    }

    pub fn mkcol(uri: &str, version: Version) -> Self {
        let method = Method::from_bytes(b"MKCOL").unwrap();
        Self::new(method, uri, version)
    }

    pub fn proppatch(uri: &str, version: Version) -> Self {
        let method = Method::from_bytes(b"PROPPATCH").unwrap();
        Self::new(method, uri, version)
    }

    pub fn propfind(uri: &str, version: Version) -> Self {
        let method = Method::from_bytes(b"PROPFIND").unwrap();
        Self::new(method, uri, version)
    }

    pub fn put(uri: &str, version: Version) -> Self {
        Self::new(Method::PUT, uri, version)
    }

    pub fn report(uri: &str, version: Version) -> Self {
        let method = Method::from_bytes(b"REPORT").unwrap();
        Self::new(method, uri, version)
    }

    pub fn host(mut self, host: &str, port: u16) -> Self {
        self.builder = self.builder.header(HOST, format!("{host}:{port}"));
        self
    }

    pub fn authorization(mut self, auth: &Auth) -> Self {
        match auth {
            Auth::Plain => (),
            Auth::Bearer { token } => {
                let auth = format!("Bearer {}", token.expose_secret());
                self.builder = self.builder.header(AUTHORIZATION, auth);
            }
            Auth::Basic { username, password } => {
                let password = password.expose_secret();
                let digest = BASE64_STANDARD.encode(format!("{username}:{password}"));
                let auth = format!("Basic {digest}");
                self.builder = self.builder.header(AUTHORIZATION, auth);
            }
        };
        self
    }

    pub fn depth(mut self, depth: usize) -> Self {
        self.builder = self.builder.header("Depth", depth);
        self
    }

    pub fn content_type(mut self, value: &str) -> Self {
        self.builder = self.builder.header(CONTENT_TYPE, value);
        self
    }

    pub fn content_type_xml(self) -> Self {
        self.content_type("text/xml; charset=utf-8")
    }

    pub fn content_type_vcard(self) -> Self {
        self.content_type("text/vcard; charset=utf-8")
    }

    pub fn body(self, body: impl IntoIterator<Item = u8>) -> http::Request<Vec<u8>> {
        self.builder.body(body.into_iter().collect()).unwrap()
    }
}
