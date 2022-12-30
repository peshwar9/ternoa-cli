mod subternxt;
use crate::subternxt::{
    counts::{get_active_validators, get_nft_count, get_nominator_count, get_total_validators},
    state::get_current_era,
};

// Clap
use ::clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "Ternoa CLI - a CLI to interact with the Ternoa chain",
    long_about = "Ternoa CLI allows you to query and perform transactions on the Ternoa chain"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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

    match &cli.command {
        Some(Commands::State(name)) => match name.state_parameter {
            StateParameter::CurrentEra => {
                let current_era = get_current_era().await?;
                println!("Current Era is  {} ", current_era);
            }
        },
        Some(Commands::Count(name)) => match name.count_parameter {
            CountParameter::Nominators => {
                let nominator_count = get_nominator_count().await?;
                println!("Nominator count = {} ", nominator_count);
            }
            CountParameter::Nfts => {
                let nft_count = get_nft_count().await?;
                println!("Nft count = {} ", nft_count);
            }
            CountParameter::TotalValidators => {
                let total_validators = get_total_validators().await?;
                println!("Total validators = {} ", total_validators);
            }
            CountParameter::ActiveValidators => {
                let active_validators = get_active_validators().await?;
                println!("Active validators = {} ", active_validators);
            }
        },
        None => {
            println!("Please provide a valid parameter");
        }
    }
    Ok(())
}
