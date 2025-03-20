use crate::{
    asset::{Asset, AssetBehavior},
    operation::{self, Operation},
    xdr,
};

impl Operation {
    /// Creates, updates, or deletes an offer to sell a specific amount of an asset for another
    pub fn manage_sell_offer(
        &self,
        selling: Asset,
        buying: Asset,
        sell_amount: i64,
        (n, d): (i32, i32),
        offer_id: i64,
    ) -> Result<xdr::Operation, operation::Error> {
        //
        if sell_amount < 0 {
            return Err(operation::Error::InvalidAmount(sell_amount));
        }
        if n <= 0 || d <= 0 {
            return Err(operation::Error::InvalidPrice(n, d));
        }
        let body = xdr::OperationBody::ManageSellOffer(xdr::ManageSellOfferOp {
            selling: selling.to_xdr_object(),
            buying: buying.to_xdr_object(),
            amount: sell_amount,
            price: xdr::Price { n, d },
            offer_id,
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
        asset::{Asset, AssetBehavior},
        keypair::{Keypair, KeypairBehavior},
        operation::{self, Operation},
    };

    #[test]
    fn test_manage_sell_offer() {
        let selling = Asset::new("ABC", Some(&Keypair::random().unwrap().public_key())).unwrap();
        let buying = Asset::new("XYZ", Some(&Keypair::random().unwrap().public_key())).unwrap();
        let sell_amount = 38 * operation::ONE;
        let n = 1;
        let d = 2;
        let offer_id = 0;
        let op = Operation::new()
            .manage_sell_offer(selling, buying, sell_amount, (n, d), offer_id)
            .unwrap();
        todo!();
    }
}
