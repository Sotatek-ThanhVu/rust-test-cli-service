mod config;
mod constants;
mod scan;
mod types;

use clap::Parser;
use scan::get_historical_balance;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Blockchain address to check balance
    #[arg(short, long)]
    address: String,

    /// Timestamp of query in miliseconds
    #[arg(short, long)]
    timestamp: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let args = Args::parse();

    let historical_balances = get_historical_balance(args.address, args.timestamp).await?;
    dbg!(&historical_balances);

    Ok(())
}
