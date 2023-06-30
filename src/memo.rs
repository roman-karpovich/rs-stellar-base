
//! A memo is a field that can be included in a transaction
//! 
//! It can be used to attach additional information to it.
//! It is similar to adding a note 
//! or comment to a transaction for reference purposes.

use std::{error::Error, str::FromStr};
use crate::xdr;

/// Maximum length of text memo.
pub const MAX_MEMO_TEXT_LEN: usize = 28;

/// Maximum length of hash and return memo.
pub const MAX_HASH_LEN: usize = 32;

// Memo Struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Memo {
    /// No memo
    None,
    /// Text Memo
    Text(String),
    /// Id Memo
    Id(u64),
    /// Hash Memo
    Hash([u8; 32]),
    /// Return Memo
    Return([u8; 32]),
}

impl Memo {

    pub fn new_none() -> Memo {
        Memo::None
    }

    pub fn new_id(id: u64) -> Memo {
        Memo::Id(id)
    }

    pub fn new_text<S: Into<String>>(text: S) -> Result<Memo, Box<dyn Error> > {
        let text = text.into();
        if text.len() > MAX_MEMO_TEXT_LEN {
            Err("Invalid Memo text".into())
        } else {
            Ok(Memo::Text(text))
        }
    }

    pub fn new_hash(hash: &[u8]) -> Result<Memo, Box<dyn Error> > {
        if hash.len() > MAX_HASH_LEN {
            Err("Invalid Memo Hash".into())
        } else {
            let mut memo_hash: [u8; 32] = Default::default();
            memo_hash[..hash.len()].copy_from_slice(&hash);
            Ok(Memo::Hash(memo_hash))
        }
    }

    pub fn new_return(ret: &[u8]) -> Result<Memo, Box<dyn Error>> {
        if ret.len() > MAX_HASH_LEN {
            Err("Invalid Memo Return".into())
        } else {
            let mut memo_ret: [u8; 32] = Default::default();
            memo_ret[..ret.len()].copy_from_slice(&ret);
            Ok(Memo::Return(memo_ret))
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Memo::None => true,
            _ => false,
        }
    }

    pub fn as_id(&self) -> Option<&u64> {
        match *self {
            Memo::Id(ref id) => Some(id),
            _ => None,
        }
    }

    pub fn as_id_mut(&mut self) -> Option<&mut u64> {
        match *self {
            Memo::Id(ref mut id) => Some(id),
            _ => None,
        }
    }

    pub fn is_id(&self) -> bool {
        self.as_id().is_some()
    }

    pub fn as_text(&self) -> Option<&str> {
        match *self {
            Memo::Text(ref text) => Some(text),
            _ => None,
        }
    }

    pub fn as_text_mut(&mut self) -> Option<&mut str> {
        match *self {
            Memo::Text(ref mut text) => Some(text),
            _ => None,
        }
    }

    pub fn is_text(&self) -> bool {
        self.as_text().is_some()
    }

    pub fn as_hash(&self) -> Option<&[u8; 32]> {
        match *self {
            Memo::Hash(ref hash) => Some(hash),
            _ => None,
        }
    }

    pub fn as_hash_mut(&mut self) -> Option<&mut [u8; 32]> {
        match *self {
            Memo::Hash(ref mut hash) => Some(hash),
            _ => None,
        }
    }

    pub fn is_hash(&self) -> bool {
        self.as_hash().is_some()
    }

    pub fn as_return(&self) -> Option<&[u8; 32]> {
        match *self {
            Memo::Return(ref hash) => Some(hash),
            _ => None,
        }
    }

    pub fn as_return_mut(&mut self) -> Option<&mut [u8; 32]> {
        match *self {
            Memo::Return(ref mut hash) => Some(hash),
            _ => None,
        }
    }

    pub fn is_return(&self) -> bool {
        self.as_return().is_some()
    }

    pub fn from_xdr(x: &stellar_xdr::Memo) -> Result<Memo, Box<dyn Error>> {
        match x {
            stellar_xdr::Memo::None => Ok(Memo::new_none()),
            stellar_xdr::Memo::Text(text) => Memo::new_text(text.to_string().unwrap()),
            stellar_xdr::Memo::Id(id) => Ok(Memo::new_id(id.to_owned())),
            stellar_xdr::Memo::Hash(hash) => Memo::new_hash(&hash.0),
            stellar_xdr::Memo::Return(ret) => Memo::new_return(&ret.0),
        }
    }

    pub fn to_xdr(&self) -> Result<stellar_xdr::Memo, Box<dyn Error>> {
        match self {
            Memo::None => Ok(stellar_xdr::Memo::None),
            Memo::Text(text) => Ok(stellar_xdr::Memo::Text(stellar_xdr::StringM::<28>::from_str(text.as_str()).unwrap())),
            Memo::Id(id) => Ok(stellar_xdr::Memo::Id(*id)),
            Memo::Hash(hash) => {
                let hash = stellar_xdr::Hash::from(hash.to_owned());
                Ok(stellar_xdr::Memo::Hash(hash))
            }
            Memo::Return(ret) => {
                let ret = stellar_xdr::Hash::from(ret.to_owned());
                Ok(stellar_xdr::Memo::Return(ret))
            }
        }
    }

}

impl Default for Memo {
    fn default() -> Memo {
        Memo::new_none()
    }
}

