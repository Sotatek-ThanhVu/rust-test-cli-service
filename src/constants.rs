pub enum EEnv {
    Network,
}

impl EEnv {
    fn to_str(&self) -> &str {
        match self {
            Self::Network => "NETWORK",
        }
    }

    pub fn get_value(&self) -> String {
        let key = self.to_str();
        std::env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
    }
}

#[derive(PartialEq)]
pub enum ENetwork {
    Testnet,
    Mainnet,
}
