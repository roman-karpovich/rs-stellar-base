use stellar_strkey::ed25519::PublicKey;

use crate::{account::Account, utils::decode_encode_muxed_account::encode_muxed_account};

pub struct MuxedAccount {
    account: Account,
    muxed_xdr: Vec<u8>,
    m_address: String,
    id: String,
}

impl MuxedAccount {
    fn new(base_account: Account, id: &str)  { // Result<Self, Box<dyn std::error::Error>>
        let account_id = base_account.account_id();
        
        let key = PublicKey::from_string(account_id);

        if key.is_err() {
            // return Err("accountId is invalid".into());
        }

        let muxed_xdr = encode_muxed_account(&account_id, id); // Replace with your actual encoding function
        // let m_address = encode_muxed_account_to_address(&muxed_xdr); // Replace with your actual encoding function

        // Ok(MuxedAccount {
        //     account: base_account,
        //     muxed_xdr,
        //     m_address,
        //     id: id.to_string(),
        // })
    }
}