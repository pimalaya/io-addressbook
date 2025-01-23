mod client;
mod config;
#[path = "create-addressbook.rs"]
pub mod create_addressbook;
pub mod fs;
#[path = "list-addressbooks.rs"]
pub mod list_addressbooks;

#[doc(inline)]
pub use self::{
    client::Client, config::Config, create_addressbook::CreateAddressbook,
    list_addressbooks::ListAddressbooks,
};
