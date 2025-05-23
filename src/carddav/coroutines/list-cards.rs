use std::collections::HashSet;

use calcard::vcard::VCard;
use io_http::v1_1::coroutines::Send;
use io_stream::Io;
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{
        config::Authentication,
        response::{Multistatus, Value},
        Config, Request,
    },
    Card,
};

#[derive(Debug)]
pub struct ListCards {
    addressbook_id: String,
    send: Send,
}

impl ListCards {
    const BODY: &'static str = include_str!("./list-cards.xml");

    pub fn new(config: &Config, addressbook_id: impl ToString) -> Self {
        let addressbook_id = addressbook_id.to_string();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}");
        let mut request = Request::report(uri, config.http_version)
            .content_type_xml()
            .host(&config.host, config.port)
            .connection_keep_alive()
            .depth(1);

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        let body = Self::BODY.as_bytes().to_vec();

        Self {
            addressbook_id,
            send: Send::new(request.body(body)),
        }
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<HashSet<Card>, Io> {
        let response = self.send.resume(input)?;
        let body = String::from_utf8_lossy(response.body());

        if !response.status().is_success() {
            let err = format!("HTTP {}: {body}", response.status());
            return Err(Io::err(err));
        }

        let body: Response = match quick_xml::de::from_str(&body) {
            Ok(xml) => xml,
            Err(err) => {
                let err = format!("HTTP response error: XML body parsing error: {err}");
                return Err(Io::err(err));
            }
        };

        let mut cards = HashSet::new();

        let Some(responses) = body.responses else {
            return Ok(cards);
        };

        for response in responses {
            trace!("process multistatus");

            if let Some(status) = response.status {
                if !status.is_success() {
                    debug!("multistatus response error");
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
                    debug!("multistatus propstat error");
                    continue;
                }

                let Some(content) = propstat.prop.address_data else {
                    continue;
                };

                let Ok(vcard) = VCard::parse(content.value) else {
                    continue;
                };

                card.replace(Card {
                    id: id.to_string(),
                    addressbook_id: self.addressbook_id.clone(),
                    vcard,
                });

                break;
            }

            let Some(card) = card else {
                continue;
            };

            cards.insert(card);
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
