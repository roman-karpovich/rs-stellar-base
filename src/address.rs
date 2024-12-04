
use stellar_strkey::{ed25519::PublicKey, Contract, Strkey};
use stellar_xdr::next::*;

pub enum AddressType {
    Account,
    Contract,
}

pub struct Address {
    address_type: AddressType,
    key: Vec<u8>,
}

pub trait AddressTrait {
    /// Creates a new Address instance from a string representation.
    fn new(address: &str) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// Parses a string and returns an Address object.
    fn from_string(address: &str) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// Creates a new account Address object from a buffer of raw bytes.
    fn account(buffer: &[u8]) -> Self
    where
        Self: Sized;

    /// Creates a new contract Address object from a buffer of raw bytes.
    fn contract(buffer: &[u8]) -> Self
    where
        Self: Sized;

    /// Convert from an xdr.ScVal type.
    fn from_sc_val(sc_val: &ScVal) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// Convert from an xdr.ScAddress type.
    fn from_sc_address(sc_address: &ScAddress) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// Serialize an address to string.
    fn to_string(&self) -> String;

    /// Convert the Address to an xdr.ScVal type.
    fn to_sc_val(&self) -> Result<ScVal, &'static str>;

    /// Convert the Address to an xdr.ScAddress type.
    fn to_sc_address(&self) -> Result<ScAddress, &'static str>;

    /// Return the raw public key bytes for this address.
    fn to_buffer(&self) -> Vec<u8>;
}

impl AddressTrait for Address {
    fn new(address: &str) -> Result<Self, &'static str>
    where
        Self: Sized,
    {   
        
        let value =  match stellar_strkey::Strkey::from_string(address) {
            Ok(Strkey::PublicKeyEd25519(public_key)) => (AddressType::Account, public_key.to_string().as_bytes().to_vec()),
            Ok(Strkey::Contract(contract)) => (AddressType::Contract, contract.to_string().as_bytes().to_vec()),
            Ok(Strkey::MuxedAccountEd25519(x)) =>  return Err("Unsupported address type MuxedAccount"),
            _ => return Err("Unsupported address type"),
            
        };

        Ok(Self {
            address_type: value.0,
            key: value.1,
        })

    }
    fn from_string(address: &str) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        Self::new(address)
    }

    fn account(buffer: &[u8]) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn contract(buffer: &[u8]) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_sc_val(sc_val: &ScVal) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_sc_address(sc_address: &ScAddress) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        todo!()
    }

    fn to_string(&self) -> String {
        match &self.address_type {
            AddressType::Account => Strkey::PublicKeyEd25519(PublicKey::from_string(&String::from_utf8(self.key.clone()).expect("Invalid UTF-8 sequence")).unwrap()).to_string(),
            AddressType::Contract => Strkey::Contract(Contract::from_string(&String::from_utf8(self.key.clone()).expect("Invalid UTF-8 sequence")).unwrap()).to_string(),
        }
    }

    fn to_sc_val(&self) -> Result<ScVal, &'static str> {
        todo!()
    }

    fn to_sc_address(&self) -> Result<ScAddress, &'static str> {
        todo!()
    }

    fn to_buffer(&self) -> Vec<u8> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const ACCOUNT: &str = "GBBM6BKZPEHWYO3E3YKREDPQXMS4VK35YLNU7NFBRI26RAN7GI5POFBB";
    const CONTRACT: &str = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
    const MUXED_ADDRESS: &str = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAAAJLK";

    #[test]
    fn test_invalid_address_creation() {
        let result = Address::new("GBBB");
        assert!(result.is_err(), "Should fail for invalid address");
    }

    #[test]
    fn test_account_address_creation() {
        let account = Address::new(ACCOUNT).expect("Should create account address");
        assert_eq!(account.to_string(), ACCOUNT);
    }

    #[test]
    fn test_contract_address_creation() {
        let contract = Address::new(CONTRACT).expect("Should create contract address");
        assert_eq!(contract.to_string(), CONTRACT);
    }

    #[test]
    fn test_muxed_account_creation_fails() {
        const MUXED_ADDRESS: &str = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAAAJLK";
        
        // In Rust, this is typically done by checking for a specific error type or message
        let result = Address::new(MUXED_ADDRESS);
        assert!(result.is_err(), "Should fail for muxed account address");
        
        // Optionally, you can check the specific error message
        match result {
            Err(error_msg) => {
                assert!(error_msg.contains("MuxedAccount"), "Error should mention MuxedAccount");
            },
            _ => panic!("Should have failed for muxed account address")
        }
    }
}