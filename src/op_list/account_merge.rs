use std::str::FromStr;

use crate::{
    operation::{self, Operation},
    xdr,
};

impl Operation {
    /// Transfers the XLM balance of an account to another account and removes the source account
    /// from the ledger
    ///
    /// Threshold: High
    pub fn account_merge(&self, destination: &str) -> Result<xdr::Operation, operation::Error> {
        //
        let muxed = xdr::MuxedAccount::from_str(destination)
            .map_err(|_| operation::Error::InvalidField("destination".into()))?;
        let body = xdr::OperationBody::AccountMerge(muxed);
        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{self, Operation};
    use crate::xdr::{Limits, WriteXdr};

    #[test]
    fn test_account_merge() {
        let destination = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        let result = Operation::new().account_merge(destination);
        if let Ok(op) = result {
            let xdr = op.to_xdr(Limits::none()).unwrap();
            let obj = Operation::from_xdr_object(op).unwrap();

            match obj.get("type").unwrap() {
                operation::Value::Single(x) => assert_eq!(x, "accountMerge"),
                _ => panic!("Invalid operation"),
            };
        } else {
            panic!("Fail")
        }
    }
    #[test]
    fn test_account_merge_with_source() {
        let destination = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        let source = "GAQODVWAY3AYAGEAT4CG3YSPM4FBTBB2QSXCYJLM3HVIV5ILTP5BRXCD";
        let result = Operation::with_source(source)
            .unwrap()
            .account_merge(destination);
        if let Ok(op) = result {
            let xdr = op.to_xdr(Limits::none()).unwrap();
            let obj = Operation::from_xdr_object(op).unwrap();

            match obj.get("type").unwrap() {
                operation::Value::Single(x) => assert_eq!(x, "accountMerge"),
                _ => panic!("Invalid operation"),
            };
        } else {
            panic!("Fail")
        }
    }
}
