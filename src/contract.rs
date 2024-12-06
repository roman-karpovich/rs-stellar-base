use core::str;
use std::str::FromStr;

use crate::address::{Address, AddressTrait};
use stellar_strkey::{Contract, Strkey};
use stellar_xdr::next::{
    ContractDataDurability, Hash, InvokeContractArgs, InvokeHostFunctionOp, LedgerKey,
    LedgerKeyContractData, Operation, ScAddress, ScSymbol, ScVal, SorobanAuthorizationEntry,
    StringM, VecM,
};

#[derive(Clone, Debug)]
pub struct Contracts {
    id: Vec<u8>,
}

pub trait ContractBehavior {
    /// Creates a new Contract instance from a string representation of the contract ID.
    fn new(contract_id: &str) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// Returns the Stellar contract ID as a string.
    fn contract_id(&self) -> String;

    /// Returns the contract ID as a string (similar to contract_id method).
    fn to_string(&self) -> String;

    /// Returns the wrapped address of this contract.
    fn address(&self) -> Address; // Address type needs to be defined.

    /// Invokes a contract call with the specified method and parameters.
    fn call(&self, method: &str, params: Option<Vec<ScVal>>) -> Operation; // Operation and ScVal types need to be defined.

    /// Returns the read-only footprint entries necessary for invocations to this contract.
    fn get_footprint(&self) -> LedgerKey; // LedgerKey type needs to be defined.
}

// Implement the trait for the Contracts struct
impl ContractBehavior for Contracts {
    fn new(contract_id: &str) -> std::result::Result<Contracts, &'static str> {
        let contract_id = Strkey::Contract(Contract::from_str(contract_id).map_err(|_| "Failed to decode contract ID")?);
        Ok(Self {
            id: contract_id.to_string().as_bytes().to_vec(),
        })
    }

    fn call(
        &self,
        method: &str,
        params: Option<Vec<stellar_xdr::next::ScVal>>,
    ) -> stellar_xdr::next::Operation {
        stellar_xdr::next::Operation {
            source_account: None,
            body: stellar_xdr::next::OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function: stellar_xdr::next::HostFunction::InvokeContract(
                    InvokeContractArgs {
                        contract_address: stellar_xdr::next::ScAddress::Contract(Hash(
                            contract_id_strkey(
                                String::from_utf8(self.id.clone()).unwrap().as_str(),
                            )
                            .0,
                        )),
                        function_name: ScSymbol::from(StringM::from_str(method).unwrap()),
                        args: VecM::<ScVal>::try_from(params.unwrap_or_default()).unwrap(),
                    },
                ),
                auth: VecM::<SorobanAuthorizationEntry>::try_from(Vec::new()).unwrap(),
            }),
        }
    }

    fn contract_id(&self) -> String {
        str::from_utf8(&self.id)
            .map(|s| s.to_string())
            .unwrap_or_else(|_| String::from(""))
    }

    fn to_string(&self) -> String {
        self.contract_id()
    }


    fn address(&self) -> Address {
        //TODO: Simplify this lol, wtf you doin
        Address::contract(&contract_id_strkey(String::from_utf8(self.id.clone()).unwrap().as_str()).0).unwrap()
    }

    fn get_footprint(&self) -> LedgerKey {
        LedgerKey::ContractData(LedgerKeyContractData {
            contract: ScAddress::Contract(Hash(contract_id_strkey(&self.contract_id()).0)),
            key: ScVal::LedgerKeyContractInstance,
            durability: ContractDataDurability::Persistent,
        })
    }
}

pub fn contract_id_strkey(contract_id: &str) -> stellar_strkey::Contract {
    stellar_strkey::Contract::from_string(contract_id).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const NULL_ADDRESS: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM";

    #[test]
    fn test_contract_constructor() {
        let test_addresses = vec![
            NULL_ADDRESS,
            "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE"
        ];

        for cid in test_addresses {
            let contract = Contracts::new(cid).expect("Failed to create contract");
            assert_eq!(contract.contract_id(), cid);
        }
    }

    #[test]
    fn test_contract_obsolete_hex_id() {
        // Create a string of 63 zeros followed by a 1
        let obsolete_hex_id = "0".repeat(63) + "1";
        
        // Test that creating a contract with this ID results in an error
        let result = Contracts::new(&obsolete_hex_id);
        
        // Assert that the result is an error
        assert!(result.is_err(), "Expected an error for obsolete hex ID");
    }

    #[test]
    fn test_contract_invalid_id() {
        // Test with an entirely invalid string
        let invalid_id = "foobar";
        
        // Test that creating a contract with this ID results in an error
        let result = Contracts::new(invalid_id);
        
        // Assert that the result is an error
        assert!(result.is_err(), "Expected an error for invalid contract ID");
    }

    #[test]
    fn test_contract_address() {
        // Create a contract using the NULL_ADDRESS
        let contract = Contracts::new(NULL_ADDRESS).expect("Failed to create contract");
        
        // Get the address and convert to string
        let address_str = contract.address().to_string();
        
        // Assert that the address string matches the original contract ID
        assert_eq!(address_str, NULL_ADDRESS, "Contract address should match the original contract ID");
    }

    
}