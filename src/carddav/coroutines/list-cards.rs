use std::collections::HashSet;

use calcard::vcard::VCard;
use io_stream::Io;
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    carddav::{
        response::{Multistatus, Value},
        Config, Request,
    },
    Card,
};

use super::Send;

#[derive(Debug)]
pub struct ListCards {
    addressbook_id: String,
    send: Send<Multistatus<Prop>>,
}

impl ListCards {
    const BODY: &'static str = include_str!("./list-cards.xml");

    pub fn new(config: &Config, addressbook_id: impl ToString) -> Self {
        let addressbook_id = addressbook_id.to_string();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}");

        let request = Request::report(uri, config.http_version)
            .content_type_xml()
            .depth(1);

        Self {
            addressbook_id,
            send: Send::new(config, request, Self::BODY.as_bytes()),
        }
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<HashSet<Card>, Io> {
        let body = self.send.resume(input)?;

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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub address_data: Option<Value>,
}
