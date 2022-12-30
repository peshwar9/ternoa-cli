use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./metadata-mainnet.scale")]
pub mod polkadot {}

pub async fn get_nominator_count() -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://mainnet.ternoa.network:443").await?;

    // Get number of nominators
    let mut nominator_count = 0;
    let nom_count = polkadot::storage().staking().counter_for_nominators();
    if let Some(count) = api.storage().fetch(&nom_count, None).await? {
        nominator_count = count;
    } else {
        nominator_count = 0;
    }
    Ok(nominator_count)
}

pub async fn get_nft_count() -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://mainnet.ternoa.network:443").await?;

    // Get nft count
    let mut nft_count = 0;
    let count_query = polkadot::storage().nft().next_nft_id();
    if let Some(count) = api.storage().fetch(&count_query, None).await? {
        nft_count = count;
    } else {
        nft_count = 0;
    }
    Ok(nft_count)
}

pub async fn get_active_validators() -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://mainnet.ternoa.network:443").await?;

    // Get Active validators
    let mut active_validator_count = 0;
    let count_query = polkadot::storage().staking().validator_count();
    if let Some(count) = api.storage().fetch(&count_query, None).await? {
        active_validator_count = count;
    } else {
        active_validator_count = 0;
    }
    Ok(active_validator_count)
}

pub async fn get_total_validators() -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://mainnet.ternoa.network:443").await?;

    // Get All validators
    let mut total_validator_count = 0;
    let count_query = polkadot::storage().staking().counter_for_validators();
    if let Some(count) = api.storage().fetch(&count_query, None).await? {
        total_validator_count = count;
    } else {
        total_validator_count = 0;
    }
    Ok(total_validator_count)
}
