use crate::address::Address;
use crate::asset::Asset;
use crate::keypair::Keypair;
use crate::operation;
use crate::operation::Operation;
use crate::utils::decode_encode_muxed_account::encode_muxed_account_to_address;
use crate::xdr;
use std::str::FromStr;

impl Operation {
    pub fn invoke_host_function(
        &self,
        func: xdr::HostFunction,
        auth: Option<xdr::VecM<xdr::SorobanAuthorizationEntry>>,
    ) -> Result<xdr::Operation, &'static str> {
        let auth_arr = auth.unwrap_or_default();

        let invoke_host_function_op = xdr::InvokeHostFunctionOp {
            host_function: func,
            auth: auth_arr,
        };

        let op_body = xdr::OperationBody::InvokeHostFunction(invoke_host_function_op);

        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body: op_body,
        })
    }
}

#[cfg(test)]
mod tests {
    use stellar_strkey::Strkey;

    use crate::contract::ContractBehavior;
    use crate::contract::Contracts;
    use crate::xdr::WriteXdr;

    use super::*;

    #[test]
    fn test_invoke_host_function() {
        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let id = if let Strkey::Contract(stellar_strkey::Contract(id)) =
            Strkey::from_str(contract_id).unwrap()
        {
            id
        } else {
            panic!("Fail")
        };

        let func = xdr::HostFunction::InvokeContract(xdr::InvokeContractArgs {
            contract_address: xdr::ScAddress::Contract(xdr::Hash::from(id)),
            function_name: xdr::ScSymbol::from(xdr::StringM::from_str("hello").unwrap()),
            args: vec![xdr::ScVal::String(xdr::ScString::from(
                xdr::StringM::from_str("world").unwrap(),
            ))]
            .try_into()
            .unwrap(),
        });

        let op = Operation::new()
            .invoke_host_function(func.clone(), None)
            .unwrap();

        if let xdr::OperationBody::InvokeHostFunction(f) = op.body {
            assert_eq!(f.host_function, func);
            if let xdr::HostFunction::InvokeContract(xdr::InvokeContractArgs {
                contract_address,
                function_name,
                args,
            }) = f.host_function
            {
                if let xdr::ScAddress::Contract(xdr::Hash(cid)) = contract_address {
                    assert_eq!(cid, id);
                } else {
                    panic!("Fail")
                }
            }
        }
    }
}
