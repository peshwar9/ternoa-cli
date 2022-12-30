mod subternxt;
use crate::subternxt::counts::{get_nominator_count, get_nft_count};

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
    string: Option<String>,
}

#[derive(Args)]
struct Count {
    /// The parameter for which count is requested
    string: Option<String>,
}

enum CountOptions {
    Nfts,
    Nominators,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Parse commandline
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::State(name)) => match name.string {
            Some(ref _name) => {
                println!("Hello not implemented yet");
            }
            None => {
                println!("Please provide a parameter for which you want the State");
            }
        },
        Some(Commands::Count(name)) => match name.string {
            Some(ref name) => {
                if name == "Nominators" {
                    let nominator_count = get_nominator_count().await?;
                    println!("Nominator count = {} ", nominator_count);
                } else if name == "Nfts" {
                        let nft_count = get_nft_count().await?;
                        println!("Nft count = {} ", nft_count);
                }
                else {
                    println!("Sorry, not yet implemented");
                }
            },
            None => {
                println!("Please provide a parameter for which you want the Count");
            }
        },
        None => {
            println!("Please provide a valid parameter");
        }
    }
    Ok(())
}
