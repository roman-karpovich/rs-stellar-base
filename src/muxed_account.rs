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
        self.muxed_xdr = muxed_xdr;

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

#[cfg(test)]
mod tests {
    use stellar_strkey::{ed25519, Strkey};
    use crate::utils::decode_encode_muxed_account::{encode_muxed_account, encode_muxed_account_to_address, decode_address_to_muxed_account, extract_base_address};    use super::*;
    fn assert_convert_roundtrip(s: &str, strkey: &Strkey) {
        let strkey_result = Strkey::from_string(s).unwrap();
        assert_eq!(&strkey_result, strkey);
        let str_result = format!("{strkey}");
        assert_eq!(s, str_result)
    }
    
    #[test]
    fn test_generate_addresses() {

        let pubkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";
        let mpubkey_zero = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAAAACJUQ";
        let mpubkey_id = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAABUTGI4";

        let mut base_account = Account::new(pubkey, "1").unwrap(); 
        let mut mux = MuxedAccount::new(base_account, "0").expect("Error creating MuxedAccount");
        
        assert_eq!(mux.base_account().account_id(), pubkey);
        assert_eq!(mux.account_id(), "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAAAACJUQ");
        assert_eq!(mux.id(), "0");

        mux.set_id("420").expect("Error setting MuxedAccount ID");
        assert_eq!(mux.id(), "420");
        assert_eq!(mux.account_id(), mpubkey_id);

        let mux_xdr = mux.to_xdr_object().discriminant();
        assert_eq!(
            mux_xdr,
            stellar_xdr::CryptoKeyType::MuxedEd25519
        );

        // let mux_xdr = mux.to_xdr_object();
        // mux.account.
        // // assert!(mux
        // //     .ed25519()
        // //     .eq(StrKey::decode_ed25519_public_key(pubkey).expect("Error decoding Ed25519 public key")));
        // // assert_eq!(
        // //     inner_mux.id(),
        // //     xdr::Uint64::from_string("420").expect("Error creating Uint64 from string")
        // // );

       let encoded_address =  encode_muxed_account_to_address(mux_xdr); // Implement this function
        assert_eq!(encoded_address, mux.account_id());
    }
    
}
