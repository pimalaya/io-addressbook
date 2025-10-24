use io_stream::io::StreamIo;

use crate::{
    addressbook::Addressbook,
    carddav::{config::CarddavConfig, request::Request},
};

use super::send::{Empty, Send, SendResult};

#[derive(Debug)]
pub struct CreateAddressbook(Send<Empty>);

impl CreateAddressbook {
    pub fn new(config: &CarddavConfig, mut addressbook: Addressbook) -> Self {
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

        let request = Request::mkcol(config, addressbook.id).content_type_xml();
        let body = format!(include_str!("./create-addressbook.xml"), name, color, desc);

        Self(Send::new(request, body.as_bytes().to_vec()))
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<Empty> {
        self.0.resume(arg)
    }
}
