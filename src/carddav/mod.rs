mod client;
#[path = "current-user-principal.rs"]
mod current_user_principal;

pub mod serde;

#[doc(inline)]
pub use self::{client::Client, current_user_principal::*};
