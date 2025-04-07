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

#[cfg(test)]
mod tests {
    use serde::de::IntoDeserializer;

    use crate::{
        operation::{self, Operation},
        xdr,
    };

    #[test]
    fn test_lp_deposit() {
        let pool_id = hex::encode([8; 32]);
        let max_amount_a = 12 * operation::ONE;
        let max_amount_b = 40 * operation::ONE;
        let min_price = (10, 30);
        let max_price = (15, 30);
        let op = Operation::new()
            .liquidity_pool_deposit(&pool_id, max_amount_a, max_amount_b, min_price, max_price)
            .unwrap();
        if let xdr::OperationBody::LiquidityPoolDeposit(xdr::LiquidityPoolDepositOp {
            liquidity_pool_id: xdr::PoolId(xdr::Hash(h)),
            max_amount_a: max_a,
            max_amount_b: max_b,
            min_price: xdr::Price { n: min_n, d: min_d },
            max_price: xdr::Price { n: max_n, d: max_d },
        }) = op.body
        {
            assert_eq!(h, [8; 32]);
            assert_eq!(max_a, max_amount_a);
            assert_eq!(max_b, max_amount_b);
            assert_eq!((min_n, min_d), min_price);
            assert_eq!((max_n, max_d), max_price);

            //
        } else {
            panic!("Fail")
        }
    }
    #[test]
    fn test_lp_deposit_bad_id() {
        let pool_id = hex::encode([8; 33]);
        let max_amount_a = 12 * operation::ONE;
        let max_amount_b = 40 * operation::ONE;
        let min_price = (10, 30);
        let max_price = (15, 30);
        let op = Operation::new().liquidity_pool_deposit(
            &pool_id,
            max_amount_a,
            max_amount_b,
            min_price,
            max_price,
        );
        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidField("pool_id".into()))
        );
    }
    #[test]
    fn test_lp_deposit_bad_id2() {
        let pool_id = hex::encode([8; 31]);
        let max_amount_a = 12 * operation::ONE;
        let max_amount_b = 40 * operation::ONE;
        let min_price = (10, 30);
        let max_price = (15, 30);
        let op = Operation::new().liquidity_pool_deposit(
            &pool_id,
            max_amount_a,
            max_amount_b,
            min_price,
            max_price,
        );
        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidField("pool_id".into()))
        );
    }
    #[test]
    fn test_lp_deposit_bad_price() {
        let pool_id = hex::encode([8; 32]);
        let max_amount_a = 12 * operation::ONE;
        let max_amount_b = 40 * operation::ONE;
        let min_price = (-10, 30);
        let max_price = (15, 30);
        let op = Operation::new().liquidity_pool_deposit(
            &pool_id,
            max_amount_a,
            max_amount_b,
            min_price,
            max_price,
        );
        assert_eq!(op.err(), Some(operation::Error::InvalidPrice(-10, 30)));
    }
    #[test]
    fn test_lp_deposit_bad_price2() {
        let pool_id = hex::encode([8; 32]);
        let max_amount_a = 12 * operation::ONE;
        let max_amount_b = 40 * operation::ONE;
        let min_price = (10, 30);
        let max_price = (15, -30);
        let op = Operation::new().liquidity_pool_deposit(
            &pool_id,
            max_amount_a,
            max_amount_b,
            min_price,
            max_price,
        );
        assert_eq!(op.err(), Some(operation::Error::InvalidPrice(15, -30)));
    }
    #[test]
    fn test_lp_deposit_bad_amount() {
        let pool_id = hex::encode([8; 32]);
        let max_amount_a = 12 * operation::ONE;
        let max_amount_b = -40 * operation::ONE;
        let min_price = (10, 30);
        let max_price = (15, 30);
        let op = Operation::new().liquidity_pool_deposit(
            &pool_id,
            max_amount_a,
            max_amount_b,
            min_price,
            max_price,
        );
        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidAmount(max_amount_b))
        );
    }
    #[test]
    fn test_lp_deposit_bad_amount2() {
        let pool_id = hex::encode([8; 32]);
        let max_amount_a = -12 * operation::ONE;
        let max_amount_b = 40 * operation::ONE;
        let min_price = (10, 30);
        let max_price = (15, 30);
        let op = Operation::new().liquidity_pool_deposit(
            &pool_id,
            max_amount_a,
            max_amount_b,
            min_price,
            max_price,
        );
        assert_eq!(
            op.err(),
            Some(operation::Error::InvalidAmount(max_amount_a))
        );
    }
}
