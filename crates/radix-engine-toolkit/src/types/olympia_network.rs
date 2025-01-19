pub enum OlympiaNetwork {
    Mainnet,
    Stokenet,
    Releasenet,
    RCNet,
    Milestonenet,
    Devopsnet,
    Sandpitnet,
    Localnet,
}

impl OlympiaNetwork {
    pub const fn hrp(&self) -> &str {
        match self {
            Self::Mainnet => "rdx",
            Self::Stokenet => "tdx",
            Self::Releasenet => "tdx3",
            Self::RCNet => "tdx4",
            Self::Milestonenet => "tdx5",
            Self::Devopsnet => "tdx6",
            Self::Sandpitnet => "tdx7",
            Self::Localnet => "ddx",
        }
    }
}
