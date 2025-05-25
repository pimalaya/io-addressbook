use io_http::v1_1::coroutines::Send;
use io_stream::Io;
use io_vdir::constants::VCF;

use crate::{
    carddav::{config::Authentication, Config, Request},
    Card,
};

#[derive(Debug)]
pub struct CreateCard(Send);

impl CreateCard {
    pub fn new(config: &Config, card: Card) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}/{}.{VCF}", card.addressbook_id, card.id);
        let mut request = Request::put(uri, config.http_version)
            .content_type_vcard()
            .host(&config.host, config.port);

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        let request = request.body(card.to_string().into_bytes());

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
