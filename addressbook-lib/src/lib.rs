#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../../README.md")]

pub mod carddav;
pub mod http;
pub mod tcp;

////////////////////////////////////////////////////////////////
// TODO: sort me

use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The main configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    /// The configuration of all the accounts.
    pub accounts: HashMap<String, AccountConfig>,
}

/// The account configuration.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct AccountConfig {
    /// The defaultness of the current account.
    #[serde(default)]
    pub default: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum HttpVersion {
    #[serde(rename = "1.0")]
    Http1_0,
    #[default]
    #[serde(rename = "1.1")]
    Http1_1,
}

impl AsRef<str> for HttpVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::Http1_0 => "1.0",
            Self::Http1_1 => "1.1",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Authentication {
    #[default]
    None,
    Basic(BasicAuthenticationConfig),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Encryption {
    None,
    Rustls(Rustls),
}

impl Default for Encryption {
    fn default() -> Self {
        Self::Rustls(Rustls::default())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rustls {
    #[serde(default)]
    provider: RustlsProvider,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RustlsProvider {
    #[default]
    AwsLc,
    Ring,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BasicAuthenticationConfig {
    #[serde(alias = "user", alias = "login")]
    pub username: String,
    #[serde(alias = "pass")]
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Addressbook {
    pub id: String,
    pub name: String,
    pub desc: Option<String>,
    pub color: Option<String>,
}

impl Default for Addressbook {
    fn default() -> Self {
        let uuid = Uuid::new_v4();

        Self {
            id: uuid.to_string(),
            name: uuid.to_string(),
            desc: None,
            color: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Addressbooks(Vec<Addressbook>);

impl Deref for Addressbooks {
    type Target = Vec<Addressbook>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Addressbooks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl TryFrom<ListAddressbooksFlow> for Addressbooks {
//     type Error = quick_xml::DeError;

//     fn try_from(flow: ListAddressbooksFlow) -> Result<Self, Self::Error> {
//         let mut addressbooks = Vec::new();
//         let output = flow.output()?;

//         for response in output.responses {
//             let id = &response.href.value;

//             for propstat in response.propstats {
//                 if let Some(t) = propstat.prop.resourcetype {
//                     if t.addressbook.is_some() {
//                         addressbooks.push(Addressbook {
//                             id: id.clone(),
//                             name: propstat.prop.displayname,
//                             desc: propstat.prop.addressbook_description,
//                             color: propstat.prop.addressbook_color,
//                         })
//                     }
//                 }
//             }
//         }

//         Ok(Self(addressbooks))
//     }
// }

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Card {
    pub id: String,
    pub content: String,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card")
            .field("id", &self.id)
            .field("content", &self.content)
            .field(
                "lines",
                &self
                    .content
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.trim().is_empty() {
                            None
                        } else {
                            Some(line)
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Default for Card {
    fn default() -> Self {
        let uuid = Uuid::new_v4();

        Self {
            id: uuid.to_string(),
            content: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Cards(Vec<Card>);

// impl TryFrom<ListCardsFlow> for Cards {
//     type Error = quick_xml::DeError;

//     fn try_from(flow: ListCardsFlow) -> Result<Self, Self::Error> {
//         let mut cards = Vec::new();
//         let output = flow.output()?;

//         for response in output.responses {
//             let id = &response.href.value;

//             for propstat in response.propstats {
//                 if let Some(vcf) = propstat.prop.address_data {
//                     let mut card = Card {
//                         id: id.clone(),
//                         props: Default::default(),
//                     };

//                     for line in Parser::new(&vcf.value) {
//                         let name = line.name().to_string();
//                         let value = line.value().to_string();
//                         card.props.insert(name, value);
//                     }

//                     cards.push(card)
//                 }
//             }
//         }

//         Ok(Self(cards))
//     }
// }

impl Deref for Cards {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
