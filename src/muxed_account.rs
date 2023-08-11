use stellar_strkey::ed25519::PublicKey;

use crate::{account::Account, utils::decode_encode_muxed_account::{encode_muxed_account, encode_muxed_account_to_address, decode_address_to_muxed_account, extract_base_address}};

pub struct MuxedAccount {
    account: Account,
    muxed_xdr: stellar_xdr::MuxedAccount,
    m_address: String,
    id: String,
}

impl MuxedAccount {
    fn new(base_account: Account, id: &str) ->  Result<Self, Box<dyn std::error::Error>>  {
        let account_id = base_account.account_id();
        
        let key = PublicKey::from_string(account_id);

        if key.is_err() {
            return Err("accountId is invalid".into());
        }

        let muxed_xdr = encode_muxed_account(&account_id, id); 
        let m_address = encode_muxed_account_to_address(&muxed_xdr); 
        
        Ok(Self {
            account: base_account,
            id: id.to_string(),
            muxed_xdr,
            m_address,
        })
    }

    fn from_address(m_address: &str, sequence_num: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let muxed_account = decode_address_to_muxed_account(m_address); // Replace with your actual decoding function
        let g_address = extract_base_address(m_address)?; // Replace with your actual extraction function
        let id = muxed_account.id;

        let account = Account::new(&g_address, sequence_num).unwrap(); // Replace with the appropriate way to create an Account
        
        Self::new(account, &id.to_string())
    }

    fn set_id(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !id.chars().all(|c| c.is_digit(10)) {
            return Err("id should be a string representing a number (uint64)".into());
        }

        let val = match &self.muxed_xdr {
            stellar_xdr::MuxedAccount::MuxedEd25519(x) => x,
            _ => return Err("Bad XDR".into())
        };
        
        let muxed_xdr = stellar_xdr::MuxedAccount::MuxedEd25519(
            stellar_xdr::MuxedAccountMed25519 {
                id: id.parse::<u64>().unwrap(),
                ed25519: val.ed25519.clone(),
            }
        );
        self.m_address = encode_muxed_account_to_address(&self.muxed_xdr); // Replace with your actual encoding function
        self.id = id.to_string();

        Ok(())
    }


    fn base_account(&self) -> &Account {
        &self.account
    }

    fn account_id(&self) -> &str {
        &self.m_address
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn sequence_number(&self) -> String {
        self.account.sequence_number()
    }

    fn increment_sequence_number(&mut self) {
        self.account.increment_sequence_number();
    }

    fn to_xdr_object(&self) -> &stellar_xdr::MuxedAccount {
        &self.muxed_xdr
    }

    fn equals(&self, other_muxed_account: &MuxedAccount) -> bool {
        self.account.account_id() == other_muxed_account.account.account_id()
    }
    
}