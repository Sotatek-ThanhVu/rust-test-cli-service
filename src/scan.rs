use std::str::FromStr;

use crate::{
    config::{BSCSCAN_API_KEY, NETWORK},
    constants::ENetwork,
    errors::RequestError,
    types::{GetBlockNumberByTimestampResponse, GetTransactionResponse, RequestResult},
};

use anyhow::Ok;
use num_bigint::BigInt;
use reqwest::Client;

pub async fn get_block_number_by_timestamp(timestamp: u64) -> RequestResult<String> {
    let network = match *NETWORK {
        ENetwork::Mainnet => "api",
        ENetwork::Testnet => "api-testnet",
    };
    let url = format!(
        "https://{}.bscscan.com/api?module=block&action=getblocknobytime&timestamp={}&closest=before&apikey={}",
        network, timestamp, *BSCSCAN_API_KEY
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    let block_number = response.json::<GetBlockNumberByTimestampResponse>().await?;

    if block_number.status != "1" {
        return Err(RequestError::RequestBlockNumberFailed.into());
    }

    Ok(block_number.result)
}

pub async fn get_historical_balance(
    address: String,
    block_number: String,
) -> RequestResult<BigInt> {
    println!("{}", address);
    let network = match *NETWORK {
        ENetwork::Mainnet => "api",
        ENetwork::Testnet => "api-testnet",
    };
    let url = format!(
        "https://{}.bscscan.com/api?module=account&action=txlist&address={}&startblock=0&endblock={}&sort=asc&apikey={}",
        network, address.to_owned(), block_number, *BSCSCAN_API_KEY
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;
    let account_balance = response.json::<GetTransactionResponse>().await?;

    if account_balance.status != "1" {
        return Err(
            RequestError::RequestTransactionByAddressFailed(account_balance.message).into(),
        );
    }

    let account_balance = account_balance
        .result
        .into_iter()
        .fold(BigInt::from(0), |acc, trx| {
            let value = BigInt::from_str(&trx.value).unwrap();
            let gas_used = BigInt::from_str(&trx.gas_used).unwrap();
            let gas_price = BigInt::from_str(&trx.gas_price).unwrap();

            if address.eq(&trx.from) {
                acc - value - gas_used * gas_price
            } else {
                acc + value
            }
        });
    Ok(account_balance)
}
