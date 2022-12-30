use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./metadata-mainnet.scale")]
pub mod polkadot {}

pub async fn get_current_era(chain_url: String) -> Result<u32, Box<dyn std::error::Error>> {
    // Create API client for Ternoa chain
    let api = OnlineClient::<PolkadotConfig>::from_url(chain_url).await?;

    // Get current era
    let current_era = polkadot::storage().staking().current_era();
    if let Some(era) = api.storage().fetch(&current_era, None).await? {
        Ok(era)
    } else {
        Ok(0)
    }
}
