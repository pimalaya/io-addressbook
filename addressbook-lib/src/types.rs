use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use tracing::{debug, warn};
use uuid::Uuid;
use vparser::Parser;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub version: CardVersion,
    pub properties: HashMap<String, String>,
}

impl Card {
    pub fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn new(version: CardVersion) -> Self {
        Self {
            id: Self::generate_id(),
            version,
            properties: Default::default(),
        }
    }

    pub fn new_v2_1() -> Self {
        Self {
            id: Self::generate_id(),
            version: CardVersion::V2_1,
            properties: Default::default(),
        }
    }

    pub fn new_v3_0() -> Self {
        Self {
            id: Self::generate_id(),
            version: CardVersion::V3_0,
            properties: Default::default(),
        }
    }

    pub fn new_v4_0() -> Self {
        Self {
            id: Self::generate_id(),
            version: CardVersion::V4_0,
            properties: Default::default(),
        }
    }

    pub fn parse(id: impl ToString, content: impl AsRef<str>) -> Option<Self> {
        let id = id.to_string();

        let mut version = None;
        let mut properties = HashMap::new();

        for line in Parser::new(content.as_ref()) {
            match line.name().as_ref() {
                "BEGIN" => continue,
                "END" => continue,
                "VERSION" => match line.value().as_ref() {
                    "2.1" => version = Some(CardVersion::V2_1),
                    "3.0" => version = Some(CardVersion::V3_0),
                    "4.0" => version = Some(CardVersion::V4_0),
                    v => {
                        debug!("unknown vCard version {v}");
                        version = None;
                    }
                },
                name => {
                    properties.insert(name.to_owned(), line.value().to_string());
                }
            };
        }

        let Some(version) = version else {
            warn!(id, "discard vCard with invalid version");
            return None;
        };

        Some(Card {
            id,
            version,
            properties,
        })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BEGIN:VCARD\r")?;
        writeln!(f, "VERSION:{}\r", self.version)?;

        for (key, val) in &self.properties {
            writeln!(f, "{key}:{val}\r")?;
        }

        writeln!(f, "END:VCARD\r")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CardVersion {
    #[serde(rename = "2.1")]
    V2_1,
    #[serde(rename = "3.0")]
    V3_0,
    #[serde(rename = "4.0")]
    V4_0,
}

impl fmt::Display for CardVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V2_1 => write!(f, "2.1"),
            Self::V3_0 => write!(f, "3.0"),
            Self::V4_0 => write!(f, "4.0"),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Cards(Vec<Card>);

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
