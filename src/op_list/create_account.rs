use std::str::FromStr;

use crate::operation::{self, Operation};
use crate::xdr;
impl Operation {
    /// Creates and funds a new account with the specified starting balance
    /// (the `starting_balance` is in stroops)
    ///
    /// Threshold: Medium
    pub fn create_account(
        &self,
        destination: &str,
        starting_balance: i64,
    ) -> Result<xdr::Operation, operation::Error> {
        let destination = xdr::AccountId::from_str(destination)
            .map_err(|_| operation::Error::InvalidField("destination".into()))?;
        let body = xdr::OperationBody::CreateAccount(xdr::CreateAccountOp {
            destination,
            starting_balance,
        });

        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }
}
