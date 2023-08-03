
use std::{str::FromStr, cmp::Ordering};

use crate::utils::util::trim_end;
use stellar_strkey::Strkey::{PublicKeyEd25519, self};
use stellar_xdr::{*};
use crate::keypair::Keypair;
use regex::Regex;
use stellar_xdr::Asset::CreditAlphanum4;
use stellar_xdr::WriteXdr;

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

    fn from_operation(asset_xdr: stellar_xdr::Asset) -> Result<Asset, String> {
        match asset_xdr {
            stellar_xdr::Asset::Native => Ok(Asset::native()),
            stellar_xdr::Asset::CreditAlphanum4(alpha_num_4) => {
                let anum = alpha_num_4;
                let issuer = Some(anum.issuer.0);
                let issuer = if let Some(PublicKey::PublicKeyTypeEd25519(inner)) = issuer {
                    Some(stellar_strkey::ed25519::PublicKey(inner.0).to_string())
                } else {
                    None
                };
                let code = trim_end(anum.asset_code.to_string(), '\0');
                Ok(Asset::new(&code, issuer.as_deref())?)
            }
            stellar_xdr::Asset::CreditAlphanum12(alpha_num_12) => {
                let anum = alpha_num_12;
                let issuer = Some(anum.issuer.0);
                let issuer = if let Some(PublicKey::PublicKeyTypeEd25519(inner)) = issuer {
                    Some(stellar_strkey::ed25519::PublicKey(inner.0).to_string())
                } else {
                    None
                };
                let code = trim_end(anum.asset_code.to_string(), '\0');                // let code = anum.asset_code().trim_end_matches('\0').to_string();
                Ok(Asset::new(&code, issuer.as_deref())?)
            }
            _ => Err(format!("Invalid asset type: {:?}", asset_xdr)),
        }
    }

    fn to_xdr_object(&self) -> stellar_xdr::Asset {
        self._to_xdr_object()
    }

    fn to_change_trust_xdr_object(&self) -> stellar_xdr::ChangeTrustAsset {
        self._to_change_trust_xdr_object()
    }

    fn to_trust_line_xdr_object(&self) -> stellar_xdr::TrustLineAsset {
        self._to_trustline_xdr_object()
    }

    fn _to_trustline_xdr_object(&self) -> stellar_xdr::TrustLineAsset {
        if self.is_native() {
            return stellar_xdr::TrustLineAsset::Native;
        } else {
            if self.code.len() <= 4 {
                
                let pad_length = if self.code.len() <= 4 { 4 } else { 12 };
                let padded_code = format!("{:width$}", self.code, width = pad_length).replace(" ", "\0");
                
                let addr = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                    stellar_strkey::ed25519::PublicKey::from_string(
                        &self.issuer.clone().unwrap()
                    )
                    .unwrap()
                    .0,
                )));

                let asset = stellar_xdr::TrustLineAsset::CreditAlphanum4(AlphaNum4 {
                    asset_code: AssetCode4::from_str(&padded_code).unwrap(),
                    issuer: addr.clone(),
                });

                asset
            

            } else {
                let pad_length = if self.code.len() <= 4 { 4 } else { 12 };
                let padded_code = format!("{:width$}", self.code, width = pad_length).replace(" ", "\0");

                let addr = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                    stellar_strkey::ed25519::PublicKey::from_string(
                        &self.issuer.clone().unwrap()
                    )
                    .unwrap()
                    .0,
                )));

                let asset = stellar_xdr::TrustLineAsset::CreditAlphanum12(AlphaNum12 {
                    asset_code: AssetCode12::from_str(&padded_code).unwrap(),
                    issuer: addr.clone(),
                });

                asset
            }
        }

    }

    fn _to_change_trust_xdr_object(&self) -> stellar_xdr::ChangeTrustAsset {
        if self.is_native() {
            return stellar_xdr::ChangeTrustAsset::Native;
        } else {
            if self.code.len() <= 4 {
                
                let pad_length = if self.code.len() <= 4 { 4 } else { 12 };
                let padded_code = format!("{:width$}", self.code, width = pad_length).replace(" ", "\0");
                
                let addr = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                    stellar_strkey::ed25519::PublicKey::from_string(
                        &self.issuer.clone().unwrap()
                    )
                    .unwrap()
                    .0,
                )));

                let asset = stellar_xdr::ChangeTrustAsset::CreditAlphanum4(AlphaNum4 {
                    asset_code: AssetCode4::from_str(&padded_code).unwrap(),
                    issuer: addr.clone(),
                });

                asset
            

            } else {
                let pad_length = if self.code.len() <= 4 { 4 } else { 12 };
                let padded_code = format!("{:width$}", self.code, width = pad_length).replace(" ", "\0");

                let addr = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                    stellar_strkey::ed25519::PublicKey::from_string(
                        &self.issuer.clone().unwrap()
                    )
                    .unwrap()
                    .0,
                )));

                let asset = stellar_xdr::ChangeTrustAsset::CreditAlphanum12(AlphaNum12 {
                    asset_code: AssetCode12::from_str(&padded_code).unwrap(),
                    issuer: addr.clone(),
                });

                asset
            }
        }

    }
    
    fn _to_xdr_object(&self) -> stellar_xdr::Asset {
        if self.is_native() {
            return stellar_xdr::Asset::Native;
        } else {
            if self.code.len() <= 4 {
                
                let pad_length = if self.code.len() <= 4 { 4 } else { 12 };
                
                // let padded_code = format!("{:width$}", self.code, width = pad_length).replace(" ", "\0");
                // let str_val = padded_code.as_str();
                let mut asset_code: [u8; 4] = [0; 4];

                for (i, b) in self.code.as_bytes().iter().enumerate() {
                    asset_code[i] = *b;
                }
    
                
                let addr = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                    stellar_strkey::ed25519::PublicKey::from_string(
                        &self.issuer.clone().unwrap()
                    )
                    .unwrap()
                    .0,
                )));

                // println!("Padded Code {:?}", padded_code);
                let asset = stellar_xdr::Asset::CreditAlphanum4(AlphaNum4 {
                    asset_code: AssetCode4(asset_code),
                    issuer: addr.clone(),
                });

                asset
            

            } else {
                let pad_length = if self.code.len() <= 4 { 4 } else { 12 };
                let padded_code = format!("{:width$}", self.code, width = pad_length).replace(" ", "\0");

                let addr = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
                    stellar_strkey::ed25519::PublicKey::from_string(
                        &self.issuer.clone().unwrap()
                    )
                    .unwrap()
                    .0,
                )));

                let asset = stellar_xdr::Asset::CreditAlphanum12(AlphaNum12 {
                    asset_code: AssetCode12::from_str(&padded_code).unwrap(),
                    issuer: addr.clone(),
                });

                asset
            }

        }

        
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

    fn is_native(&self) -> bool {
        self.issuer.is_none()
    }

    fn compare(asset_a: &Asset, asset_b: &Asset) -> Result<Ordering, String> {

        if asset_a.equals(asset_b) {
            return Ok(Ordering::Equal);
        }

        let xdr_a_type = asset_a.get_raw_asset_type()?;
        let xdr_b_type = asset_b.get_raw_asset_type()?;

        if xdr_a_type != xdr_b_type {
            return Ok(xdr_a_type.cmp(&xdr_b_type));
        }

        let code_compare = Self::ascii_compare(&asset_a.get_code().unwrap_or("".to_owned()), &asset_b.get_code().unwrap_or("".to_owned()));
        if code_compare != Ordering::Equal {
            return Ok(code_compare);
        }

        Ok(Self::ascii_compare(&asset_a.get_issuer().unwrap_or("".to_owned()), &asset_b.get_issuer().unwrap_or("".to_owned())))
    }

    fn get_asset_type(&self) -> String {
        match self.get_raw_asset_type() {
            Ok(stellar_xdr::AssetType::Native) => "native".to_string(),
            Ok(stellar_xdr::AssetType::CreditAlphanum4) => "credit_alphanum4".to_string(),
            Ok(stellar_xdr::AssetType::CreditAlphanum12) => "credit_alphanum12".to_string(),
            _ => "unknown".to_string(),
        }
    }

    fn get_raw_asset_type(&self) -> Result<stellar_xdr::AssetType, String> {
        if self.is_native() {
            Ok(stellar_xdr::AssetType::Native)
        } else if self.code.len() <= 4 {
            Ok(stellar_xdr::AssetType::CreditAlphanum4)
        } else {
            Ok(stellar_xdr::AssetType::CreditAlphanum12)
        }
    }

    fn equals(&self, asset: &Asset) -> bool {
        self.get_code() == asset.get_code() && self.get_issuer() == asset.get_issuer()
    }
    
    fn get_code(&self) -> Option<String> {
        Some(self.code.clone())
    }

    fn get_issuer(&self) -> Option<String> {
        self.issuer.clone()
    }

    fn to_string_asset(&self) -> String {
        if self.is_native() {
            return "native".to_string();
        }

        match (self.get_code(), self.get_issuer()) {
            (Some(code), Some(issuer)) => format!("{}:{}", code, issuer),
            _ => "".to_string(),
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::Asset;
    use stellar_xdr::WriteXdr;

    
    #[test]
    fn test_no_issuer_for_non_xlm_asset() {
        let err_val = Asset::new("USD", None).unwrap_err();
        assert_eq!(err_val, "Issuer cannot be null");
    }

    #[test]
    fn test_invalid_asset_code() {
        let err_val = Asset::new("", Some("GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ")).unwrap_err();
        assert_eq!(err_val, "Asset code is invalid (maximum alphanumeric, 12 characters at max)");
        let err_val = super::Asset::new("1234567890123", Some("GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ")).unwrap_err();
        assert_eq!(err_val, "Asset code is invalid (maximum alphanumeric, 12 characters at max)");
        let err_val = Asset::new("ab_", Some("GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ")).unwrap_err();
        assert_eq!(err_val, "Asset code is invalid (maximum alphanumeric, 12 characters at max)");

    }

    #[test]
    fn test_native_asset_code() {
        let asset = Asset::native();
        assert_eq!(asset.get_code().unwrap(), "XLM");
    }

    #[test]
    fn test_asset_code() {
        let issuer = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let asset = Asset::new("USD", Some(issuer)).unwrap();
        assert_eq!(asset.get_code().unwrap(), "USD");
    }

    #[test]
    fn test_native_asset_issuer() {
        let asset = Asset::native();
        assert!(asset.get_issuer().is_none());
    }

    #[test]
    fn test_non_native_asset_issuer() {
        let issuer = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let asset = Asset::new("USD", Some(issuer)).unwrap();
        assert_eq!(asset.get_issuer(), Some(issuer.to_string()));
    }

    #[test]
    fn test_native_asset_type() {
        let asset = Asset::native();
        assert_eq!(asset.get_asset_type(), "native");
    }

    #[test]
    fn test_credit_alphanum4_asset_type() {
        let issuer = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let asset = Asset::new("ABCD", Some(issuer)).unwrap();
        assert_eq!(asset.get_asset_type(), "credit_alphanum4");
    }

    #[test]
    fn test_credit_alphanum12_asset_type() {
        let issuer = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let asset = Asset::new("ABCDEF", Some(issuer)).unwrap();
        assert_eq!(asset.get_asset_type(), "credit_alphanum12");
    }

    #[test]
    fn test_parse_native_asset() {
        let asset = Asset::native();

        // Test toXDRObject() for Asset
        let xdr = asset.to_xdr_object();
        
        assert_eq!(
            String::from_utf8(xdr.to_xdr().unwrap()),
            String::from_utf8([0u8, 0u8, 0u8, 0u8].to_vec())
        );

        let xdr = asset.to_change_trust_xdr_object();
        assert_eq!(
            String::from_utf8(xdr.to_xdr().unwrap()),
            String::from_utf8([0u8, 0u8, 0u8, 0u8].to_vec())
        );

        // // Test toTrustLineXDRObject() for TrustLineAsset
        let xdr = asset.to_trust_line_xdr_object();
        assert_eq!(
            String::from_utf8(xdr.to_xdr().unwrap()),
            String::from_utf8([0u8, 0u8, 0u8, 0u8].to_vec())
        );
    }

    #[test]
    fn test_parse_alphanum_asset() {
        let issuer = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let asset = Asset::new("USD", Some(issuer)).unwrap();
        let xdr = asset.to_xdr_object();
        
        match xdr {
            stellar_xdr::Asset::CreditAlphanum4(x) => assert_eq!(hex::encode(x.asset_code), hex::encode("USD\0".to_string())),
            _ => panic!("Error")
        }
    }

}