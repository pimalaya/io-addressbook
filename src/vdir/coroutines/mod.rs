#[path = "create-addressbook.rs"]
pub mod create_addressbook;
#[path = "create-card.rs"]
pub mod create_card;
#[path = "delete-addressbook.rs"]
pub mod delete_addressbook;
#[path = "delete-card.rs"]
pub mod delete_card;
#[path = "list-addressbooks.rs"]
pub mod list_addressbooks;
#[path = "list-cards.rs"]
pub mod list_cards;
#[path = "read-card.rs"]
pub mod read_card;
#[path = "update-addressbook.rs"]
pub mod update_addressbook;
#[path = "update-card.rs"]
pub mod update_card;

#[doc(inline)]
pub use self::{
    create_addressbook::CreateAddressbook, create_card::CreateCard,
    delete_addressbook::DeleteAddressbook, delete_card::DeleteCard,
    list_addressbooks::ListAddressbooks, list_cards::ListCards, read_card::ReadCard,
    update_addressbook::UpdateAddressbook, update_card::UpdateCard,
};
