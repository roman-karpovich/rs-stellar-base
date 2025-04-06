use crate::{
    liquidity_pool_id::{self, LiquidityPoolId, LiquidityPoolIdBehavior},
    operation::{self, Operation},
    xdr,
};

impl Operation {
    pub fn liquidity_pool_deposit(
        &self,
        pool_id: &str,
        max_amount_a: i64,
        max_amount_b: i64,
        min_price: (i32, i32),
        max_price: (i32, i32),
    ) -> Result<xdr::Operation, operation::Error> {
        //
        let mut h = [0; 32];
        hex::decode_to_slice(pool_id, &mut h)
            .map_err(|_| operation::Error::InvalidField("pool_id".into()))?;
        let liquidity_pool_id = xdr::PoolId(xdr::Hash(h));

        if max_amount_a < 0 {
            return Err(operation::Error::InvalidAmount(max_amount_a));
        }
        if max_amount_b < 0 {
            return Err(operation::Error::InvalidAmount(max_amount_b));
        }

        if min_price.0 <= 0 || min_price.1 <= 0 {
            return Err(operation::Error::InvalidPrice(min_price.0, min_price.1));
        }
        if max_price.0 <= 0 || max_price.1 <= 0 {
            return Err(operation::Error::InvalidPrice(max_price.0, max_price.1));
        }

        let body = xdr::OperationBody::LiquidityPoolDeposit(xdr::LiquidityPoolDepositOp {
            liquidity_pool_id,
            max_amount_a,
            max_amount_b,
            min_price: xdr::Price {
                n: min_price.0,
                d: min_price.1,
            },
            max_price: xdr::Price {
                n: max_price.0,
                d: max_price.1,
            },
        });

        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }
}
