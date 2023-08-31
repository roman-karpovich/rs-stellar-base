use std::str::FromStr;

use num_bigint::BigInt;
use num_traits::ToPrimitive;

const MEMO_NONE: &str = "none";
const MEMO_ID: &str = "id";
const MEMO_TEXT: &str = "text";
const MEMO_HASH: &str = "hash";
const MEMO_RETURN: &str = "return";

pub enum MemoValue {
    NoneValue,
    IdValue(String),
    TextValue(Vec<u8>),
    HashValue(Vec<u8>),
    ReturnValue(Vec<u8>),
}

#[derive(Debug)]
pub struct Memo {
    memo_type: String,
    value: Option<String>,
}

impl Memo {
    pub fn new(memo_type: &str, value: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut value_buf = None;
        match memo_type {
            MEMO_NONE => {}
            MEMO_ID => {
                Self::_validate_id_value(value.expect("Expected a value for MEMO_ID"));
            }
            MEMO_TEXT => {
                Self::_validate_text_value(value.expect("Expected a value for MEMO_TEXT"));
                if let Some(v) = value {
                    unsafe {
                        value_buf = Some(String::from_utf8_unchecked(v.into()));
                    }
                }
            }
            MEMO_HASH | MEMO_RETURN => {
                Self::_validate_hash_value(value.expect("Expected a value for MEMO_HASH or MEMO_RETURN"));
                if let Some(v) = value {
                    value_buf = Some(v.try_into().unwrap());
                }
            }
            _ => return Err("Invalid memo type".into()),
        }

        Ok(Memo {
            memo_type: memo_type.to_string(),
            value: value_buf,
        })
    }

    fn _validate_id_value(value: &str) -> Result<(), String> {
        let error = format!("Expects an int64 as a string. Got {}", value);
    
        let number = match BigInt::from_str(value) {
            Ok(num) => num,
            Err(_) => return Err(error.clone()),
        };
    
        
        if let Some(val) = number.to_i64() {
            let converted_back: BigInt = val.into();
            if converted_back != number {
                return Err(error.clone());
            }
        } else {
            return Err(error.clone());
        }
    
        Ok(())
    }

    fn _validate_text_value(value: &str) {
        let _ = stellar_xdr::Memo::Text(value.try_into().unwrap());
    }


    pub fn text(input: &str) -> Self {
        unsafe {
            Memo {
                memo_type: MEMO_TEXT.to_string(),
                value: Some(String::from_utf8_unchecked(input.into())),
            }
        }
    }

    pub fn text_buffer(input: Vec<u8>) -> Self {
        unsafe {
            Memo {
                memo_type: MEMO_TEXT.to_string(),
                value: Some(String::from_utf8_unchecked(input)),
            }
        }   
    }

    fn _validate_hash_value(value: &str) {
        unimplemented!()
    }

    pub fn none() -> Self {
        Self { memo_type: MEMO_NONE.to_owned(), value: None }
    }

    pub fn value(&self) -> Result<MemoValue, &'static str> {
        match self.memo_type.as_str() {
            MEMO_NONE => Ok(MemoValue::NoneValue),
            MEMO_ID => Ok(MemoValue::IdValue(self.value.clone().unwrap())),
            MEMO_TEXT => Ok(MemoValue::TextValue(self.value.clone().unwrap().as_bytes().to_vec())),
            MEMO_HASH | MEMO_RETURN => Ok(MemoValue::HashValue(self.value.clone().unwrap().as_bytes().to_vec())),
            _ => Err("Invalid memo type"),
        }
    }

    pub fn from_xdr_object(object: stellar_xdr::Memo) -> Result<Self, &'static str> {
        match object {
            stellar_xdr::Memo::None => Ok(Memo { memo_type: MEMO_NONE.to_owned(), value: None }),
            stellar_xdr::Memo::Text(x) => Ok(Memo { memo_type: MEMO_TEXT.to_owned(), value: Some(x.to_string().unwrap()) }),
            stellar_xdr::Memo::Id(x) => Ok(Memo { memo_type: MEMO_ID.to_owned(), value: Some(x.to_string()) }),
            stellar_xdr::Memo::Hash(x) =>  Ok(Memo { memo_type: MEMO_HASH.to_owned(), value: Some(x.to_string()) }),
            stellar_xdr::Memo::Return(x) => Ok(Memo { memo_type: MEMO_RETURN.to_owned(), value: Some(x.to_string()) }),
        }
    }

    pub fn to_xdr_object(&self) -> Option<stellar_xdr::Memo> {
        match self.memo_type.as_str() {
            MEMO_NONE => Some(stellar_xdr::Memo::None),
            // MemoType::MemoID => {
            //     let unsigned_hyper = UnsignedHyper::from_str(&self._value);
            //     Some(XDRMemo::memo_id(unsigned_hyper))
            // },
            MEMO_TEXT => Some(stellar_xdr::Memo::Text(self.value.clone().unwrap().as_str().try_into().unwrap())),
            // MemoType::MemoHash => Some(XDRMemo::memo_hash(&self._value)),
            // MemoType::MemoReturn => Some(XDRMemo::memo_return(&self._value)),
            _ => None,
        }
    }
    
}



#[cfg(test)]
mod tests {
    use core::panic;

    use stellar_xdr::WriteXdr;

    use crate::memo::MEMO_NONE;

    use super::{Memo, MEMO_TEXT};

    #[test]
    fn constructor_throws_error_when_type_is_invalid() {
        let result = Memo::new("test", None);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.err().unwrap());
        assert!(err_msg.contains("Invalid memo type"));
    }

    #[test]
    fn memo_none_converts_to_from_xdr() {
        let memo = Memo::none().to_xdr_object().unwrap();
        let base_memo = Memo::from_xdr_object(memo).unwrap();
        assert_eq!(base_memo.memo_type, MEMO_NONE);
        assert!(base_memo.value.is_none());
    }

    #[test]
    fn memo_text_returns_value_for_correct_argument() {
        
        let _ = Memo::new(MEMO_TEXT, Some("test"));

        let memo_utf8 = Memo::new(MEMO_TEXT, Some("三代之時")).unwrap();
        let val = match memo_utf8.to_xdr_object().unwrap() {
            stellar_xdr::Memo::Text(x) => x.to_string(),
           
           _ => panic!("Invalid Type"),
        };
        let b = String::from("三代之時");
        assert_eq!(val.unwrap(), b, "Memo text value does not match expected value");
    }

    #[test]
    fn returns_value_for_correct_argument_utf8() {
        let vec2: Vec<u8> = vec![0xd1];
        let expected: Vec<u8>= vec![
            // memo_text
            0x00, 0x00, 0x00, 0x01, // memo_text
            0x00, 0x00, 0x00, 0x01, // length
            0xd1, 0x00, 0x00, 0x00
        ];
        // let mut memo_text: Vec<u8> = vec![];
        let memo_text = Memo::text_buffer(vec2.clone()).to_xdr_object().unwrap().to_xdr().unwrap();

        unsafe {
            let memo_text_2 = Memo::new(MEMO_TEXT, Some(&String::from_utf8_unchecked(vec2.clone()))).unwrap().to_xdr_object().unwrap().to_xdr().unwrap();
            assert_eq!(memo_text_2, expected);

        }
        assert_eq!(memo_text, expected);
    
    }
}
