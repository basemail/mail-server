// directory/basemail/config.rs

use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use std::sync::Arc;
use utils::config::{utils::AsKey, Config};

use super::basemail_account::BasemailAccount;
use super::BasemailDirectory;

impl BasemailDirectory {
    pub async fn from_config(config: &mut Config, prefix: impl AsKey) -> Option<Self> {
        // Get required values from the config
        let prefix = prefix.as_key();
        let api_url = config
            .value_require((prefix.as_str(), "api_url"))?
            .to_string();
        let chain_id = config
            .value_require((prefix.as_str(), "chain_id"))?
            .parse::<u64>()
            .unwrap();
        let rpc_url = config
            .value_require((prefix.as_str(), "rpc_url"))?
            .to_string();
        let basemail_address = config
            .value_require((prefix.as_str(), "basemail_address"))?
            .parse::<Address>()
            .unwrap();

        // Create the RPC provider and check that the chain ID matches the provided one
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url).unwrap());
        let network_id = provider.get_chainid().await.unwrap().as_u64();
        if network_id != chain_id {
            return None;
        }

        // Create a constract instance for the basemail contract with the provider
        let basemail_contract = BasemailAccount::new(basemail_address, provider.clone());

        // Return the directory
        Some(BasemailDirectory {
            api_url,
            chain_id,
            basemail_contract,
        })
    }
}
