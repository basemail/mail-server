// directory/src/basemail/lookup.rs
use mail_send::Credentials;
use store::{NamedRows, Rows, Value};

use crate::{backend::internal::manage::ManageDirectory, Principal, QueryBy, Type};

use super::BasemailDirectory;

// Note: this service is similar to the internal directory, but uses the basemail auth service instead of internally stored credentials
impl BasemailDirectory {
    pub async fn query(
        &self,
        by: QueryBy<'_>,
        return_member_of: bool,
    ) -> crate::Result<Option<Principal<u32>>> {
        let (account_id, secret) = match by {
            QueryBy::Name(name) => (self.get_account_id(name).await?, None),
            QueryBy::Id(account_id) => (account_id.into(), None),
            QueryBy::Credentials(credentials) => match credentials {
                Credentials::Plain { address, siwe_access_token } => {
                    // username = address
                    // secret = siwe_access_token
                    // The secret is an OAuth token, but we use the plain credential format 
                    // to avoid local validation of the token and to validate the address againt the one returned by the token

                    // Check that the token is valid
                    let token_address = match self.auth_service.validate(siwe_access_token).await {
                        Ok(address) => address.parse::<Address>().unwrap(),
                        Err(_) => return Ok(None),
                    };

                    // Check that the address matches the one in the token
                    if (address.parse::<Address>().unwrap() != token_address) {
                        return Ok(None);
                    }

                    // Get the account ID for the address
                    (self.get_account_id(address).await?, Some(siwe_access_token))
                }
                _ => (None, None), // OAuth and XOauth2 are not supported
            },
        };

        if let Some(account_id) = account_id {
            match self.get_value::<Principal<u32>>(ValueKey::from(ValueClass::Directory(
                DirectoryClass::Principal(account_id)
            )))
            .await {
                Some(principal) => Ok(Some(principal)),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

}