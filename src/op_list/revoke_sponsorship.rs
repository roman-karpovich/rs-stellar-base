use std::str::FromStr;

use stellar_strkey::Strkey;

use crate::{
    asset::{Asset, AssetBehavior},
    operation::{self, Operation},
    xdr,
};

impl Operation {
    /// Revoke sponsorship for the `signer` on the `account`
    ///
    /// The `signer` is the Strkey representation of:
    /// - An Ed25529 public key
    /// - An Ed25519SignedPayload
    /// - A PreAuthTx
    /// - A HashX
    ///
    /// Threshold: Medium
    pub fn revoke_signer_sponsorship(
        &self,
        account: &str,
        signer: &str,
    ) -> Result<xdr::Operation, operation::Error> {
        let account_id = xdr::AccountId::from_str(account)
            .map_err(|_| operation::Error::InvalidField("account".into()))?;
        let signer_key = match Strkey::from_string(signer)
            .map_err(|_| operation::Error::InvalidField("signer".into()))?
        {
            Strkey::PublicKeyEd25519(stellar_strkey::ed25519::PublicKey(key)) => {
                xdr::SignerKey::Ed25519(xdr::Uint256(key))
            }
            Strkey::PreAuthTx(stellar_strkey::PreAuthTx(key)) => {
                xdr::SignerKey::PreAuthTx(xdr::Uint256(key))
            }
            Strkey::HashX(stellar_strkey::HashX(key)) => xdr::SignerKey::HashX(xdr::Uint256(key)),
            Strkey::SignedPayloadEd25519(stellar_strkey::ed25519::SignedPayload {
                ed25519,
                payload,
            }) => xdr::SignerKey::Ed25519SignedPayload(xdr::SignerKeyEd25519SignedPayload {
                ed25519: xdr::Uint256(ed25519),
                payload: payload
                    .try_into()
                    .map_err(|_| operation::Error::InvalidField("signer".into()))?,
            }),
            _ => return Err(operation::Error::InvalidField("signer".into())),
        };

        let body = xdr::OperationBody::RevokeSponsorship(xdr::RevokeSponsorshipOp::Signer(
            xdr::RevokeSponsorshipOpSigner {
                account_id,
                signer_key,
            },
        ));

        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }

    pub fn revoke_account_sponsorship(
        &self,
        account: &str,
    ) -> Result<xdr::Operation, operation::Error> {
        let account_id = xdr::AccountId::from_str(account)
            .map_err(|_| operation::Error::InvalidField("account".into()))?;
        let key = xdr::LedgerKey::Account(xdr::LedgerKeyAccount { account_id });
        self.revoke_ledger_key_sponsorship(key)
    }

    pub fn revoke_trustline_sponsorship(
        &self,
        account: &str,
        asset: &Asset,
    ) -> Result<xdr::Operation, operation::Error> {
        let account_id = xdr::AccountId::from_str(account)
            .map_err(|_| operation::Error::InvalidField("account".into()))?;
        let key = xdr::LedgerKey::Trustline(xdr::LedgerKeyTrustLine {
            account_id,
            asset: asset.into(),
        });
        self.revoke_ledger_key_sponsorship(key)
    }

    fn revoke_ledger_key_sponsorship(
        &self,
        key: xdr::LedgerKey,
    ) -> Result<xdr::Operation, operation::Error> {
        let body =
            xdr::OperationBody::RevokeSponsorship(xdr::RevokeSponsorshipOp::LedgerEntry(key));

        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        keypair::{Keypair, KeypairBehavior},
        operation::Operation,
    };

    #[test]
    fn test_revoke_sponsorship() {
        todo!()
    }
}
