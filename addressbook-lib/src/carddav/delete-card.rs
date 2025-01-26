use crate::{
    http::{Request, SendHttpRequest},
    tcp,
};

use super::{client::Authentication, Config};

#[derive(Debug)]
pub struct DeleteCard {
    http: SendHttpRequest,
}

impl DeleteCard {
    const BODY: &'static str = "";

    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card_id: impl AsRef<str>) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!(
            "{base_uri}/{}/{}.vcf",
            addressbook_id.as_ref(),
            card_id.as_ref()
        );
        let mut request = Request::delete(uri.as_ref(), config.http_version.as_ref());

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            http: SendHttpRequest::new(request.body(Self::BODY)),
        }
    }
}

impl AsMut<tcp::State> for DeleteCard {
    fn as_mut(&mut self) -> &mut tcp::State {
        self.http.as_mut()
    }
}

impl Iterator for DeleteCard {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
