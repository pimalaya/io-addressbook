mod client;
pub mod response;

#[path = "addressbook-home-set.rs"]
pub mod addressbook_home_set;
#[path = "create-addressbook.rs"]
pub mod create_addressbook;
#[path = "current-user-principal.rs"]
pub mod current_user_principal;
#[path = "delete-addressbook.rs"]
pub mod delete_addressbook;
#[path = "list-addressbooks.rs"]
pub mod list_addressbooks;
#[path = "update-addressbook.rs"]
pub mod update_addressbook;

#[doc(inline)]
pub use self::{
    addressbook_home_set::AddressbookHomeSet,
    client::{Client, Config},
    create_addressbook::CreateAddressbook,
    current_user_principal::CurrentUserPrincipal,
    delete_addressbook::DeleteAddressbook,
    list_addressbooks::ListAddressbooks,
    update_addressbook::UpdateAddressbook,
};
