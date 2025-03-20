use crate::{
    asset::{Asset, AssetBehavior},
    operation::{self, Operation},
    xdr,
};

impl Operation {
    /// Creates an offer to sell one asset for another without taking a reverse offer of equal price
    pub fn create_passive_sell_offer(
        &self,
        selling: Asset,
        buying: Asset,
        amount: i64,
        (n, d): (i32, i32),
    ) -> Result<xdr::Operation, operation::Error> {
        //
        if amount < 0 {
            return Err(operation::Error::InvalidAmount(amount));
        }
        if n <= 0 || d <= 0 {
            return Err(operation::Error::InvalidPrice(n, d));
        }
        let body = xdr::OperationBody::CreatePassiveSellOffer(xdr::CreatePassiveSellOfferOp {
            selling: selling.to_xdr_object(),
            buying: buying.to_xdr_object(),
            amount,
            price: xdr::Price { n, d },
        });
        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }
}
