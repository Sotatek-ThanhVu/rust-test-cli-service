use anyhow::Result;

pub type RequestResult<T> = Result<T>;

#[derive(Debug, serde::Deserialize)]
pub struct Coin {
    pub amount: String,
    pub denom: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: String,
    // pub index: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct Event {
    pub r#type: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Log {
    // pub msg_index: String,
    pub events: Vec<Event>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GasFee {
    pub amount: Vec<Coin>,
    // pub gas_limit: u128,
    // pub granter: String,
    pub payer: String,
}

// #[derive(Debug, serde::Deserialize)]
// pub struct MessageValue {
//     pub from_address: Option<String>,
//     pub to_address: Option<String>,
//     pub amount: Option<Vec<Coin>>,
//     pub sender: Option<String>,
//     pub contract: Option<String>,
//     pub msg: Option<String>,
//     pub funds: Option<String>,
// }

// #[derive(Debug, serde::Deserialize)]
// pub struct Message {
//     pub r#type: String,
//     pub value: MessageValue,
// }

// /// Signature wraps tx signature
// #[derive(Debug, serde::Deserialize)]
// pub struct Signature {
//     pub address: String,
//     pub pubkey: String,
//     pub sequence: u128,
//     pub signature: String,
// }

/// Paging defines the structure for required params for handling pagination
#[derive(Debug, serde::Deserialize)]
pub struct Paging {
    /// Total number of txs saved in database
    pub total: u64,
    // /// Index num
    // pub from: u32,

    // /// Index num
    // pub to: u32,
}

/// TxDetailData wraps tx data includes details information
#[derive(Debug, serde::Deserialize)]
pub struct TxDetailData {
    // pub id: String,
    // pub block_number: u128,
    // pub block_timestamp: String,
    // pub hash: String,
    // pub code: u128,
    // pub data: String,
    // pub info: String,
    // pub gas_wanted: u64,
    // pub gas_used: u64,
    pub gas_fee: GasFee,
    // pub codespace: String,
    // pub tx_type: String,
    // pub messages: Vec<Message>,
    // pub signatures: Vec<Signature>,
    // pub memo: String,
    // pub tx_number: u128,
    // /// Block timestamp in unix milli
    // pub block_unix_timestamp: u128,
    // pub error_log: String,
    pub logs: Option<Vec<Log>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetTransactionResponse {
    pub data: Option<Vec<TxDetailData>>,
    pub paging: Paging,
}
