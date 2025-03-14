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

#[cfg(test)]
mod tests {

    use crate::{
        account::Account,
        keypair::{self, Keypair},
    };
    use keypair::KeypairBehavior;
    use stellar_strkey::Strkey;
    use xdr::{Int64, OperationBody, ReadXdr};

    use super::*;

    #[test]
    fn create_account_op_test() {
        let destination = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let starting_balance = 1000 * operation::ONE;

        let op = Operation::new()
            .create_account(destination, starting_balance)
            .unwrap();

        if let OperationBody::CreateAccount(op) = op.body {
            assert_eq!(op.starting_balance, starting_balance);
            let xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(pk))) =
                op.destination;
            if let Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(from_pk)) =
                Strkey::from_str(destination).unwrap()
            {
                assert_eq!(pk, from_pk);
            } else {
                panic!("Fail")
            }
        } else {
            panic!("op is not the type expected");
        }
    }
}
