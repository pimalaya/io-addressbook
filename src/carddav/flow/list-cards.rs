use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{
        config::Authentication,
        http::{Request, SendHttpRequest},
        response::{Multistatus, Value},
        tcp, Config,
    },
    Card, Cards,
};

#[derive(Debug)]
pub struct ListCards {
    http: SendHttpRequest,
}

impl ListCards {
    const BODY: &'static str = include_str!("./list-cards.xml");

    pub fn new(config: &Config, addressbook_id: impl AsRef<str>) -> Self {
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{}", addressbook_id.as_ref());
        let mut request = Request::report(uri, config.http_version.as_ref())
            .content_type_xml()
            .depth("1");

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            http: SendHttpRequest::new(request.body(Self::BODY)),
        }
    }

    pub fn output(self) -> Result<Cards, quick_xml::de::DeError> {
        let body = self.http.take_body();
        let response: Response = quick_xml::de::from_reader(body.as_slice())?;
        let mut cards = Cards::default();

        let Some(responses) = response.responses else {
            return Ok(cards);
        };

        for response in responses {
            trace!(?response, "process multistatus");

            if let Some(status) = response.status {
                if !status.is_success() {
                    debug!(?status, "multistatus response error");
                    continue;
                }
            };

            let Some(propstats) = response.propstats else {
                continue;
            };

            let mut parts = response.href.value.trim_end_matches('/').rsplit(['.', '/']);
            // SAFETY: cards have .vcf extension
            parts.next().unwrap();
            // SAFETY: cards belong to addressbooks
            let id = parts.next().unwrap();

            let mut card = None;

            for propstat in propstats {
                if !propstat.status.is_success() {
                    debug!(?propstat.status, "multistatus propstat error");
                    continue;
                }

                let Some(content) = propstat.prop.address_data else {
                    continue;
                };

                let Some(this_card) = Card::parse(id, content.value) else {
                    continue;
                };

                card.replace(this_card);
                break;
            }

            let Some(card) = card else {
                continue;
            };

            cards.push(card);
        }

        Ok(cards)
    }
}

pub type Response = Multistatus<Prop>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub address_data: Option<Value>,
}

impl AsMut<tcp::State> for ListCards {
    fn as_mut(&mut self) -> &mut tcp::State {
        self.http.as_mut()
    }
}

impl Iterator for ListCards {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
