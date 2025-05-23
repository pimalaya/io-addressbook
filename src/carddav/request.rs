use base64::{prelude::BASE64_STANDARD, Engine};
use http::{
    header::{AUTHORIZATION, CONNECTION, CONTENT_TYPE, HOST},
    Method, Version,
};
use secrecy::{ExposeSecret, SecretString};

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

    pub fn basic_auth(mut self, user: &str, pass: &SecretString) -> Self {
        let pass = pass.expose_secret();
        let auth = BASE64_STANDARD.encode(format!("{user}:{pass}"));
        self.builder = self.builder.header(AUTHORIZATION, format!("Basic {auth}"));
        self
    }

    pub fn depth(mut self, depth: usize) -> Self {
        self.builder = self.builder.header("Depth", depth);
        self
    }

    pub fn connection_keep_alive(mut self) -> Self {
        self.builder = self.builder.header(CONNECTION, "keep-alive");
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
