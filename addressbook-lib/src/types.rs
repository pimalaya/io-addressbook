use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PartialAddressbook {
    pub id: String,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub color: Option<String>,
}

impl PartialAddressbook {
    pub fn new(id: impl ToString) -> Self {
        Self {
            id: id.to_string(),
            name: None,
            desc: None,
            color: None,
        }
    }
}

impl From<Addressbook> for PartialAddressbook {
    fn from(addressbook: Addressbook) -> Self {
        Self {
            id: addressbook.id,
            name: Some(addressbook.name),
            desc: addressbook.desc,
            color: addressbook.color,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Eq, PartialEq)]
pub struct Card {
    pub id: String,
    pub content: String,
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
