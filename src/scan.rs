use std::{collections::HashMap, str::FromStr};

use crate::{
    config::NETWORK,
    constants::ENetwork,
    types::{Coin, GetTransactionResponse, RequestResult, TxDetailData},
};

use num_bigint::BigInt;
use regex::Regex;
use reqwest::Client;

async fn get_historical_transactions(
    address: &String,
    timestamp: u64,
) -> RequestResult<Vec<TxDetailData>> {
    let host = match *NETWORK {
        ENetwork::Mainnet => "https://sentry.exchange.grpc-web.injective.network",
        ENetwork::Testnet => "https://testnet.sentry.exchange.grpc-web.injective.network",
    };
    let base_url = format!(
        "{}/api/explorer/v1/accountTxs/{}?end_time={}",
        host, address, timestamp
    );

    let client = Client::new();
    let mut offset = 0;
    let mut transactions: Vec<TxDetailData> = Vec::new();
    loop {
        println!("Fetching trx history at offset {}", offset);

        let url = format!("{}&skip={}", base_url, offset);
        let response = client.get(&url).send().await?;
        let trx_response: GetTransactionResponse = response.json().await?;

        transactions.extend(trx_response.data.unwrap());

        if trx_response.paging.total - offset < 100 {
            break;
        }
        offset += 100;
    }

    Ok(transactions)
}

pub async fn get_historical_balance(
    address: String,
    timestamp: u64,
) -> RequestResult<HashMap<String, BigInt>> {
    let transactions = get_historical_transactions(&address, timestamp).await?;

    let mut balances: HashMap<String, BigInt> = HashMap::new();
    let re = Regex::new(r"^(\d+)(.+)$").unwrap();

    transactions.into_iter().rev().for_each(|trx| {
        if trx.gas_fee.payer == address {
            trx.gas_fee.amount.into_iter().for_each(|amount| {
                let balance = balances.entry(amount.denom).or_insert(BigInt::from(0));
                *balance -= &BigInt::from_str(&amount.amount).unwrap();
            });
        }

        if let Some(logs) = trx.logs {
            let events: Vec<_> = logs
                .into_iter()
                .flat_map(|log| {
                    log.events.into_iter().filter(|event| {
                        (event.r#type == "transfer"
                            && event
                                .attributes
                                .iter()
                                .any(|attribute| attribute.value == address))
                            || event.r#type == "injective.exchange.v1beta1.EventSubaccountDeposit"
                    })
                })
                .collect();

            events.into_iter().for_each(|event| {
                let amount_attr = event
                    .attributes
                    .iter()
                    .find(|attr| attr.key == "amount")
                    .unwrap();

                if event.r#type == "transfer" {
                    let recipient_attr = event
                        .attributes
                        .iter()
                        .find(|attr| attr.key == "recipient")
                        .unwrap();

                    let captures = re.captures(&amount_attr.value).unwrap();
                    let amount = BigInt::from_str(captures.get(1).unwrap().as_str()).unwrap();
                    let token = captures.get(2).unwrap().as_str().to_string();
                    let balance = balances.entry(token.clone()).or_insert(BigInt::from(0));

                    if recipient_attr.value == address {
                        *balance += &amount;
                    } else {
                        *balance -= &amount;
                    }
                } else {
                    let coin: Coin = serde_json::from_str(&amount_attr.value).unwrap();
                    let balance = balances.entry(coin.denom).or_insert(BigInt::from(0));
                    *balance += &BigInt::from_str(&coin.amount).unwrap();
                }
            });
        }
    });

    Ok(balances)
}
