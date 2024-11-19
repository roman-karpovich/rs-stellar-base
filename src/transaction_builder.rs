use std::cell::RefCell;
use std::collections::hash_map::ValuesMut;
use std::error::Error;
use std::i64;
use std::rc::Rc;
use std::str::FromStr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use hex_literal::hex;
use num_bigint::BigUint;
use serde_json::from_str;
use stellar_xdr::next::DecoratedSignature;
use stellar_xdr::next::Limits;
use stellar_xdr::next::SorobanTransactionData;
use stellar_xdr::next::StringM;

use crate::account::Account;
use crate::account::AccountBehavior;
use crate::hashing::Sha256Hasher;
use crate::keypair::Keypair;
use crate::op_list::create_account::create_account;
use crate::transaction::Transaction;
use crate::utils::decode_encode_muxed_account::decode_address_fully_to_muxed_account;
use crate::utils::decode_encode_muxed_account::decode_address_to_muxed_account;
use crate::utils::decode_encode_muxed_account::decode_address_to_muxed_account_fix_for_g_address;
use crate::utils::decode_encode_muxed_account::encode_muxed_account;
use stellar_xdr::next::LedgerBounds;
use stellar_xdr::next::Memo;
use stellar_xdr::next::MuxedAccount;
use stellar_xdr::next::Operation;
use stellar_xdr::next::Preconditions;
use stellar_xdr::next::ReadXdr;
use stellar_xdr::next::SequenceNumber;
use stellar_xdr::next::Signature;
use stellar_xdr::next::TimeBounds;
use stellar_xdr::next::TransactionEnvelope;
use stellar_xdr::next::TransactionExt;
use stellar_xdr::next::TransactionV0Envelope;
use stellar_xdr::next::TransactionV1Envelope;
use stellar_xdr::next::Uint256;
use stellar_xdr::next::VecM;
use stellar_xdr::next::WriteXdr;

#[derive(Default, Clone)]
pub struct TransactionBuilder {
    tx: Option<stellar_xdr::next::Transaction>,
    tx_v0: Option<stellar_xdr::next::TransactionV0>,
    network_passphrase: Option<String>,
    signatures: Option<Vec<DecoratedSignature>>,
    fee: Option<u32>,
    envelope_type: Option<stellar_xdr::next::EnvelopeType>,
    memo: Option<stellar_xdr::next::Memo>,
    sequence: Option<String>,
    source: Option<Rc<RefCell<Account>>>,
    time_bounds: Option<TimeBounds>,
    ledger_bounds: Option<LedgerBounds>,
    min_account_sequence: Option<String>,
    min_account_sequence_age: Option<u32>,
    min_account_sequence_ledger_gap: Option<u32>,
    extra_signers: Option<Vec<stellar_xdr::next::AccountId>>,
    operations: Option<Vec<Operation>>,
    soroban_data: Option<SorobanTransactionData>,
}

// Define a trait for TransactionBuilder behavior
pub trait TransactionBuilderBehavior {
    fn set_soroban_data_from_xdr_base64(&mut self, soroban_data: &str) -> &mut Self;
    fn new(
        source_account: Rc<RefCell<Account>>,
        network: &str,
        time_bounds: Option<TimeBounds>,
    ) -> Self;
    fn fee(&mut self, fee: impl Into<u32>) -> &mut Self;
    fn add_operation(&mut self, operation: Operation) -> &mut Self;
    fn build(&mut self) -> Transaction;
    fn add_memo(&mut self, memo_text: &str) -> &mut Self;
    fn set_timeout(&mut self, timeout_seconds: i64) -> Result<&mut Self, String>;
    fn set_time_bounds(&mut self, time_bounds: TimeBounds) -> &mut Self;
    fn set_soroban_data(&mut self, soroban_data: SorobanTransactionData) -> &mut Self;
    fn clear_operations(&mut self) -> &mut Self;
}

pub const TIMEOUT_INFINITE: i64 = 0;

impl TransactionBuilderBehavior for TransactionBuilder {
    fn new(
        source_account: Rc<RefCell<Account>>,
        network: &str,
        time_bounds: Option<TimeBounds>,
    ) -> Self {
        Self {
            tx: None,
            tx_v0: None,
            network_passphrase: Some(network.to_string()),
            signatures: None,
            fee: None,
            envelope_type: None,
            memo: None,
            sequence: None,
            source: Some(source_account),
            time_bounds,
            ledger_bounds: None,
            min_account_sequence: None,
            min_account_sequence_age: None,
            min_account_sequence_ledger_gap: None,
            extra_signers: None,
            operations: Some(Vec::new()),
            soroban_data: None,
        }
    }

    fn fee(&mut self, fee: impl Into<u32>) -> &mut Self {
        self.fee.insert(fee.into());
        self
    }

    fn add_operation(&mut self, operation: Operation) -> &mut Self {
        if let Some(ref mut vec) = self.operations {
            vec.push(operation);
        }
        self
    }

    fn add_memo(&mut self, memo_text: &str) -> &mut Self {
        self.memo = Some(stellar_xdr::next::Memo::Text(
            StringM::<28>::from_str(memo_text).unwrap(),
        ));
        self
    }

    fn set_timeout(&mut self, timeout_seconds: i64) -> Result<&mut Self, String> {
        if let Some(timebounds) = &self.time_bounds {
            if timebounds.max_time > stellar_xdr::next::TimePoint(0) {
                return Err("TimeBounds.max_time has been already set - setting timeout would overwrite it.".to_string());
            }
        }

        if timeout_seconds < 0 {
            return Err("timeout cannot be negative".to_string());
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Error getting current time: {}", e))?
            .as_secs();

        if timeout_seconds > 0 {
            let timeout_timestamp = current_time + timeout_seconds as u64;
            self.time_bounds = Some(TimeBounds {
                min_time: self
                    .time_bounds
                    .as_ref()
                    .map_or(stellar_xdr::next::TimePoint(0), |tb| tb.min_time.clone()),
                max_time: stellar_xdr::next::TimePoint(timeout_timestamp),
            });
        } else {
            self.time_bounds = Some(TimeBounds {
                min_time: stellar_xdr::next::TimePoint(0),
                max_time: stellar_xdr::next::TimePoint(0),
            });
        }

        Ok(self)
    }

    fn set_time_bounds(&mut self, time_bounds: TimeBounds) -> &mut Self {
        self.time_bounds = Some(time_bounds);
        self
    }
    
    fn set_soroban_data(&mut self, soroban_data: SorobanTransactionData) -> &mut Self {
        self.soroban_data = Some(soroban_data);
        self
    }

    fn set_soroban_data_from_xdr_base64(&mut self, soroban_data: &str) -> &mut Self {
        let data = SorobanTransactionData::from_xdr_base64(soroban_data, Limits::none()).unwrap();
        self.soroban_data = Some(data);
        self
    }

    fn clear_operations(&mut self) -> &mut Self {
        self.operations = Some(Vec::new());
        self
    }

 
    fn build(&mut self) -> Transaction {

        let source = self.source.as_ref().expect("Source account not set");
        let mut source_ref = source.borrow_mut();
        let current_seq_num = BigUint::from_str(source_ref.sequence_number().as_str()).unwrap();
        let incremented_seq_num = current_seq_num.clone() + BigUint::from(1u32);
        source_ref.increment_sequence_number();

        let fee = self
            .fee
            .unwrap()
            .checked_mul(self.operations.clone().unwrap().len().try_into().unwrap());
        let account_id = source_ref.account_id();
        let binding = hex::encode(account_id);
        let hex_val = binding.as_bytes();
        let mut array: [u8; 32] = [0; 32];
        array.copy_from_slice(&hex_val[..32]);


        let ext_on_the_fly = if self.soroban_data.is_some() {
            TransactionExt::V1(self.soroban_data.clone().unwrap())
        } else {
            TransactionExt::V0
        };


        // let old = MuxedAccount::Ed25519(Uint256::from(array));
        // let new = encode_muxed_account(account_id, &incremented_seq_num.to_string());
        // println!("Incremented {:?}", incremented_seq_num);
        let vv = decode_address_to_muxed_account_fix_for_g_address(account_id);

        let tx_obj = stellar_xdr::next::Transaction {
            source_account: vv, // MuxedAccount::Ed25519(Uint256([0; 32]))
            fee: fee.unwrap(),
            seq_num: SequenceNumber(incremented_seq_num.clone().clone().try_into().unwrap_or_else(|_| panic!("Number too large for i64"))),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: self.operations.clone().unwrap().try_into().unwrap(),
            ext: ext_on_the_fly,
        };
        Transaction {
            tx: Some(tx_obj),
            network_passphrase: self.network_passphrase.clone().unwrap(),
            signatures: Vec::new(),
            fee: fee.unwrap(),
            envelope_type: stellar_xdr::next::EnvelopeType::Tx,
            memo: None,
            sequence: Some(incremented_seq_num.clone().to_string()),
            source: Some(source_ref.account_id().to_string()),
            time_bounds: self.time_bounds.clone(),
            ledger_bounds: None,
            min_account_sequence: Some("0".to_string()),
            min_account_sequence_age: Some(0),
            min_account_sequence_ledger_gap: Some(0),
            extra_signers: Some(Vec::new()),
            operations: self.operations.clone(),
            hash: None,
            soroban_data: self.soroban_data.clone(),
            tx_v0: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use core::panic;
    use keypair::KeypairBehavior;

    use sha2::digest::crypto_common::Key;
    use stellar_xdr::next::{Hash, HostFunction, InvokeContractArgs, ScAddress, ScString, ScSymbol, ScVal};
    use stellar_xdr::next::ScAddress::Contract;

    use super::*;
    // use crate::{
    //     account::Account, asset::{Asset, AssetBehavior}, keypair::{self, Keypair}, network::{NetworkPassphrase, Networks}, operation::PaymentOpts, transaction::TransactionBehavior
    // };
    use crate::{
        account::{Account, AccountBehavior}, asset::{Asset, AssetBehavior}, contract::{ContractBehavior, Contracts}, keypair::{self, Keypair}, network::{NetworkPassphrase, Networks}, op_list::invoke_host, operation::{Operation, OperationBehavior, PaymentOpts}, soroban_data_builder::{SorobanDataBuilder, SorobanDataBuilderBehavior}, transaction::{self, TransactionBehavior}, transaction_builder::{TransactionBuilder, TransactionBuilderBehavior, TIMEOUT_INFINITE}
    };

    #[test]
    fn test_creates_and_signs() {
        let source = Rc::new(RefCell::new(
            Account::new(
                "GBBM6BKZPEHWYO3E3YKREDPQXMS4VK35YLNU7NFBRI26RAN7GI5POFBB",
                "20",
            )
            .unwrap(),
        ));

        let destination = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let signer = Keypair::master(Some(Networks::testnet())).unwrap();
        let mut tx = TransactionBuilder::new(source, Networks::testnet(), None)
            .fee(100_u32)
            .add_operation(create_account(destination, "10".to_string()).unwrap())
            .build();

        tx.sign(&[signer.clone()]);
        let sig = &tx.signatures[0].signature.0;
        let verified = signer.verify(&tx.hash(), sig);
        assert_eq!(verified, true);
    }

    #[test]
    fn test_constructs_native_payment_transaction() {
        let source = Rc::new(RefCell::new(
            Account::new(
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
                "0",
            )
            .unwrap(),
        ));

        let destination = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let amount = "1000".to_string();
        let asset = Asset::native();
        let memo = Memo::Id(100);
        let mut builder = TransactionBuilder::new(source.clone(), Networks::testnet(), None);

        builder
            .fee(100_u32)
            .add_operation(
                Operation::payment(PaymentOpts {
                    destination: destination.to_owned(),
                    asset,
                    amount: amount.to_owned(),
                    source: None,
                })
                .unwrap(),
            )
            .add_memo("100")
            .set_timeout(TIMEOUT_INFINITE)
            .unwrap();

        let transaction = builder.build();

        // Use RefCell::borrow() explicitly
        assert_eq!(transaction.source, Some(RefCell::borrow(&source).account_id().to_string()));
        assert_eq!(transaction.sequence.unwrap(),"1");
        assert_eq!(RefCell::borrow(&source).sequence_number(), "1");
        assert_eq!(transaction.operations.unwrap().len(), 1);
        assert_eq!(transaction.fee, 100);
    }

    #[test]
    fn test_constructs_native_payment_transaction_with_two_operations() {
        let source = Rc::new(RefCell::new(
            Account::new(
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
                "0",
            )
            .unwrap(),
        ));

        let destination1 = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let amount1 = "1000".to_string();
        let destination2 = "GC6ACGSA2NJGD6YWUNX2BYBL3VM4MZRSEU2RLIUZZL35NLV5IAHAX2E2".to_string();
        let amount2 = "2000".to_string();
        let asset = Asset::native();

        let mut builder = TransactionBuilder::new(source.clone(), Networks::testnet(), None);

        builder
            .fee(100_u32)
            .add_operation(
                Operation::payment(PaymentOpts {
                    destination: destination1,
                    asset: asset.clone(),
                    amount: amount1,
                    source: None,
                })
                .unwrap(),
            )
            .add_operation(
                Operation::payment(PaymentOpts {
                    destination: destination2,
                    asset,
                    amount: amount2,
                    source: None,
                })
                .unwrap(),
            )
            .set_timeout(TIMEOUT_INFINITE)
            .unwrap();

        let transaction = builder.build();

        // Use RefCell::borrow() explicitly
        assert_eq!(transaction.source, Some(RefCell::borrow(&source).account_id().to_string()));
        assert_eq!(transaction.sequence.unwrap(), "1");
        assert_eq!(RefCell::borrow(&source).sequence_number(), "1");
        assert_eq!(transaction.operations.unwrap().len(), 2);
        assert_eq!(transaction.fee, 200);
    }

    #[test]
    fn constructs_native_payment_transaction_with_custom_base_fee() {
        // Set up test data
        let source = Rc::new(RefCell::new(
            Account::new(
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
                "0",
            )
            .unwrap(),
        ));

        let destination1 = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let amount1 = "1000".to_string();
        let destination2 = "GC6ACGSA2NJGD6YWUNX2BYBL3VM4MZRSEU2RLIUZZL35NLV5IAHAX2E2".to_string();
        let amount2 = "2000".to_string();
        let asset = Asset::native();

        // Create transaction
        let mut builder = TransactionBuilder::new(source.clone(), Networks::testnet(), None);
        let transaction = builder
            .fee(1000_u32) // Set custom base fee
            .add_operation(
                Operation::payment(PaymentOpts {
                    destination: destination1,
                    asset: asset.clone(),
                    amount: amount1,
                    source: None,
                })
                .unwrap(),
            )
            .add_operation(
                Operation::payment(PaymentOpts {
                    destination: destination2,
                    asset,
                    amount: amount2,
                    source: None,
                })
                .unwrap(),
            )
            .set_timeout(TIMEOUT_INFINITE)
            .unwrap()
            .build();

        // Assert that the total fee is 2000 stroops (1000 per operation, 2 operations)
        assert_eq!(transaction.fee, 2000);
    }

    #[test]
    fn constructs_native_payment_transaction_with_integer_timebounds() {
        let source = Rc::new(RefCell::new(
            Account::new(
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
                "0",
            )
            .unwrap(),
        ));

        let timebounds = TimeBounds {
            min_time: stellar_xdr::next::TimePoint(1455287522),
            max_time: stellar_xdr::next::TimePoint(1455297545),
        };

        let mut builder = TransactionBuilder::new(
            source.clone(),
            Networks::testnet(),
            Some(timebounds.clone()),
        );
        builder.fee(100_u32).add_operation(
            Operation::payment(PaymentOpts {
                destination: "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string(),
                asset: Asset::native(),
                amount: "1000".to_string(),
                source: None,
            })
            .unwrap(),
        );

        // Set the timebounds
        builder.time_bounds = Some(timebounds.clone());

        let transaction = builder.build();

        assert_eq!(
            transaction.time_bounds.as_ref().unwrap().min_time,
            timebounds.min_time
        );
        assert_eq!(
            transaction.time_bounds.as_ref().unwrap().max_time,
            timebounds.max_time
        );
    }

    //TODO: Compatibilty of TimeBounds with chrono date
    //TODO: Soroban Data Builder

    #[test]
    fn constructs_a_transaction_with_soroban_data() {
        // Arrange
        let source = Rc::new(RefCell::new(Account::new(
            "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
            "0",
        ).unwrap()));
    
        let mut soroban_data_builder = SorobanDataBuilder::new(None);
        soroban_data_builder
            .set_resources(0, 5, 0)
            .set_refundable_fee(1);
        let soroban_transaction_data = soroban_data_builder.build();
    
        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let binding = hex::encode(contract_id);
        let hex_id = binding.as_bytes();
        let mut array = [0u8; 32];
        array.copy_from_slice(&hex_id[0..32]);
    
        let func = HostFunction::InvokeContract(InvokeContractArgs {
            contract_address: ScAddress::from(Contract(Hash::from(array))),
            function_name: ScSymbol::from(StringM::from_str("hello").unwrap()),
            args: vec![ScVal::String(ScString::from(
                StringM::from_str("world").unwrap(),
            ))]
            .try_into()
            .unwrap(),
        });
    
        // Act
        let mut transaction_builder = TransactionBuilder::new(
            source.clone(),
            Networks::testnet(),
            None,
        );
        let transaction = transaction_builder
            .fee(100_u32)
            .add_operation(Operation::invoke_host_function(func, None, None).unwrap())
            .set_soroban_data(soroban_transaction_data.clone())
            .set_timeout(TIMEOUT_INFINITE)
            .unwrap()
            .build();
    
        // Assert
        assert_eq!(transaction.soroban_data, Some(soroban_transaction_data));
        assert_eq!(transaction.operations.unwrap().len(), 1);
        assert_eq!(transaction.fee, 100);
    }


    #[test]
    fn test_set_soroban_data_from_xdr() {
        // Arrange
        let source = Rc::new(RefCell::new(
            Account::new(
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
                "0",
            )
            .unwrap(),
        ));

        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let binding = hex::encode(contract_id);
        let hex_id = binding.as_bytes();
        let mut array = [0u8; 32];
        array.copy_from_slice(&hex_id[0..32]);

        let func = HostFunction::InvokeContract(InvokeContractArgs {
            contract_address: ScAddress::from(Contract(Hash::from(array))),
            function_name: ScSymbol::from(StringM::from_str("hello").unwrap()),
            args: vec![ScVal::String(ScString::from(
                StringM::from_str("world").unwrap(),
            ))]
            .try_into()
            .unwrap(),
        });

        let mut soroban_data_builder = SorobanDataBuilder::new(None);
        soroban_data_builder
            .set_resources(0, 5, 0)
            .set_refundable_fee(1);
        let soroban_transaction_data = soroban_data_builder.build();

        // Act
        let mut transaction_builder = TransactionBuilder::new(
            source.clone(),
            Networks::testnet(),
            None,
        );
        let transaction = transaction_builder
            .fee(100_u32)
            .add_operation(Operation::invoke_host_function(func, None, None).unwrap())
            .set_soroban_data_from_xdr_base64(&(soroban_transaction_data.clone().to_xdr_base64(Limits::none()).unwrap()))
            .set_timeout(TIMEOUT_INFINITE)
            .unwrap()
            .build();

        // Assert
        assert_eq!(transaction.soroban_data, Some(soroban_transaction_data));
        assert_eq!(transaction.operations.unwrap().len(), 1);
        assert_eq!(transaction.fee, 100);
    }

    #[test]
    fn test_set_transaction_ext_when_soroban_data_present() {
        // Arrange
        let source = Rc::new(RefCell::new(
            Account::new(
                "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
                "0",
            )
            .unwrap(),
        ));

        let contract_id = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let binding = hex::encode(contract_id);
        let hex_id = binding.as_bytes();
        let mut array = [0u8; 32];
        array.copy_from_slice(&hex_id[0..32]);

        let func = HostFunction::InvokeContract(InvokeContractArgs {
            contract_address: ScAddress::from(Contract(Hash::from(array))),
            function_name: ScSymbol::from(StringM::from_str("hello").unwrap()),
            args: vec![ScVal::String(ScString::from(
                StringM::from_str("world").unwrap(),
            ))]
            .try_into()
            .unwrap(),
        });

        let mut soroban_data_builder = SorobanDataBuilder::new(None);
        soroban_data_builder
            .set_resources(0, 5, 0)
            .set_refundable_fee(1);
        let soroban_transaction_data = soroban_data_builder.build();

        // Act
        let mut transaction_builder = TransactionBuilder::new(
            source.clone(),
            Networks::testnet(),
            None,
        );
        let transaction = transaction_builder
            .fee(100_u32)
            .add_operation(Operation::invoke_host_function(func, None, None).unwrap())
            .set_soroban_data_from_xdr_base64(&(soroban_transaction_data.clone().to_xdr_base64(Limits::none()).unwrap()))
            .set_timeout(TIMEOUT_INFINITE)
            .unwrap()
            .build();

        // Assert

        let tx_env = transaction.to_envelope().unwrap();

        let val = match tx_env {
            TransactionEnvelope::Tx(transaction_v1_envelope) => transaction_v1_envelope,
            _ => panic!("Wrong"),
        };

        let inner_val = val.tx.ext;
        assert_eq!(inner_val, TransactionExt::V1(soroban_transaction_data));
    }
}
