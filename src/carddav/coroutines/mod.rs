#[path = "addressbook-home-set.rs"]
mod addressbook_home_set;
// #[path = "create-addressbook.rs"]
// mod create_addressbook;
// #[path = "create-card.rs"]
// mod create_card;
#[path = "current-user-principal.rs"]
mod current_user_principal;
// #[path = "delete-addressbook.rs"]
// mod delete_addressbook;
// #[path = "delete-card.rs"]
// mod delete_card;
#[path = "list-addressbooks.rs"]
mod list_addressbooks;
#[path = "list-cards.rs"]
mod list_cards;
// #[path = "read-card.rs"]
// mod read_card;
// #[path = "update-addressbook.rs"]
// mod update_addressbook;
// #[path = "update-card.rs"]
// mod update_card;

#[doc(inline)]
pub use self::{
    addressbook_home_set::AddressbookHomeSet,
    // create_addressbook::CreateAddressbook,
    // create_card::CreateCard,
    current_user_principal::CurrentUserPrincipal,
    // delete_addressbook::DeleteAddressbook,
    // delete_card::DeleteCard,
    list_addressbooks::ListAddressbooks,
    list_cards::ListCards,
    // read_card::ReadCard,
    // update_addressbook::UpdateAddressbook,
    // update_card::UpdateCard,
};
