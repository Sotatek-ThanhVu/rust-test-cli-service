use anyhow::Result;

pub type RequestResult<T> = Result<T>;

#[derive(serde::Deserialize)]
pub struct GetBlockNumberByTimestampResponse {
    pub status: String,
    // pub message: String,
    pub result: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GetTransactionResult {
    // #[serde(rename = "blockNumber")]
    // pub block_number: String,
    // #[serde(rename = "timeStamp")]
    // pub timestamp: String,
    // #[serde(rename = "isError")]
    // pub is_error: String,
    pub from: String,
    // pub to: String,
    pub value: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct GetTransactionResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<GetTransactionResult>,
}
