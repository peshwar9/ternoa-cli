mod subternxt;
use crate::subternxt::constants::{ALPHANET_URL, MAINNET_URL};
use crate::subternxt::{
    counts::{get_active_validators, get_nft_count, get_nominator_count, get_total_validators},
    state::get_current_era,
};

// Clap
use ::clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Prabhu Eshwarla", version = "0.1.0")]
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
    /// Provides the current state of the parameter from Ternoa chain
    State(State),
    /// Provides a count of the parameter from Ternoa chain
    Count(Count),
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Parse commandline
    let cli = Cli::parse();
    let network: String = if let Some(chain_name) = cli.network {
        println!("Network selected is: {}", chain_name);
        if chain_name.to_lowercase() == "mainnet" {
            MAINNET_URL.to_string()
        } else if chain_name.to_lowercase() == "alphanet" {
            ALPHANET_URL.to_string()
        } else {
            chain_name
        }
    } else {
        MAINNET_URL.to_string()
    };

    match &cli.command {
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
        },
        None => {
            println!("Please provide a valid parameter");
        }
    }
    Ok(())
}
