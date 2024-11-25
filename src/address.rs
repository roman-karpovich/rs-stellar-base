
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
            _ => return Err("Unsupported address type")
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
        todo!()
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

    #[test]
    fn test_invalid_address_creation() {
        let result = Address::new("GBBB");
        assert!(result.is_err(), "Should fail for invalid address");
    }
}