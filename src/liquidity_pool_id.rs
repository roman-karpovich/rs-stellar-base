use regex::Regex;
use stellar_xdr::{TrustLineAsset, PoolId, ReadXdr, Hash};
use std::{error::Error, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct LiquidityPoolId {
    liquidity_pool_id: String,
}

impl LiquidityPoolId {
    pub fn new(liquidity_pool_id: &str) -> Result<Self, Box<dyn Error>> {
        if liquidity_pool_id.is_empty() {
            return Err("liquidityPoolId cannot be empty".into());
        }

        let re = Regex::new(r"^[a-f0-9]{64}$").unwrap();
        if !re.is_match(liquidity_pool_id) {
            return Err("Liquidity pool ID is not a valid hash".into());
        }

        Ok(Self {
            liquidity_pool_id: liquidity_pool_id.to_string(),
        })
    }

    pub fn from_operation(tl_asset_xdr: TrustLineAsset) -> Result<Self, &'static str> {
        match tl_asset_xdr {
            
            TrustLineAsset::PoolShare(x) => {
                let liquidity_pool_id = x.0.to_string();
                Ok(Self { liquidity_pool_id })
            }
            
            _ => panic!("Invalid type")
        }
    }

    pub fn get_asset_type(&self) -> &'static str {
        "liquidity_pool_shares"
    }

    pub fn to_xdr_object(&self) -> TrustLineAsset {
        let val = Hash::from_str(&self.liquidity_pool_id).unwrap();
        TrustLineAsset::PoolShare(PoolId(val))
    }
    
    pub fn get_liquidity_pool_id(&self) -> &str {
        &self.liquidity_pool_id
    }

    pub fn equals(&self, asset: &LiquidityPoolId) -> bool {
        self.liquidity_pool_id == asset.get_liquidity_pool_id()
    }

    pub fn to_string(&self) -> String {
        format!("liquidity_pool:{}", self.liquidity_pool_id)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throws_error_when_no_parameter_provided() {
        let x = LiquidityPoolId::new("");
        assert_eq!(x.unwrap_err().to_string(), "liquidityPoolId cannot be empty");
    }

    #[test]
    fn throws_error_when_pool_id_not_valid_hash() {
        let x = LiquidityPoolId::new("abc");
        assert_eq!(x.unwrap_err().to_string(), "Liquidity pool ID is not a valid hash");
    }

    #[test]
    fn throws_error_when_pool_id_not_all_lowercase() {
        let x = LiquidityPoolId::new(
            "DD7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7"
        );
        assert_eq!(x.unwrap_err().to_string(), "Liquidity pool ID is not a valid hash");
    }

    #[test]
    fn does_not_throw_when_pool_id_is_valid_hash() {
        let x = LiquidityPoolId::new(
                "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7");
        assert!(x.is_ok());
    }

    #[test]
    fn get_liquidity_pool_id_returns_id_of_liquidity_pool_asset() {
        let asset = LiquidityPoolId::new("dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7").unwrap();
        assert_eq!(asset.get_liquidity_pool_id(), "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7");
    }

    #[test]
    fn get_asset_type_returns_liquidity_pool_shares_for_liquidity_pool_id() {
        let asset = LiquidityPoolId::new("dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7").unwrap();
        assert_eq!(asset.get_asset_type(), "liquidity_pool_shares");
    }

    #[test]
    fn test_to_xdr_object() {
        let asset = LiquidityPoolId::new("dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7").unwrap();
        let tl_xdr = asset.to_xdr_object();

       
        let val = match tl_xdr {
            
            TrustLineAsset::PoolShare(x) => {
                let liquidity_pool_id = x.0.to_string();
                liquidity_pool_id
            }
            
            _ => panic!("Invalid type")
        };
        assert_eq!(val, "dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7");
        assert_eq!("dd7b1ab831c273310ddbec6f97870aa83c2fbd78ce22aded37ecbf4f3380fac7", asset.get_liquidity_pool_id());
    }

}
