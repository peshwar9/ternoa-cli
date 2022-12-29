mod subternxt;
use subternxt::counts::get_nominator_count;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nominator_count = get_nominator_count().await?;
    println!("Nominator count = {} ", nominator_count);
    Ok(())
}
