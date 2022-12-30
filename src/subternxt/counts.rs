use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./metadata-mainnet.scale")]
pub mod polkadot {}

pub async fn get_nominator_count(chain_url: String) -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url(chain_url).await?;

    // Get number of nominators
    let nominator_count;
    let nom_count = polkadot::storage().staking().counter_for_nominators();
    if let Some(count) = api.storage().fetch(&nom_count, None).await? {
        nominator_count = count;
    } else {
        nominator_count = 0;
    }
    Ok(nominator_count)
}

pub async fn get_nft_count(chain_url: String) -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url(chain_url).await?;

    // Get nft count
    let nft_count;
    let count_query = polkadot::storage().nft().next_nft_id();
    if let Some(count) = api.storage().fetch(&count_query, None).await? {
        nft_count = count;
    } else {
        nft_count = 0;
    }
    Ok(nft_count)
}

pub async fn get_active_validators(chain_url: String) -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url(chain_url).await?;

    // Get Active validators
    let active_validator_count;
    let count_query = polkadot::storage().staking().validator_count();
    if let Some(count) = api.storage().fetch(&count_query, None).await? {
        active_validator_count = count;
    } else {
        active_validator_count = 0;
    }
    Ok(active_validator_count)
}

pub async fn get_total_validators(chain_url: String) -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url(chain_url).await?;

    // Get All validators
    let total_validator_count;
    let count_query = polkadot::storage().staking().counter_for_validators();
    if let Some(count) = api.storage().fetch(&count_query, None).await? {
        total_validator_count = count;
    } else {
        total_validator_count = 0;
    }
    Ok(total_validator_count)
}
