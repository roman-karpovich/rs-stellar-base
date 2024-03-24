use stellar_xdr::next::*;
use crate::keypair::Keypair;
use crate::address::Address;
use crate::asset::Asset;
use crate::operation;
use crate::operation::OpAttributes;
use crate::operation::Operation;
use crate::operation::OperationBehavior;
use std::str::FromStr;

impl Operation {
    pub fn invoke_host_function(
        func: HostFunction,
        auth: Option<VecM<SorobanAuthorizationEntry>>,
        source: Option<String>,
    ) -> Result<stellar_xdr::next::Operation, &'static str> {
        let mut op = Operation {
            op_attrs: None,
            opts: None,
        };
    
        let auth_arr;

        if auth.is_none() {
            auth_arr = VecM::default();    
        } else {
            auth_arr = auth.unwrap();
        }
        let invoke_host_function_op = InvokeHostFunctionOp {
            host_function: func,
            auth: auth_arr,
        };

        let op_body = OperationBody::InvokeHostFunction(invoke_host_function_op);
       
        if source.is_none() {
            ;
        } else {
            op.set_source_account(Some(&source.unwrap()));
        }

        Ok(stellar_xdr::next::Operation { source_account: None, body: op_body })
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::ContractBehavior;
    use crate::contract::Contracts;
    use stellar_xdr::next::ScAddress::Contract;

    use super::*;

    #[test]
    fn test_invoke_host_function() {
        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        // let contract = Contracts::new(contract_id).expect("Failed to create contract");
        let binding = hex::encode(contract_id);
        let hex_id = binding.as_bytes();
        let mut array = [0u8; 32];
        array.copy_from_slice(&hex_id[0..32]);
    
        let func = HostFunction::InvokeContract(InvokeContractArgs {
            contract_address: ScAddress::from(Contract( Hash::from(array))),
            function_name: ScSymbol::from(StringM::from_str("hello").unwrap()),
            args: vec![ScVal::String(ScString::from(StringM::from_str("world").unwrap()))].try_into().unwrap(),
        });

        let op = Operation::invoke_host_function(
            func, None, None
        ).unwrap();
        
        let xdr = op.to_xdr(Limits::none()).unwrap();
        let obj = Operation::from_xdr_object(op).unwrap();
        
        match obj.get("type").unwrap() {
            operation::Value::Single(x) => assert_eq!(x, "invokeHostFunction"),
            _ => panic!("Invalid operation")
        };

    
    }
}