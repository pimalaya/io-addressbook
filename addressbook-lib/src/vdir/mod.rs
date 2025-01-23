mod client;
mod config;
pub mod fs;
#[path = "list-addressbooks.rs"]
pub mod list_addressbooks;

#[doc(inline)]
pub use self::{client::Client, config::Config, list_addressbooks::ListAddressbooks};
