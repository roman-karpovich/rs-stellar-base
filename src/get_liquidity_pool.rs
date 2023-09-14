use std::error::Error;

use stellar_xdr::curr::{WriteXdr, LiquidityPoolParameters};

use crate::{asset::Asset, hashing::hash, liquidity_pool_asset};

// Note: you'll need to bring in equivalent Rust libraries/types for xdr, Asset, and hashing.

const LIQUIDITY_POOL_FEE_V18: i32 = 30;

/// Computes the Pool ID for the given assets, fee, and pool type.
/// Returns the raw Pool ID buffer.
pub fn get_liquidity_pool_id(
    liquidity_pool_type: &str,
    liquidity_pool_parameters: LiquidityPoolParameters,
) -> Result<Vec<u8>, Box<dyn Error>> {

    if liquidity_pool_type != "constant_product" {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "liquidityPoolType is invalid")));
    }
    let liquidity_pool_parametes_x = match liquidity_pool_parameters.clone() {
        LiquidityPoolParameters::LiquidityPoolConstantProduct(x) => x,
    };

   
    if liquidity_pool_parametes_x.fee != LIQUIDITY_POOL_FEE_V18 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "fee is invalid")));
    }

    if Asset::compare(&Asset::from_operation(liquidity_pool_parametes_x.clone().asset_a).unwrap(), &Asset::from_operation(liquidity_pool_parametes_x.clone().asset_b).unwrap()) != -1 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Assets are not in lexicographic order")));
    }
    let va_1 = liquidity_pool_parametes_x.clone().asset_a;

    let lp_type_data = stellar_xdr::curr::LiquidityPoolType::LiquidityPoolConstantProduct.to_xdr();
    let lp_params_data = stellar_xdr::curr::LiquidityPoolConstantProductParameters {
        asset_a: liquidity_pool_parametes_x.clone().asset_a,
        asset_b: liquidity_pool_parametes_x.clone().asset_b,
        fee: liquidity_pool_parametes_x.fee,
    }.to_xdr();
    
    let mut payload = Vec::new();
    payload.extend(lp_type_data.unwrap());
    payload.extend(lp_params_data.unwrap());
    
    Ok(hash(payload).to_vec())
}
