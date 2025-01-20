use chrono::{DateTime, Utc};
use memchr::memmem;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteAddressbook {
    #[serde(rename = "response")]
    pub responses: Vec<DeleteAddressbookResponse>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteAddressbookResponse {
    pub href: Href,
    pub status: Status,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Multistatus<T> {
    #[serde(rename = "response")]
    pub responses: Option<Vec<Response<T>>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MkcolResponse<T> {
    pub propstat: Propstat<T>,
}

// impl Multistatus<AddressbookProp> {
//     pub fn get_addressbook_hrefs(&self) -> impl Iterator<Item = &str> {
//         self.responses
//             .iter()
//             .filter_map(Response::get_addressbook_href)
//     }
// }

#[derive(Clone, Debug, Deserialize)]
pub struct Response<T> {
    pub href: Href,
    pub status: Option<Status>,
    #[serde(rename = "propstat")]
    pub propstats: Option<Vec<Propstat<T>>>,
}

// impl Response<AddressbookProp> {
//     pub fn get_addressbook_href(&self) -> Option<&str> {
//         for propstat in &self.propstats {
//             if let Some(resourcetype) = &propstat.prop.resourcetype {
//                 if resourcetype.addressbook.is_some() {
//                     return Some(self.href.value.as_str());
//                 }
//             }
//         }

//         None
//     }
// }

#[derive(Clone, Debug, Deserialize)]
pub struct Propstat<T> {
    pub prop: T,
    pub status: Status,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Href {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Displayname {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    #[serde(rename = "$value")]
    pub value: String,
}

impl Status {
    pub fn is_success(&self) -> bool {
        memmem::find(self.value.as_bytes(), b" 2").is_some()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Ctag {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Etag {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LastModified {
    #[serde(with = "date_parser", rename = "$value", default)]
    pub value: DateTime<Utc>,
}

mod date_parser {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc2822(&s)
            .map(|d| d.into())
            .map_err(serde::de::Error::custom)
    }
}

// Current user principal structs

// Addressbook home set structs

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddressbookHomeSetProp {
    pub addressbook_home_set: AddressbookHomeSet,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddressbookHomeSet {
    pub href: Href,
}

// Addressbook structs

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddressbookProp {
    pub resourcetype: Option<AddressbookResourceType>,
    pub displayname: Option<String>,
    pub addressbook_color: Option<String>,
    pub addressbook_description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddressbookResourceType {
    pub addressbook: Option<Addressbook>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Addressbook {}

// Address data structs

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddressDataProp {
    pub address_data: Option<AddressData>,
    pub getetag: Option<Etag>,
    pub getlastmodified: Option<LastModified>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddressData {
    #[serde(rename = "$value")]
    pub value: String,
}

// Ctag structs

#[derive(Clone, Debug, Deserialize)]
pub struct CtagProp {
    pub getctag: Ctag,
}
