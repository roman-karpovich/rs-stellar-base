//! Contains passphrases for common networks

/// - `Networks::PUBLIC`: `Public Global Stellar Network ; September 2015`
/// - `Networks::TESTNET`: `Test SDF Network ; September 2015`

pub trait NetworkPassphrase {
    fn public() -> &'static str;
    fn testnet() -> &'static str;
}

#[derive(Debug)]
pub struct Networks;

impl NetworkPassphrase for Networks {
    /// Passphrase for the Public Global Stellar Network, created in September 2015.
    fn public() -> &'static str {
        "Public Global Stellar Network ; September 2015"
    }

    /// Passphrase for the Test SDF Network, created in September 2015.
    fn testnet() -> &'static str {
        "Test SDF Network ; September 2015"
    }
}
