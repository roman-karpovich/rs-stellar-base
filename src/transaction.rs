

use stellar_xdr::*;
use crate::hashing::hash;
use stellar_strkey::*;
use crate::operation::*;

pub struct Transaction {
    envelope_type: stellar_xdr::EnvelopeType,
    memo: stellar_xdr::Memo,
    sequence: String,
    source: String,
    time_bounds: Option<TimeBounds>,
    ledger_bounds: Option<LedgerBounds>,
    min_account_sequence: Option<String>,
    min_account_sequence_age: u32,
    min_account_sequence_ledger_gap: u32,
    extra_signers: Vec<stellar_xdr::AccountId>,
    operations: Vec<Operation>,
    network_passphrase: String,
    signatures: Vec<stellar_xdr::DecoratedSignature>,
    fee: String,
}
//TODO: Create the Transaction Builder
//TODO: Create the Transaction Object
//TODO: Write unit tests for the Transaction module
#[cfg(test)]
mod tests { 

    use super::*;
    #[test]
    fn test_build_small_tx() {
        let mut te = TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: stellar_xdr::Transaction {
                source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
                fee: 0,
                seq_num: SequenceNumber(1),
                cond: Preconditions::None,
                memo: Memo::Text("Stellar".as_bytes().try_into().unwrap()),
                operations: [].to_vec().try_into().unwrap(),
                ext: TransactionExt::V0,
            },
            signatures: [].try_into().unwrap(),
        });
        let xdr = te.to_xdr_base64().unwrap();
        assert_eq!(xdr, "AAAAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAABAAAAB1N0ZWxsYXIAAAAAAAAAAAAAAAAA");
    }
}
