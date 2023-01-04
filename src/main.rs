mod subternxt;
use crate::subternxt::constants::{
    ALPHANET_CHAIN_URL, MAINNET_CHAIN_URL, ALPHANET_DICTIONARY_URL, MAINNET_DICTIONARY_URL, MAINNET_INDEXER_URL,
};
use crate::subternxt::{
    counts::{get_active_validators, get_nft_count, get_nominator_count, get_total_validators},
    graphql::{active_wallets::get_active_wallets, first_item_block_id::get_first_item_block_id, total_transactions::get_total_transactions},
    state::get_current_era,
};

// Clap
use ::clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Prabhu Eshwarla", version)]
#[command(
    about = "Ternoa CLI - a CLI to interact with the Ternoa chain",
    long_about = "Ternoa CLI allows you to query and perform transactions on the Ternoa chain"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short = 'n', long = "network", required = false)]
    network: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Provides block ID for major events from Ternoa chain
    Block(Block),
    /// Provides the current state of the parameter from Ternoa chain
    State(State),
    /// Provides a count of the parameter from Ternoa chain
    Count(Count),
}

#[derive(Args)]
struct Block {
    /// The parameter for which block is requested
    #[command(subcommand)]
    block_parameter: BlockParameter,
}

#[derive(Subcommand)]
enum BlockParameter {
    /// Returns the block ID of the first NFT minted
    FirstNFT,
    /// Returns the block ID of the first Collection minted
    FirstCollection,
    /// Returns the block ID of the first Marketplace created
    FirstMarketplace
}

#[derive(Args)]
struct State {
    /// The parameter for which state is requested
    #[command(subcommand)]
    state_parameter: StateParameter,
}

#[derive(Subcommand)]
enum StateParameter {
    /// Returns the current era the chain is in
    CurrentEra,
}

#[derive(Args)]
struct Count {
    /// The parameter for which count is requested
    #[command(subcommand)]
    count_parameter: CountParameter,
}

#[derive(Subcommand)]
enum CountParameter {
    /// Shows the total count of NFTs in Ternoa chain
    Nfts,
    /// Shows the total number of nominators on Ternoa chain
    Nominators,
    /// Shows the total number of validators on Ternoa chain
    TotalValidators,
    /// Shows the total number of only the active validators on Ternoa chain
    ActiveValidators,
    /// Shows the total number of active wallets from indexer (available only for the Mainnet)
    ActiveWallets,
    /// Shows the total number of transactions on chain from dictionary (available only for the Mainnet)
    TotalTransactions,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Parse commandline
    let cli = Cli::parse();

    let (network, dictionary_url) = if let Some(ref chain_name) = cli.network {
        println!("Network selected is: {}", chain_name);

        let chain_name = chain_name.to_lowercase();
        let chain_url = match chain_name.as_str() {
            "mainnet" => MAINNET_CHAIN_URL,
            "alphanet" => ALPHANET_CHAIN_URL,
            _ => &chain_name,
        };

        let dictionary_url = match chain_name.as_str() {
            "mainnet" => MAINNET_DICTIONARY_URL,
            "alphanet" => ALPHANET_DICTIONARY_URL,
            _ => &chain_name,
        };

        (chain_url.to_string(), dictionary_url.to_string())
    } else {
        (MAINNET_CHAIN_URL.to_string(), MAINNET_DICTIONARY_URL.to_string())
    };

    match &cli.command {
        Some(Commands::Block(name)) => match name.block_parameter {
            BlockParameter::FirstNFT => {
                let block_id = get_first_item_block_id(dictionary_url, "NFTCreated".to_string()).await?;
                println!("First NFT minted on block {} ", block_id);
            },
            BlockParameter::FirstCollection => {
                let block_id = get_first_item_block_id(dictionary_url, "CollectionCreated".to_string()).await?;
                println!("First Collection minted on block {} ", block_id);
            },
            BlockParameter::FirstMarketplace => {
                let block_id = get_first_item_block_id(dictionary_url, "MarketplaceCreated".to_string()).await?;
                println!("First Marketplace created on block {} ", block_id);
            }
        },
        Some(Commands::State(name)) => match name.state_parameter {
            StateParameter::CurrentEra => {
                let current_era = get_current_era(network).await?;
                println!("Current Era is  {} ", current_era);
            }
        },
        Some(Commands::Count(name)) => match name.count_parameter {
            CountParameter::Nominators => {
                let nominator_count = get_nominator_count(network).await?;
                println!("Nominator count = {} ", nominator_count);
            }
            CountParameter::Nfts => {
                let nft_count = get_nft_count(network).await?;
                println!("Nft count = {} ", nft_count);
            }
            CountParameter::TotalValidators => {
                let total_validators = get_total_validators(network).await?;
                println!("Total validators = {} ", total_validators);
            }
            CountParameter::ActiveValidators => {
                let active_validators = get_active_validators(network).await?;
                println!("Active validators = {} ", active_validators);
            }
            CountParameter::ActiveWallets => {
                let active_wallets = get_active_wallets(MAINNET_INDEXER_URL.to_string()).await?;
                println!("Active wallets on the Mainnet = {:?} ", active_wallets);
            }
            CountParameter::TotalTransactions => {
                let total_transactions =
                    get_total_transactions(MAINNET_DICTIONARY_URL.to_string()).await?;
                println!(
                    "Total transactions on the Mainnet = {:?} ",
                    total_transactions
                );
            }
        },
        None => {
            println!("Please provide a valid parameter");
        }
    }
    Ok(())
}
