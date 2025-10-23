use std::collections::HashSet;

use calcard::vcard::VCard;
use io_stream::io::StreamIo;
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    card::Card,
    carddav::{
        config::CarddavConfig,
        request::Request,
        response::{Multistatus, Value},
    },
};

use super::send::{Send, SendOk, SendResult};

#[derive(Debug)]
pub struct ListCards {
    addressbook_id: String,
    send: Send<Multistatus<Prop>>,
}

impl ListCards {
    const BODY: &'static str = include_str!("./list-cards.xml");

    pub fn new(config: &CarddavConfig, addressbook_id: impl AsRef<str>) -> Self {
        let addressbook_id = addressbook_id.as_ref().to_owned();
        let request = Request::report(config, &addressbook_id)
            .content_type_xml()
            .depth(1);

        Self {
            addressbook_id,
            send: Send::new(request, Self::BODY.as_bytes().to_vec()),
        }
    }

    pub fn resume(&mut self, arg: Option<StreamIo>) -> SendResult<HashSet<Card>> {
        let ok = match self.send.resume(arg) {
            SendResult::Ok(ok) => ok,
            SendResult::Err(err) => return SendResult::Err(err),
            SendResult::Io(io) => return SendResult::Io(io),
        };

        let mut cards = HashSet::new();

        if let Some(responses) = ok.body.responses {
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
        };

        SendResult::Ok(SendOk {
            request: ok.request,
            response: ok.response,
            keep_alive: ok.keep_alive,
            body: cards,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub address_data: Option<Value>,
}
