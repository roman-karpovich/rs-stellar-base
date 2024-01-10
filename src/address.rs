use stellar_xdr::next::*;
use stellar_strkey::Strkey;

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
    fn new(address: &str) -> Result<Self, &'static str> where Self: Sized;

    /// Parses a string and returns an Address object.
    fn from_string(address: &str) -> Result<Self, &'static str> where Self: Sized;

    /// Creates a new account Address object from a buffer of raw bytes.
    fn account(buffer: &[u8]) -> Self where Self: Sized;

    /// Creates a new contract Address object from a buffer of raw bytes.
    fn contract(buffer: &[u8]) -> Self where Self: Sized;

    /// Convert from an xdr.ScVal type.
    fn from_sc_val(sc_val: &ScVal) -> Result<Self, &'static str> where Self: Sized;

    /// Convert from an xdr.ScAddress type.
    fn from_sc_address(sc_address: &ScAddress) -> Result<Self, &'static str> where Self: Sized;

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
    fn new(address: &str) -> Result<Self, &'static str> where Self: Sized {
        todo!()
    }

    fn from_string(address: &str) -> Result<Self, &'static str> where Self: Sized {
        todo!()
    }

    fn account(buffer: &[u8]) -> Self where Self: Sized {
        todo!()
    }

    fn contract(buffer: &[u8]) -> Self where Self: Sized {
        todo!()
    }

    fn from_sc_val(sc_val: &ScVal) -> Result<Self, &'static str> where Self: Sized {
        todo!()
    }

    fn from_sc_address(sc_address: &ScAddress) -> Result<Self, &'static str> where Self: Sized {
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