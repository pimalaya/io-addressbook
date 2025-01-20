mod client;
pub mod response;

#[path = "addressbook-home-set.rs"]
pub mod addressbook_home_set;
#[path = "current-user-principal.rs"]
pub mod current_user_principal;

#[doc(inline)]
pub use self::{
    addressbook_home_set::AddressbookHomeSet, client::*,
    current_user_principal::CurrentUserPrincipal,
};
