use subxt::{ext::sp_runtime::AccountId32, OnlineClient, PolkadotConfig};

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
