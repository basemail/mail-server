
use credentials::Credentials;
use ethers::{
    types::{Address, U256},
    provider::{Provider, Http},
};
// use mongodb::Client as MongoClient;
use reqwest::Client as WebClient;
use std::sync::Arc;
use super::basemail_account::BasemailAccount;

struct BasemailAuth {
    api_url: String,
    database_uri: String,
    database_name: String,
    chain_id: u64,
    provider: Arc<Provider<Http>>,
    basemail_contract: BasemailAccount,
}

impl BasemailAuth {

    pub fn new(api_url: String, database_uri: String, database_name: String, rpc_url: String, basemail_address: Address) -> Self {
        // TODO 
        // Validate that the chain ID is supported by the database
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url).unwrap());
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let basemail_contract = BasemailAccount.new(basemail_address, provider.clone());

        BasemailAuth {
            api_url,
            database_uri,
            database_name,
            chain_id,
            basemail_contract,
        }
    }

    pub async fn validate(&self, token: String) -> Result<Address, Error> {
        // Validate the token with the auth service
        // Token needs to be in json format
        let client = WebClient::new();
        let response = client.get(&format!("{}/validate/", self.api_url))
            .json(&token)
            .send()
            .await;
        let address = match response {
            Ok(response) => {
                if response.status() != 200 {
                    return Err(Error::new("Invalid token"));
                }
                // Parse the response
                match response.json::<Address>().await {
                    Ok(address) => address,
                    Err(e) => {
                        return Err(Error::new("Error parsing token"));
                    }
                }
            }
            Err(e) => {
                return Err(Error::new("Error validating token"));
            }
        };

        Ok(address)
    }

    pub async fn get_account_ids(&self, address: Address) -> Result<Vec<U256>, Error> {
        // Create ethers provider and contract instance
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url).unwrap());
        let chain_id = provider.get_chainid().await.unwrap().as_u64();
        let basemail_contract = BasemailAccount.new(basemail_address, provider.clone());

        // Get the accounts owned by the address
        let accounts = basemail_contract.get_accounts(address).await.unwrap();

        Ok(accounts)
    }
}

