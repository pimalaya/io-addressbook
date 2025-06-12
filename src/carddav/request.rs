use base64::{prelude::BASE64_STANDARD, Engine};
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE, HOST},
    Method, Uri,
};
use secrecy::ExposeSecret;

use super::config::{CarddavAuth, CarddavConfig};

#[derive(Debug, Default)]
pub struct Request {
    builder: http::request::Builder,
}

impl Request {
    pub fn new(config: &CarddavConfig, method: Method, path: impl AsRef<str>) -> Self {
        let uri = push_uri_path(config.uri.clone().into_owned(), path);
        let mut builder = http::Request::builder().method(method).uri(uri);

        match (config.uri.host(), config.uri.port()) {
            (Some(host), Some(port)) => builder = builder.header(HOST, format!("{host}:{port}")),
            (Some(host), None) => builder = builder.header(HOST, host.to_string()),
            (None, _) => (),
        };

        match &config.auth {
            CarddavAuth::Plain => (),
            CarddavAuth::Bearer { token } => {
                let auth = format!("Bearer {}", token.expose_secret());
                builder = builder.header(AUTHORIZATION, auth);
            }
            CarddavAuth::Basic { username, password } => {
                let password = password.expose_secret();
                let digest = BASE64_STANDARD.encode(format!("{username}:{password}"));
                let auth = format!("Basic {digest}");
                builder = builder.header(AUTHORIZATION, auth);
            }
        }

        Self { builder }
    }

    pub fn delete(config: &CarddavConfig, path: impl AsRef<str>) -> Self {
        Self::new(config, Method::DELETE, path)
    }

    pub fn get(config: &CarddavConfig, path: impl AsRef<str>) -> Self {
        Self::new(config, Method::GET, path)
    }

    pub fn mkcol(config: &CarddavConfig, path: impl AsRef<str>) -> Self {
        let method = Method::from_bytes(b"MKCOL").unwrap();
        Self::new(config, method, path)
    }

    pub fn proppatch(config: &CarddavConfig, path: impl AsRef<str>) -> Self {
        let method = Method::from_bytes(b"PROPPATCH").unwrap();
        Self::new(config, method, path)
    }

    pub fn propfind(config: &CarddavConfig, path: impl AsRef<str>) -> Self {
        let method = Method::from_bytes(b"PROPFIND").unwrap();
        Self::new(config, method, path)
    }

    pub fn put(config: &CarddavConfig, path: impl AsRef<str>) -> Self {
        Self::new(config, Method::PUT, path)
    }

    pub fn report(config: &CarddavConfig, path: impl AsRef<str>) -> Self {
        let method = Method::from_bytes(b"REPORT").unwrap();
        Self::new(config, method, path)
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

pub fn set_uri_path(uri: Uri, path: impl AsRef<str>) -> Uri {
    let mut uri = uri.into_parts();
    uri.path_and_query = Some(path.as_ref().parse().unwrap());
    Uri::from_parts(uri).unwrap()
}

pub fn push_uri_path(uri: Uri, path: impl AsRef<str>) -> Uri {
    let path = path.as_ref();

    if path.is_empty() {
        return uri;
    }

    let mut uri = uri.into_parts();

    uri.path_and_query = Some(match uri.path_and_query {
        None => path.parse().unwrap(),
        Some(path_and_query) => {
            let base_path = path_and_query.path().trim_end_matches('/');
            let path = path.trim_start_matches('/');
            let mut path = format!("{base_path}/{path}");
            if let Some(query) = path_and_query.query() {
                path.push('?');
                path.push_str(query)
            }
            path.parse().unwrap()
        }
    });

    Uri::from_parts(uri).unwrap()
}
