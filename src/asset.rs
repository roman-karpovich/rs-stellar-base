
use std::{str::FromStr, cmp::Ordering};

use crate::utils::util::trim_end;
use stellar_strkey::Strkey::{PublicKeyEd25519, self};
use stellar_xdr::*;
use crate::keypair::Keypair;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Asset {
    pub code: String,
    pub issuer: Option<String>,
}

impl Asset {
    pub fn new(code: &str, issuer: Option<&str>) -> Result<Self, String> {
        if !Regex::new(r"^[a-zA-Z0-9]{1,12}$").unwrap().is_match(code) {
            return Err(
                "Asset code is invalid (maximum alphanumeric, 12 characters at max)".to_string(),
            );
        }
        if code.to_lowercase() != "xlm" && issuer.is_none() {
            return Err("Issuer cannot be null".to_string());
        }
        if let Some(issuer) = issuer {
                if Strkey::from_str(issuer).is_err() {
                    return Err("Not a valid ed25519 public key".to_string())
                }
        }

        let code = if code.to_lowercase() == "xlm" {
            "XLM".to_string()
        } else {
            code.to_string()
        };

        Ok(Self {
            code,
            issuer: issuer.map(String::from),
        })
    }

    fn ascii_compare(a: &str, b: &str) -> Ordering {
        let a_uppercase = a.to_ascii_uppercase();
        let b_uppercase = b.to_ascii_uppercase();
        a_uppercase.as_bytes().cmp(b_uppercase.as_bytes())
    }

    fn native() -> Self {
        // The native asset in Stellar is represented by the code 'XLM' with no issuer.
        Self {
            code: "XLM".to_string(),
            issuer: None,
        }
    }
    
}