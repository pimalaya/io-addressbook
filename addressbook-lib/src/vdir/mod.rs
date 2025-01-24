mod client;
mod config;
mod constants;
#[path = "create-addressbook.rs"]
pub mod create_addressbook;
#[path = "create-card.rs"]
pub mod create_card;
#[path = "delete-addressbook.rs"]
pub mod delete_addressbook;
pub mod fs;
#[path = "list-addressbooks.rs"]
pub mod list_addressbooks;
#[path = "read-card.rs"]
pub mod read_card;
#[path = "update-addressbook.rs"]
pub mod update_addressbook;

#[doc(inline)]
pub use self::{
    client::Client, config::Config, constants::*, create_addressbook::CreateAddressbook,
    create_card::CreateCard, delete_addressbook::DeleteAddressbook,
    list_addressbooks::ListAddressbooks, read_card::ReadCard,
    update_addressbook::UpdateAddressbook,
};
