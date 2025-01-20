mod client;
pub mod response;

#[path = "addressbook-home-set.rs"]
pub mod addressbook_home_set;
#[path = "create-addressbook.rs"]
pub mod create_addressbook;
#[path = "create-card.rs"]
pub mod create_card;
#[path = "current-user-principal.rs"]
pub mod current_user_principal;
#[path = "delete-addressbook.rs"]
pub mod delete_addressbook;
#[path = "delete-card.rs"]
pub mod delete_card;
#[path = "list-addressbooks.rs"]
pub mod list_addressbooks;
#[path = "read-card.rs"]
pub mod read_card;
#[path = "update-addressbook.rs"]
pub mod update_addressbook;
#[path = "update-card.rs"]
pub mod update_card;

#[doc(inline)]
pub use self::{
    addressbook_home_set::AddressbookHomeSet,
    client::{Client, Config},
    create_addressbook::CreateAddressbook,
    create_card::CreateCard,
    current_user_principal::CurrentUserPrincipal,
    delete_addressbook::DeleteAddressbook,
    delete_card::DeleteCard,
    list_addressbooks::ListAddressbooks,
    read_card::ReadCard,
    update_addressbook::UpdateAddressbook,
    update_card::UpdateCard,
};
