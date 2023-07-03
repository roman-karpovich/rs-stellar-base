use std::collections::hash_map::ValuesMut;

use stellar_xdr::DecoratedSignature;
use stellar_xdr::Memo;
use stellar_xdr::Operation;
use stellar_xdr::SequenceNumber;
use stellar_xdr::Signature;
use stellar_xdr::TimeBounds;
use stellar_xdr::LedgerBounds;
use stellar_xdr::TransactionEnvelope;
use stellar_xdr::TransactionV1Envelope;
use stellar_xdr::VecM;
use stellar_xdr::WriteXdr;
use crate::transaction_base::TxBase;
use stellar_xdr::TransactionV0Envelope;
pub struct Transaction {
    tx_base: TxBase,
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
}


impl Transaction {
    pub fn new(envelope: stellar_xdr::TransactionEnvelope,
               network_passphrase: &str,
     ) -> Result<Self, Box<dyn std::error::Error>> {
        let tx_val: (String, u32, VecM<DecoratedSignature, 20>, Memo) = match envelope {
            TransactionEnvelope::Tx(TransactionV1Envelope { tx, signatures }) => (tx.to_xdr_base64().unwrap(), tx.fee, signatures, tx.memo),
            TransactionEnvelope::TxV0(TransactionV0Envelope { tx, signatures }) => (tx.to_xdr_base64().unwrap(), tx.fee, signatures, tx.memo),
            TransactionEnvelope::TxFeeBump(_) => return Err("Wrong Type".into()),
		};

        let tx_base = TxBase::new(tx_val.0,tx_val.2, tx_val.1.to_string(), network_passphrase.to_string()).unwrap();
        

        let seq_num = match envelope {
            TransactionEnvelope::Tx(TransactionV1Envelope { tx, signatures }) => tx.seq_num.0.to_string(),
            TransactionEnvelope::TxV0(TransactionV0Envelope { tx, signatures }) => tx.seq_num.0.to_string(),
            TransactionEnvelope::TxFeeBump(_) => return Err("Wrong Type".into()),
		};

        let source = match envelope {
            TransactionEnvelope::Tx(TransactionV1Envelope { tx, signatures }) => stellar_strkey::ed25519::MuxedAccount::from_string(tx.source_account.to_xdr_base64().unwrap().as_str()),
            TransactionEnvelope::TxV0(TransactionV0Envelope { tx, signatures }) =>stellar_strkey::ed25519::MuxedAccount::from_string(tx.source_account_ed25519.to_xdr_base64().unwrap().as_str()) ,
            TransactionEnvelope::TxFeeBump(_) => return Err("Wrong Type".into()),
		};


     }  
}