//! Contains passphrases for common networks

/// - `Networks::PUBLIC`: `Public Global Stellar Network ; September 2015`
/// - `Networks::TESTNET`: `Test SDF Network ; September 2015`
#[derive(Debug)]
pub struct Networks;

impl Networks {
    /// Passphrase for the Public Global Stellar Network, created in September 2015.
    pub const PUBLIC: &'static str = "Public Global Stellar Network ; September 2015";

    /// Passphrase for the Test SDF Network, created in September 2015.
    pub const TESTNET: &'static str = "Test SDF Network ; September 2015";
}