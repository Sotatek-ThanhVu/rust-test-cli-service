use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Request block number failed")]
    RequestBlockNumberFailed,

    #[error("Request transactions by address failed: {0}")]
    RequestTransactionByAddressFailed(String),
}
