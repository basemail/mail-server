// directory/basemail

mod auth;
mod basemail_account;
mod config;
mod lookup;

use store::Store;
use auth::BasemailAuth;

pub struct BasemailDirectory {
    auth_service: BasemailAuth,
    pub(crate) data_store: Store,
}