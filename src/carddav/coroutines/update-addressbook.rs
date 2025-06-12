use io_stream::io::StreamIo;
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{config::CarddavConfig, request::Request, response::MkcolResponse},
    Addressbook,
};

use super::{Send, SendResult};

#[derive(Debug)]
pub struct UpdateAddressbook(Send<MkcolResponse<Prop>>);

impl UpdateAddressbook {
    pub fn new(config: &CarddavConfig, mut addressbook: Addressbook) -> Self {
        let name = match addressbook.display_name.take() {
            Some(name) => format!("<displayname>{name}</displayname>"),
            None => String::new(),
        };

        let color = match addressbook.color.take() {
            Some(color) => format!("<I:addressbook-color>{color}</I:addressbook-color>"),
            None => String::new(),
        };

        let desc = match addressbook.description.take() {
            Some(desc) => format!("<C:addressbook-description>{desc}</C:addressbook-description>"),
            None => String::new(),
        };

        let request = Request::proppatch(config, addressbook.id).content_type_xml();
        let body = format!(include_str!("./update-addressbook.xml"), name, color, desc);
        Self(Send::new(request, body.as_bytes()))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> SendResult<()> {
        let body = match self.0.resume(arg) {
            SendResult::Ok(body) => body,
            SendResult::Err(err) => return SendResult::Err(err),
            SendResult::Io(io) => return SendResult::Io(io),
            SendResult::Redirect(res) => return SendResult::Redirect(res),
        };

        let Some(propstats) = body.propstats else {
            return SendResult::Ok(());
        };

        for propstat in propstats {
            if !propstat.status.is_success() {
                debug!("multistatus propstat error");
                continue;
            }

            match propstat.prop.displayname {
                Some(name) => trace!("addressbook displayname successfully created: {name}"),
                None => debug!("adressbook displayname could not be created"),
            }

            match propstat.prop.addressbook_description {
                Some(desc) => trace!("addressbook description successfully created: {desc}"),
                None => debug!("addressbook description could not be created"),
            }

            match propstat.prop.addressbook_color {
                Some(color) => trace!("addressbook color successfully created: {color}"),
                None => debug!("addressbook color could not be created"),
            }
        }

        SendResult::Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub displayname: Option<String>,
    pub addressbook_color: Option<String>,
    pub addressbook_description: Option<String>,
}
