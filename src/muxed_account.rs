use std::{cell::RefCell, rc::Rc};

use stellar_strkey::ed25519::PublicKey;
use crate::{account::Account, utils::decode_encode_muxed_account::{encode_muxed_account, encode_muxed_account_to_address, decode_address_to_muxed_account, extract_base_address}};
use arrayref::array_ref;

pub struct MuxedAccount {
    account: Rc<RefCell<Account>>,
    muxed_xdr: stellar_xdr::MuxedAccount,
    m_address: String,
    id: String,
}

impl MuxedAccount {
    fn new( base_account: Rc<RefCell<Account>>, id: &str) ->  Result<Self, Box<dyn std::error::Error>>  {
        let account_id = base_account.borrow().account_id().to_owned();
        
        let key = PublicKey::from_string(&account_id);

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
        let mut account = Account::new(&g_address, sequence_num).unwrap(); // Replace with the appropriate way to create an Account
        let account_rc = Rc::new(RefCell::new(account));

        let muxed_xdr = encode_muxed_account(&g_address, &id.to_string()); 
        let m_address = encode_muxed_account_to_address(&muxed_xdr); 
        // Self::new(&mut account.clone(), &id.to_string())
        Ok(Self {
            account: account_rc,
            id: id.to_string(),
            muxed_xdr,
            m_address,
        })
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


    fn base_account(&self) -> Rc<RefCell<Account>> {
        self.account.clone()
    }

    fn account_id(&self) -> &str {
        &self.m_address
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn sequence_number(&self) -> String {
        self.account.borrow().sequence_number()
    }

    fn increment_sequence_number(&mut self) {
        self.account.borrow_mut().increment_sequence_number();
    }

    fn to_xdr_object(&self) -> &stellar_xdr::MuxedAccount {
        &self.muxed_xdr
    }

    fn equals(&self, other_muxed_account: &MuxedAccount) -> bool {
        self.account.borrow().account_id() == other_muxed_account.account.borrow().account_id()
    }
    
}

#[cfg(test)]
mod tests {

    use stellar_strkey::{ed25519, Strkey};
    use crate::{utils::decode_encode_muxed_account::{encode_muxed_account, encode_muxed_account_to_address, decode_address_to_muxed_account, extract_base_address}, keypair::Keypair};    use super::*;
   
    
    #[test]
    fn test_generate_addresses() {

        let pubkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";
        let mpubkey_zero = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAAAACJUQ";
        let mpubkey_id = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAABUTGI4";

        let mut base_account = Account::new(pubkey, "1").unwrap(); 
        let base_account_rc = Rc::new(RefCell::new(base_account));

        let mut mux = MuxedAccount::new(base_account_rc.clone(), "0").expect("Error creating MuxedAccount");
        
        assert_eq!(mux.base_account().borrow().account_id(), pubkey);
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

        let mux_xdr = mux.to_xdr_object();

        let inner_mux = match mux_xdr {
            stellar_xdr::MuxedAccount::MuxedEd25519(x) => x,
            _ => panic!("Bad XDR"),
        };

        // mux.account.
        let key = PublicKey::from_string(pubkey);
        
        let vv = key.clone().unwrap().0;

        assert_eq!(inner_mux.ed25519,stellar_xdr::Uint256::from(*array_ref!(vv, 0, 32)));

        assert_eq!(
            inner_mux.id,
            stellar_xdr::Uint64::from("420".parse::<u64>().unwrap())
        );

        let encoded_address =  encode_muxed_account_to_address(mux_xdr); // Implement this function
        assert_eq!(encoded_address, mux.account_id());
    }

    #[test]
    fn test_sequence_numbers() {
        let pubkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";
        let base_account = Account::new(pubkey, "12345").unwrap(); 
        let base_account_rc = Rc::new(RefCell::new(base_account));

        let mut mux1 = MuxedAccount::new(base_account_rc.clone(), "1").unwrap();
        let mut mux2 = MuxedAccount::new(base_account_rc.clone(), "2").unwrap();

        assert_eq!(base_account_rc.borrow().sequence_number(), "12345");
        assert_eq!(mux1.sequence_number(), "12345");
        assert_eq!(mux2.sequence_number(), "12345");

        mux1.increment_sequence_number();

        // println!("Checking value {:?}",base_account.sequence_number());
        assert_eq!(base_account_rc.borrow().sequence_number(), "12346");
        assert_eq!(mux1.sequence_number(), "12346");
        assert_eq!(mux2.sequence_number(), "12346");

        mux2.increment_sequence_number();

        assert_eq!(base_account_rc.borrow().sequence_number(), "12347");
        assert_eq!(mux1.sequence_number(), "12347");
        assert_eq!(mux2.sequence_number(), "12347");

        base_account_rc.borrow_mut().increment_sequence_number();

        assert_eq!(base_account_rc.borrow().sequence_number(),  "12348");
        assert_eq!(mux1.sequence_number(),  "12348");
        assert_eq!(mux2.sequence_number(),  "12348");
    }
    
    #[test]
    fn test_virtual_accounts_creation() {
        let pubkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";
        let mpubkey_zero = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAAAACJUQ";
        let mpubkey_id = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAABUTGI4";

        let base_account = Account::new(pubkey, "12345").unwrap();
        let base_account_rc = Rc::new(RefCell::new(base_account));

        let mux1 = MuxedAccount::new(base_account_rc.clone(), "1").unwrap();

        let base_account_mux1 = mux1.base_account();
        let mut mux2 = MuxedAccount::new(base_account_mux1.clone(), "420").unwrap();

        assert_eq!(mux2.id(), "420");
        assert_eq!(mux2.account_id(), mpubkey_id);
        assert_eq!(mux2.sequence_number(), "12345");

        let base_account_mux2 = mux2.base_account();
        let mux3 = MuxedAccount::new(base_account_mux2.clone(), "3").unwrap();

        mux2.increment_sequence_number();

        assert_eq!(mux1.sequence_number(), "12346");
        assert_eq!(mux2.sequence_number(), "12346");
        assert_eq!(mux3.sequence_number(), "12346");
    }

    #[test]
    fn test_parse_m_addresses() {
        let pubkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";
        let mpubkey_zero = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAAAACJUQ";
        let mpubkey_id = "MA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAABUTGI4";
        let mux1 = MuxedAccount::from_address(mpubkey_zero, "123").unwrap();

        assert_eq!(mux1.id(), "0");
        assert_eq!(mux1.account_id(), mpubkey_zero);
        assert_eq!(mux1.base_account().borrow().account_id(), pubkey);
        assert_eq!(mux1.sequence_number(), "123");
    }
}
