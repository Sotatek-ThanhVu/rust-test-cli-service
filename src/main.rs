mod config;
mod constants;
mod errors;
mod scan;
mod types;

use clap::Parser;
use scan::{get_block_number_by_timestamp, get_historical_balance};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Blockchain address to check balance
    #[arg(short, long)]
    address: String,

    /// Timestamp of query
    #[arg(short, long)]
    timestamp: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let args = Args::parse();
    let address = args.address.to_lowercase();

    let block_number = get_block_number_by_timestamp(args.timestamp).await?;
    let historical_balances = get_historical_balance(address, block_number).await?;
    println!("{:?}", historical_balances);

    Ok(())
}
