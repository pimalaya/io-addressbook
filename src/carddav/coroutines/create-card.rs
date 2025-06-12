use io_http::v1_1::coroutines::Send;
use io_stream::io::StreamIo;

use crate::{
    carddav::{config::CarddavConfig, request::Request},
    Card,
};

#[derive(Debug)]
pub struct CreateCard(Send);

impl CreateCard {
    pub fn new(config: &CarddavConfig, card: Card) -> Self {
        let path = format!("/{}/{}.vcf", card.addressbook_id, card.id);
        let request = Request::put(config, path).content_type_vcard();
        let request = request.body(card.to_string().into_bytes());
        Self(Send::new(request))
    }

    pub fn resume(&mut self, arg: Option<Io>) -> Result<(), Io> {
        let (_, response) = self.0.resume(arg)?;
        let body = String::from_utf8_lossy(response.body());

        if !response.status().is_success() {
            let err = format!("HTTP error: {}: {body}", response.status());
            return Err(Io::err(err));
        }

        Ok(())
    }
}
