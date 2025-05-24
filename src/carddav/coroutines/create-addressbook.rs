use io_stream::Io;
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{response::MkcolResponse, Config, Request},
    Addressbook,
};

use super::Send;

#[derive(Debug)]
pub struct CreateAddressbook(Send<MkcolResponse<Prop>>);

impl CreateAddressbook {
    pub fn new(config: &Config, mut addressbook: Addressbook) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}", addressbook.id);

        let name = match addressbook.display_name.take() {
            Some(name) => name,
            None => String::new(),
        };

        let color = match &addressbook.color.take() {
            Some(color) => format!("<I:addressbook-color>{color}</I:addressbook-color>"),
            None => String::new(),
        };

        let desc = match &addressbook.description.take() {
            Some(desc) => format!("<C:addressbook-description>{desc}</C:addressbook-description>"),
            None => String::new(),
        };

        let request = Request::mkcol(uri, config.http_version).content_type_xml();
        let body = format!(include_str!("./create-addressbook.xml"), name, color, desc);

        Self(Send::new(config, request, body.as_bytes()))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let body = self.0.resume(arg)?;

        let Some(propstats) = body.propstats else {
            return Ok(());
        };

        for propstat in propstats {
            if !propstat.status.is_success() {
                debug!("mkcol propstat error");
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

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub displayname: Option<String>,
    pub addressbook_color: Option<String>,
    pub addressbook_description: Option<String>,
}
