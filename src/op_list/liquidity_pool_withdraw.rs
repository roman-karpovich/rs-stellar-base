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

        if amount < 0 {
            return Err(operation::Error::InvalidAmount(amount));
        }
        if min_amount_a < 0 {
            return Err(operation::Error::InvalidAmount(min_amount_a));
        }
        if min_amount_b < 0 {
            return Err(operation::Error::InvalidAmount(min_amount_b));
        }

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

#[cfg(test)]
mod tests {
    use crate::{
        operation::{self, Operation},
        xdr,
    };

    #[test]
    fn test_lp_withdraw() {
        let pool_id = hex::encode([8; 32]);
        let amount = 50;
        let min_amount_a = 12 * operation::ONE;
        let min_amount_b = 40 * operation::ONE;

        let op = Operation::new()
            .liquidity_pool_withdraw(&pool_id, amount, min_amount_a, min_amount_b)
            .unwrap();

        if let xdr::OperationBody::LiquidityPoolWithdraw(xdr::LiquidityPoolWithdrawOp {
            liquidity_pool_id: xdr::PoolId(xdr::Hash(h)),
            amount,
            min_amount_a: min_a,
            min_amount_b: min_b,
        }) = op.body
        {
            assert_eq!(h, [8; 32]);
            assert_eq!(min_a, min_amount_a);
            assert_eq!(min_b, min_amount_b);
            //
        } else {
            panic!("Fail")
        }
    }
    #[test]
    fn test_lp_withdraw_bad_id() {
        let pool_id = hex::encode([8; 33]);
        let amount = 50;
        let min_amount_a = 12 * operation::ONE;
        let min_amount_b = 40 * operation::ONE;

        let op =
            Operation::new().liquidity_pool_withdraw(&pool_id, amount, min_amount_a, min_amount_b);

        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidField("pool_id".into()))
        );
    }
    #[test]
    fn test_lp_withdraw_bad_id2() {
        let pool_id = hex::encode([8; 31]);
        let amount = 50;
        let min_amount_a = 12 * operation::ONE;
        let min_amount_b = 40 * operation::ONE;

        let op =
            Operation::new().liquidity_pool_withdraw(&pool_id, amount, min_amount_a, min_amount_b);

        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidField("pool_id".into()))
        );
    }
    #[test]
    fn test_lp_withdraw_bad_amount() {
        let pool_id = hex::encode([8; 32]);
        let amount = -50;
        let min_amount_a = 12 * operation::ONE;
        let min_amount_b = 40 * operation::ONE;

        let op =
            Operation::new().liquidity_pool_withdraw(&pool_id, amount, min_amount_a, min_amount_b);

        assert_eq!(op.err(), Some(operation::Error::InvalidAmount(amount)));
    }
    #[test]
    fn test_lp_withdraw_bad_amount2() {
        let pool_id = hex::encode([8; 32]);
        let amount = 50;
        let min_amount_a = -12 * operation::ONE;
        let min_amount_b = 40 * operation::ONE;

        let op =
            Operation::new().liquidity_pool_withdraw(&pool_id, amount, min_amount_a, min_amount_b);

        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidAmount(min_amount_a))
        );
    }
    #[test]
    fn test_lp_withdraw_bad_amount3() {
        let pool_id = hex::encode([8; 32]);
        let amount = 50;
        let min_amount_a = 12 * operation::ONE;
        let min_amount_b = -40 * operation::ONE;

        let op =
            Operation::new().liquidity_pool_withdraw(&pool_id, amount, min_amount_a, min_amount_b);

        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidAmount(min_amount_b))
        );
    }
}
