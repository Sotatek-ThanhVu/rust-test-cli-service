use lazy_static::lazy_static;

use crate::constants::{EEnv, ENetwork};

lazy_static! {
    pub static ref NETWORK: ENetwork = {
        let network = EEnv::Network.get_value().to_lowercase();

        match network.as_str() {
            "mainnet" => ENetwork::Mainnet,
            "testnet" => ENetwork::Testnet,
            _ => panic!("Invalid network"),
        }
    };
}
