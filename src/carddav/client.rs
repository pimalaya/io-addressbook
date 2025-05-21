use secrecy::SecretString;

use crate::{Addressbook, Card, PartialAddressbook};

use super::{
    config::Authentication, AddressbookHomeSet, Config, CreateAddressbook, CreateCard,
    CurrentUserPrincipal, DeleteAddressbook, DeleteCard, ListAddressbooks, ListCards, ReadCard,
    UpdateAddressbook, UpdateCard,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(debug_assertions)]
    pub fn new_from_envs() -> Self {
        use crate::carddav::config::{Authentication, HttpVersion};

        let mut config = Config::default();

        if let Ok(hostname) = std::env::var("HOST") {
            println!("using custom HOST {hostname}");
            config.hostname = hostname;
        } else {
            println!("using default HOST {}", config.hostname);
        }

        if let Ok(port) = std::env::var("PORT") {
            let port = port.parse::<u16>().expect("should be an integer");
            println!("using custom PORT {port}");
            config.port = port;
        } else {
            println!("using default PORT {}", config.port);
        }

        if let Ok(v) = std::env::var("HTTP_VERSION") {
            config.http_version = if v == "1.0" {
                HttpVersion::V1_0
            } else {
                HttpVersion::V1_1
            };
            println!("using custom HTTP_VERSION {:?}", config.http_version);
        } else {
            println!("using default HTTP_VERSION {:?}", config.http_version);
        }

        if let (Ok(user), Ok(pass)) = (std::env::var("USER"), std::env::var("PASS")) {
            println!("using basic authentication {user}:{{{pass}}}");
            let mut args = pass.split_whitespace();
            let program = args.next().unwrap();
            let pass = std::process::Command::new(program).args(args).output();
            let pass = pass.expect("should get password from command").stdout;
            let pass = String::from_utf8_lossy(pass.trim_ascii()).to_string();
            config.authentication = Authentication::Basic(user, pass.into());
        } else {
            println!("using no authentication");
        }

        if let Ok(uri) = std::env::var("URI") {
            println!("using custom home URI {uri}");
            config.home_uri = uri;
        } else {
            let uri = &config.home_uri;
            println!("using default home URI {uri}");
        }

        Self { config }
    }

    pub fn set_basic_authentication(&mut self, user: impl ToString, pass: impl Into<SecretString>) {
        self.config.authentication = Authentication::Basic(user.to_string(), pass.into());
    }

    pub fn with_basic_authentication(
        mut self,
        user: impl ToString,
        pass: impl Into<SecretString>,
    ) -> Self {
        self.set_basic_authentication(user, pass);
        self
    }

    pub fn current_user_principal(&self) -> CurrentUserPrincipal {
        CurrentUserPrincipal::new(&self.config, "/")
    }

    pub fn addressbook_home_set(&self, uri: impl AsRef<str>) -> AddressbookHomeSet {
        AddressbookHomeSet::new(&self.config, uri)
    }

    pub fn create_addressbook(&self, addressbook: Addressbook) -> CreateAddressbook {
        CreateAddressbook::new(&self.config, addressbook)
    }

    pub fn list_addressbooks(&self) -> ListAddressbooks {
        ListAddressbooks::new(&self.config)
    }

    pub fn update_addressbook(&self, addressbook: PartialAddressbook) -> UpdateAddressbook {
        UpdateAddressbook::new(&self.config, addressbook)
    }

    pub fn delete_addressbook(&self, id: impl AsRef<str>) -> DeleteAddressbook {
        DeleteAddressbook::new(&self.config, id)
    }

    pub fn create_card(&self, addressbook_id: impl AsRef<str>, card: Card) -> CreateCard {
        CreateCard::new(&self.config, addressbook_id, card)
    }

    pub fn read_card(&self, addressbook_id: impl AsRef<str>, card_id: impl ToString) -> ReadCard {
        ReadCard::new(&self.config, addressbook_id, card_id)
    }

    pub fn list_cards(&self, addressbook_id: impl AsRef<str>) -> ListCards {
        ListCards::new(&self.config, addressbook_id)
    }

    pub fn update_card(&self, addressbook_id: impl AsRef<str>, card: Card) -> UpdateCard {
        UpdateCard::new(&self.config, addressbook_id, card)
    }

    pub fn delete_card(
        &self,
        addressbook_id: impl AsRef<str>,
        card_id: impl AsRef<str>,
    ) -> DeleteCard {
        DeleteCard::new(&self.config, addressbook_id, card_id)
    }
}
