// directory/basemail

mod auth;
mod basemail_account;
mod config;
mod lookup;

use auth::BasemailAuth;

pub struct BasemailDirectory {
    auth_service: BasemailAuth,
}