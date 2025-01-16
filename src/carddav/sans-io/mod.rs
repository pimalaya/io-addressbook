#[path = "flow-addressbook-home-set.rs"]
mod flow_addressbook_home_set;
#[path = "flow-addressbooks.rs"]
mod flow_addressbooks;
#[path = "flow-card-read.rs"]
mod flow_card_read;
#[path = "flow-cards-list.rs"]
mod flow_cards_list;
#[path = "flow-contact-list.rs"]
mod flow_contact_list;
#[path = "flow-current-user-principal.rs"]
mod flow_current_user_principal;

#[doc(inline)]
pub use self::{
    flow_addressbook_home_set::*, flow_addressbooks::*, flow_card_read::*, flow_cards_list::*,
    flow_contact_list::*, flow_current_user_principal::*,
};
