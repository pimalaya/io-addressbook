use io_stream::Io;
use io_vdir::constants::VCF;
use log::debug;

use crate::{
    carddav::{Config, Request},
    Card,
};

use super::Send;

#[derive(Debug)]
pub struct CreateCard(Send<()>);

impl CreateCard {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card: Card) -> Self {
        let addressbook_id = addressbook_id.as_ref();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}/{}.{}", card.id, VCF);
        let request = Request::put(uri, config.http_version).content_type_vcard();

        Self(Send::new(config, request, &[]))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        match self.0.resume(arg) {
            Ok(()) => {
                debug!("resume after uploading vcf file");
                Ok(())
            }
            Err(io) => {
                debug!("break: need I/O to upload vcf file");
                Err(io)
            }
        }
    }
}
