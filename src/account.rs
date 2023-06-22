use num_bigint::BigUint;
use std::ops::AddAssign;
use std::str::FromStr;

use stellar_strkey::ed25519::{MuxedAccount, PublicKey};

pub struct Account {
    account_id: String,
    sequence: BigUint,
}

impl Account {
    pub fn new(account_id: &str, sequence: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let muxed_key = MuxedAccount::from_string(account_id);

        if muxed_key.is_ok() {
            return Err("accountId is an M-address; use MuxedAccount instead".into());
        }

        let key = PublicKey::from_string(account_id);

        if key.is_err() {
            return Err("accountId is invalid".into());
        }

        let sequence = BigUint::from_str(sequence)?;
        Ok(Self {
            account_id: account_id.to_owned(),
            sequence,
        })
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn sequence_number(&self) -> String {
        self.sequence.to_string()
    }

    pub fn increment_sequence_number(&mut self) {
        self.sequence.add_assign(BigUint::from(1_u32));
    }
}
