use std::str::FromStr;

use stellar_strkey::{Contract, Strkey};
use stellar_xdr::next::{
    Hash, InvokeContractArgs, InvokeHostFunctionOp, ScSymbol, ScVal, SorobanAuthorizationEntry,
    StringM, VecM, LedgerKey, Operation,
};
use crate::address::Address;

#[derive(Clone)]
pub struct Contracts {
    id: Vec<u8>,
}

pub trait ContractBehavior {
    /// Creates a new Contract instance from a string representation of the contract ID.
    fn new(contract_id: &str) -> Result<Self, &'static str> where Self: Sized;

    /// Returns the Stellar contract ID as a string.
    fn contract_id(&self) -> String;

    /// Returns the contract ID as a string (similar to contract_id method).
    fn to_string(&self) -> String;

    /// Returns the wrapped address of this contract.
    fn address(&self) -> Address; // Address type needs to be defined.

    /// Invokes a contract call with the specified method and parameters.
    fn call(&self, method: &str, params: Vec<ScVal>) -> Operation; // Operation and ScVal types need to be defined.

    /// Returns the read-only footprint entries necessary for invocations to this contract.
    fn get_footprint(&self) -> LedgerKey; // LedgerKey type needs to be defined.
}

// Implement the trait for the Contracts struct
impl ContractBehavior for Contracts {
    fn new(contract_id: &str) -> std::result::Result<Contracts, &'static str> {
        let contract_id = Strkey::Contract(Contract::from_str(contract_id).unwrap());
        Ok(Self {
            id: contract_id.to_string().as_bytes().to_vec(),
        })
    }

    fn call(&self, method: &str, params: Vec<ScVal>) -> stellar_xdr::next::Operation {
        stellar_xdr::next::Operation {
            source_account: None,
            body: stellar_xdr::next::OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function: stellar_xdr::next::HostFunction::InvokeContract(
                    InvokeContractArgs {
                        contract_address: stellar_xdr::next::ScAddress::Contract(
                            Hash::from_str(String::from_utf8(self.id.clone()).unwrap().as_str())
                                .unwrap(),
                        ),
                        function_name: ScSymbol::from(StringM::from_str(method).unwrap()),
                        args: VecM::<ScVal>::try_from(Vec::new()).unwrap(),
                    },
                ),
                auth: VecM::<SorobanAuthorizationEntry>::try_from(Vec::new()).unwrap(),
            }),
        }
    }

    fn contract_id(&self) -> String {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn address(&self) -> Address {
        todo!()
    }

    fn get_footprint(&self) -> LedgerKey {
        todo!()
    }
}
