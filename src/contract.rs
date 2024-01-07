use std::str::FromStr;

use stellar_strkey::{Contract, Strkey};
use stellar_xdr::next::{ScSymbol, InvokeHostFunctionOp, InvokeContractArgs, Hash, StringM, VecM, ScVal, SorobanAuthorizationEntry};

#[derive(Clone)]
pub struct Contracts {
    id: Vec<u8>,
}

// Define a trait with function signatures
pub trait ContractBehavior {
    fn new(contract_id: &str) -> Self;
    fn call(&self, method: &str) -> stellar_xdr::next::Operation;
}

// Implement the trait for the Contracts struct
impl ContractBehavior for Contracts {
    fn new(contract_id: &str) -> Self {
        let contract_id = Strkey::Contract(Contract::from_str(contract_id).unwrap());
        Self { id: contract_id.to_string().as_bytes().to_vec() }
    }

    fn call(&self, method: &str) -> stellar_xdr::next::Operation {
        stellar_xdr::next::Operation {
            source_account: None,
            body: stellar_xdr::next::OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function: stellar_xdr::next::HostFunction::InvokeContract(InvokeContractArgs {
                    contract_address: stellar_xdr::next::ScAddress::Contract(Hash::from_str(String::from_utf8(self.id.clone()).unwrap().as_str()).unwrap()),
                    function_name: ScSymbol::from(StringM::from_str(method).unwrap()),
                    args: VecM::<ScVal>::try_from(Vec::new()).unwrap(),
                }),
                auth: VecM::<SorobanAuthorizationEntry>::try_from(Vec::new()).unwrap()
            }),
        }
    }
}