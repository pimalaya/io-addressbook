use io_http::v1_1::coroutines::Send;
use io_stream::Io;

use crate::{
    carddav::{Config, Request},
    Addressbook,
};

#[derive(Debug)]
pub struct CreateAddressbook(Send);

impl CreateAddressbook {
    pub fn new(config: &Config, mut addressbook: Addressbook) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}", addressbook.id);

        let name = match addressbook.display_name.take() {
            Some(name) => format!("<displayname>{name}</displayname>"),
            None => String::new(),
        };

        let desc = match &addressbook.description.take() {
            Some(desc) => format!("<C:addressbook-description>{desc}</C:addressbook-description>"),
            None => String::new(),
        };

        let color = match &addressbook.color.take() {
            Some(color) => format!("<I:addressbook-color>{color}</I:addressbook-color>"),
            None => String::new(),
        };

        let request = Request::mkcol(uri, config.http_version)
            .content_type_xml()
            .host(&config.host, config.port)
            .authorization(&config.auth);

        let body = format!(include_str!("./create-addressbook.xml"), name, color, desc);
        let request = request.body(body.as_bytes().to_vec());

        Self(Send::new(request))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let response = self.0.resume(arg)?;
        let body = String::from_utf8_lossy(response.body());

        if !response.status().is_success() {
            let err = format!("HTTP error: {}: {body}", response.status());
            return Err(Io::err(err));
        }

        Ok(())
    }
}
