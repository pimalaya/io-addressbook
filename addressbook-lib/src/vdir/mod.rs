mod client;
mod config;
mod constants;
#[path = "create-addressbook.rs"]
pub mod create_addressbook;
#[path = "delete-addressbook.rs"]
pub mod delete_addressbook;
pub mod fs;
#[path = "list-addressbooks.rs"]
pub mod list_addressbooks;
#[path = "update-addressbook.rs"]
pub mod update_addressbook;

#[doc(inline)]
pub use self::{
    client::Client, config::Config, constants::*, create_addressbook::CreateAddressbook,
    delete_addressbook::DeleteAddressbook, list_addressbooks::ListAddressbooks,
    update_addressbook::UpdateAddressbook,
};
