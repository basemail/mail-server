// directory/src/basemail/lookup.rs
use super::BasemailDirectory;
use crate::{DirectoryError, Principal, QueryBy, Type};
use mail_send::Credentials;

const DOMAIN: &str = "basechain.email";
const QUOTA: u64 = 1000000000; // 1 GB

// Note: this service is similar to the internal directory, but uses the basemail auth service instead of internally stored credentials
impl BasemailDirectory {
    pub async fn query(&self, by: QueryBy<'_>) -> crate::Result<Option<Principal<u32>>> {
        let account_id = match by {
            QueryBy::Name(name) => {
                // Get the account ID for the account name
                match self.get_account_id(name).await {
                    Ok(Some(account_id)) => {
                        if account_id == 0 {
                            return Err(DirectoryError::Basemail("Account not found".to_string()));
                        }
                        Some(account_id)
                    }
                    Ok(None) => {
                        return Err(DirectoryError::Basemail("Account not found".to_string()))
                    }
                    Err(_) => {
                        return Err(DirectoryError::Basemail(
                            "Error getting account ID for account name".to_string(),
                        ))
                    }
                }
            }
            QueryBy::Id(account_id) => account_id.into(),
            QueryBy::Credentials(credentials) => match credentials {
                Credentials::Plain { username, secret } => {
                    // username = token_id
                    // Parse from string
                    let token_id = match username.parse::<u32>() {
                        Ok(token_id) => token_id,
                        Err(_) => {
                            return Err(DirectoryError::Basemail(
                                "Error parsing token ID".to_string(),
                            ))
                        }
                    };

                    // secret = siwe_access_token
                    // The secret is an OAuth token, but we use the plain credential format
                    // to avoid local validation of the token and to validate the address againt the one returned by the token

                    // Check that the token is valid, if so return the token ID as the account ID
                    // otherwise, return an error message
                    match self.validate(token_id, secret).await {
                        Ok(valid) => {
                            if valid {
                                Some(token_id)
                            } else {
                                return Err(DirectoryError::Basemail(
                                    "Invalid credentials".to_string(),
                                ));
                            }
                        }
                        Err(e) => {
                            return Err(DirectoryError::Basemail(e.to_string()));
                        }
                    }
                }
                _ => return Ok(None), // OAuth and XOauth2 are not supported
            },
        };

        // Construct the principal from the account_id
        if let Some(account_id) = account_id {
            // All accounts have some standard parameters
            let account_type: Type = Type::Individual;
            let secrets = vec![]; // No secrets are stored because the token is provided and compared against the auth service
            let member_of: Vec<u32> = vec![]; // No groups are stored because the auth service does not provide group information

            // Lookup the current name of the account from the auth service
            let name = match self.get_account_name(&account_id).await {
                Ok(name) => name,
                Err(_) => {
                    return Err(DirectoryError::Basemail(
                        "Error getting account name".to_string(),
                    ))
                }
            };

            // The user's primary email address is the token ID and is aliased to the username
            // Using a static domain for now
            let emails = vec![
                format!("{}@{}", account_id, DOMAIN),
                format!("{}@{}", name, DOMAIN),
            ];

            // Construct the principal
            Ok(Some(Principal {
                id: account_id,
                typ: account_type,
                quota: QUOTA,
                name,
                emails,
                secrets,
                member_of,
                description: None,
            }))
        } else {
            Err(DirectoryError::Basemail("Account not found".to_string()))
        }
    }

    pub async fn email_to_ids(&self, email: &str) -> crate::Result<Vec<u32>> {
        // Check that the email is in the correct domain
        if !email.ends_with(DOMAIN) {
            return Err(DirectoryError::Basemail("Domain not supported".to_string()));
        }

        // Get the email name could be either the account (token) ID or the username
        // Need to handle both cases
        // There cannot be conflicts because the username must start with a letter, not a number
        let email_name = match email.split('@').next() {
            Some(email_name) => email_name,
            None => return Err(DirectoryError::Basemail("Error parsing email".to_string())),
        };

        // Try to parse the email name as an account ID
        match email.parse::<u32>() {
            Ok(account_id) => {
                if account_id == 0 {
                    // If the account ID is 0, return an empty list
                    return Ok(vec![]);
                } else {
                    return Ok(vec![account_id]);
                }
            }
            Err(_) => {} // continue
        };

        // If we cannot parse the email name as an account ID, try to get the account ID from the account name
        match self.get_account_id(&email_name).await {
            Ok(Some(account_id)) => {
                if account_id == 0 {
                    // If the account ID is 0, return an empty list
                    Ok(vec![])
                } else {
                    Ok(vec![account_id])
                }
            }
            _ => {
                return Err(DirectoryError::Basemail(
                    "Error getting account ID for email".to_string(),
                ))
            }
        }
    }

    pub async fn is_local_domain(&self, domain: &str) -> crate::Result<bool> {
        Ok(domain == DOMAIN)
    }

    pub async fn rcpt(&self, email: &str) -> crate::Result<bool> {
        // Get the account ID for the email
        let account_ids = self.email_to_ids(email).await?;

        // Check that the account ID is valid
        if account_ids.is_empty() {
            return Ok(false);
        }

        // There should only be one account ID
        let account_id = account_ids[0];

        // Check that the account ID is not 0
        Ok(account_id != 0)
    }

    pub async fn vrfy(&self, address: &str) -> crate::Result<Vec<String>> {
        // Split username and domain from the email address
        let (username, domain) = match address.split_once('@') {
            Some((username, domain)) => (username, domain),
            None => return Err(DirectoryError::Basemail("Error parsing email".to_string())),
        };

        // If the domain is not the local domain, return an empty list
        if domain != DOMAIN {
            return Ok(vec![]);
        }

        // Check if an account ID belongs to the username
        // If so, return the email address, otherwise return an empty list
        match self.get_account_id(username).await {
            Ok(Some(account_id)) => {
                if account_id == 0 {
                    Ok(vec![])
                } else {
                    Ok(vec![format!("{}@{}", username, DOMAIN)])
                }
            }
            _ => {
                return Err(DirectoryError::Basemail(
                    "Error getting account ID for email".to_string(),
                ))
            }
        }
    }

    pub async fn expn(&self, _: &str) -> crate::Result<Vec<String>> {
        // mailing lists are not supported right now
        Ok(vec![])
    }
}
