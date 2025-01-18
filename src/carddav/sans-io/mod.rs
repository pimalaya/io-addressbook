#[path = "flow-addressbook-create.rs"]
mod flow_addressbook_create;
#[path = "flow-addressbook-home-set.rs"]
mod flow_addressbook_home_set;
#[path = "flow-addressbooks-list.rs"]
mod flow_addressbooks_list;
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
    flow_addressbook_create::*, flow_addressbook_home_set::*, flow_addressbooks_list::*,
    flow_card_read::*, flow_cards_list::*, flow_contact_list::*, flow_current_user_principal::*,
};
