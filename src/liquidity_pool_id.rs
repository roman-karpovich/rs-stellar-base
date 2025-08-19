use crate::asset::AssetBehavior;
use crate::xdr;
use crate::xdr::ReadXdr;
use std::{error::Error, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct LiquidityPoolId {
    liquidity_pool_id: String,
}

// Define a trait for LiquidityPoolId behavior
pub trait LiquidityPoolIdBehavior {
    fn new(liquidity_pool_id: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn from_operation(tl_asset_xdr: xdr::TrustLineAsset) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn get_asset_type(&self) -> &'static str;
    fn to_xdr_object(&self) -> xdr::TrustLineAsset;
    fn get_liquidity_pool_id(&self) -> &str;
    fn equals(&self, asset: &Self) -> bool;
    fn to_string(&self) -> String;
}

impl LiquidityPoolIdBehavior for LiquidityPoolId {
    fn new(liquidity_pool_id: &str) -> Result<Self, Box<dyn Error>> {
        if liquidity_pool_id.is_empty() {
            return Err("liquidityPoolId cannot be empty".into());
        }

        if liquidity_pool_id.len() != 64
            || !liquidity_pool_id.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err("Liquidity pool ID is not a valid hash".into());
        }

        Ok(Self {
            liquidity_pool_id: liquidity_pool_id.to_string(),
        })
    }

    fn from_operation(tl_asset_xdr: xdr::TrustLineAsset) -> Result<Self, &'static str> {
        match tl_asset_xdr {
            xdr::TrustLineAsset::PoolShare(x) => {
                let liquidity_pool_id = x.0.to_string();
                Ok(Self { liquidity_pool_id })
            }

            _ => panic!("Invalid type"),
        }
    }

    fn get_asset_type(&self) -> &'static str {
        "liquidity_pool_shares"
    }

    fn to_xdr_object(&self) -> xdr::TrustLineAsset {
        let val = xdr::Hash::from_str(&self.liquidity_pool_id).unwrap();
        xdr::TrustLineAsset::PoolShare(xdr::PoolId(val))
    }

    fn get_liquidity_pool_id(&self) -> &str {
        &self.liquidity_pool_id
    }

    fn equals(&self, asset: &LiquidityPoolId) -> bool {
        self.liquidity_pool_id == asset.get_liquidity_pool_id()
    }

    fn to_string(&self) -> String {
        format!("liquidity_pool:{}", self.liquidity_pool_id)
    }
}

#[cfg(test)]
mod tests {
    use xdr::{AlphaNum4, AssetCode4};

    use crate::{asset::Asset, keypair::Keypair};

    use super::*;

    #[test]
    fn throws_error_when_no_parameter_provided() {
        let x = LiquidityPoolId::new("");
        assert_eq!(
            x.unwrap_err().to_string(),
            "liquidityPoolId cannot be empty"
        );
    }

    #[test]
    fn throws_error_when_pool_id_not_valid_hash() {
        let x = LiquidityPoolId::new("abc");
        assert_eq!(
            x.unwrap_err().to_string(),
            "Liquidity pool ID is not a valid hash"
        );
    }

    #[test]
    fn does_not_throw_when_pool_id_is_valid_hash() {
        let x = LiquidityPoolId::new(
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7",
        );
        assert!(x.is_ok());
    }

    #[test]
    fn get_liquidity_pool_id_returns_id_of_liquidity_pool_asset() {
        let asset = LiquidityPoolId::new(
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7",
        )
        .unwrap();
        assert_eq!(
            asset.get_liquidity_pool_id(),
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7"
        );
    }

    #[test]
    fn get_asset_type_returns_liquidity_pool_shares_for_liquidity_pool_id() {
        let asset = LiquidityPoolId::new(
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7",
        )
        .unwrap();
        assert_eq!(asset.get_asset_type(), "liquidity_pool_shares");
    }

    #[test]
    fn test_to_xdr_object() {
        let asset = LiquidityPoolId::new(
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7",
        )
        .unwrap();
        let tl_xdr = asset.to_xdr_object();

        let val = match tl_xdr {
            xdr::TrustLineAsset::PoolShare(x) => x.0.to_string(),
            _ => panic!("Invalid type"),
        };
        assert_eq!(
            val,
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7"
        );
        assert_eq!(
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7",
            asset.get_liquidity_pool_id()
        );
    }

    #[test]
    #[should_panic(expected = "Invalid type")]
    fn test_invalid_asset_type() {
        let xdr = xdr::TrustLineAsset::Native;
        LiquidityPoolId::from_operation(xdr);
    }

    #[test]
    #[should_panic(expected = "Invalid type")]
    fn test_invalid_asset_type_credit_alphanum4() {
        let issuer = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let asset = Asset::new("KHL", Some(issuer)).unwrap();
        let asset_xdr = asset.to_trust_line_xdr_object();
        LiquidityPoolId::from_operation(asset_xdr);
    }

    #[test]
    #[should_panic(expected = "Invalid type")]
    fn test_invalid_asset_type_credit_alphanum12() {
        let issuer = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ";
        let asset_code = "KHLTOKEN";
        let asset = Asset::new(asset_code, Some(issuer)).unwrap();
        let asset_xdr = asset.to_trust_line_xdr_object();
        LiquidityPoolId::from_operation(asset_xdr);
    }

    #[test]
    fn test_parses_liquidity_pool_id_asset_xdr() {
        let pool_id = "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7";
        let xdr_pool_id = xdr::PoolId(xdr::Hash::from_str(pool_id).unwrap());
        let asset_xdr = xdr::TrustLineAsset::PoolShare(xdr_pool_id);
        let asset = LiquidityPoolId::from_operation(asset_xdr).unwrap();
        assert_eq!(asset.get_liquidity_pool_id(), pool_id);
        assert_eq!(asset.get_asset_type(), "liquidity_pool_shares");
    }

    #[test]
    fn test_to_string_for_liquidity_pool_assets() {
        let asset = LiquidityPoolId::new(
            "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7",
        )
        .unwrap();

        assert_eq!(
            asset.to_string(),
            "liquidity_pool:dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7"
        );
    }
}
