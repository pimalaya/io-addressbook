use crate::{
    carddav::{
        config::Authentication,
        http::{Request, SendHttpRequest},
        tcp, Config,
    },
    Card,
};

#[derive(Debug)]
pub struct CreateCard {
    card: Card,
    http: SendHttpRequest,
}

impl CreateCard {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card: Card) -> Self {
        let addressbook_id = addressbook_id.as_ref();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}/{}.vcf", card.id);
        let mut request = Request::put(uri, config.http_version.as_ref()).content_type_vcard();

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        request = request.body(&card.to_string());

        Self {
            card,
            http: SendHttpRequest::new(request),
        }
    }

    pub fn output(self) -> Card {
        self.card
    }
}

impl AsMut<tcp::IoState> for CreateCard {
    fn as_mut(&mut self) -> &mut tcp::IoState {
        self.http.as_mut()
    }
}

impl Iterator for CreateCard {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
