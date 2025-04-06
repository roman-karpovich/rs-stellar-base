use crate::{
    operation::{self, Operation},
    xdr,
};

impl Operation {
    pub fn liquidity_pool_withdraw(
        &self,
        pool_id: &str,
        amount: i64,
        min_amount_a: i64,
        min_amount_b: i64,
    ) -> Result<xdr::Operation, operation::Error> {
        //
        let mut h = [0; 32];
        hex::decode_to_slice(pool_id, &mut h)
            .map_err(|_| operation::Error::InvalidField("pool_id".into()))?;
        let liquidity_pool_id = xdr::PoolId(xdr::Hash(h));
        let body = xdr::OperationBody::LiquidityPoolWithdraw(xdr::LiquidityPoolWithdrawOp {
            liquidity_pool_id,
            amount,
            min_amount_a,
            min_amount_b,
        });
        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }
}
