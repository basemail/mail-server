
use ethers::{
    types::{Address, U256},
    providers::{Provider, Http},
};
use reqwest::Client;
use std::sync::Arc;
use super::basemail_account::BasemailAccount;

pub struct BasemailAuth {
    api_url: String,
    chain_id: u64,
    basemail_contract: BasemailAccount<Provider<Http>>,
}

impl BasemailAuth {

    pub fn new(api_url: String, chain_id: u64, rpc_url: String, basemail_address: Address) -> Self {
        // TODO 
        // Validate that the chain ID is supported by the database
        // Can't validate that the chain ID matches the RPC URL here because the function is not async
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url).unwrap());
        let basemail_contract = BasemailAccount::new(basemail_address, provider.clone());

        BasemailAuth {
            api_url,
            chain_id,
            basemail_contract,
        }
    }

    pub async fn validate(&self, token: &String) -> Result<Address, &'static str> {
        // Validate the token with the auth service
        // Token needs to be in json format
        let client = Client::new();
        let response = client.get(&format!("{}/validate/", self.api_url))
            .json(&token)
            .send()
            .await;
        let address = match response {
            Ok(response) => {
                if response.status() != 200 {
                    return Err("Invalid token");
                }
                // Parse the response
                match response.json::<Address>().await {
                    Ok(address) => address,
                    Err(e) => {
                        return Err("Error parsing token");
                    }
                }
            }
            Err(e) => {
                return Err("Error validating token");
            }
        };

        Ok(address)
    }

    // pub async fn get_account_ids_for_owner(&self, address: &Address) -> Result<Vec<U256>, &'static str> {
    //     // Get the accounts owned by the address
    //     match self.basemail_contract.get_accounts(*address).await {
    //         Ok(accounts) => Ok(accounts),
    //         Err(_) => Err("Error getting accounts for address"),
    //     }
    // }

    pub async fn get_account_owner(&self, token_id: &u32) -> Result<Address, &'static str> {
        // Get the owner of the account
        match self.basemail_contract.owner_of(U256::from(*token_id)).await {
            Ok(owner) => Ok(owner),
            Err(_) => Err("Error getting owner of account"),
        }
    }

    pub async fn get_account_id(&self, name: &str) -> Result<Option<u32>, &'static str> {
        // Get the account ID for the account name
        match self.basemail_contract.get_account_id(name.to_string()).await {
            Ok(account_id_u256) => {
                // Convert account ID to u32
                let account_id = account_id_u256.as_u32(); // Note: if for whatever reason more than 2^32 - 1 accounts are created, this will overflow

                if account_id == 0 {
                    return Ok(None);
                }

                Ok(Some(account_id))
            }
            Err(_) => Err("Error getting account ID for account name"),
        }
    }

    pub async fn get_account_name(&self, token_id: &u32) -> Result<String, &'static str> {
        // Get the account name for the token ID
        match self.basemail_contract.get_username(U256::from(*token_id)).await {
            Ok(account_name) => Ok(account_name),
            Err(_) => Err("Error getting account name for token ID"),
        }
    }
}

