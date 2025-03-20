use crate::{
    asset::{Asset, AssetBehavior},
    operation::{self, Operation},
    xdr,
};

impl Operation {
    /// Creates, updates, or deletes an offer to buy a specific amount of an asset for another
    pub fn manage_buy_offer(
        &self,
        selling: Asset,
        buying: Asset,
        buy_amount: i64,
        (n, d): (i32, i32),
        offer_id: i64,
    ) -> Result<xdr::Operation, operation::Error> {
        //
        if buy_amount < 0 {
            return Err(operation::Error::InvalidAmount(buy_amount));
        }
        if n <= 0 || d <= 0 {
            return Err(operation::Error::InvalidPrice(n, d));
        }
        let body = xdr::OperationBody::ManageBuyOffer(xdr::ManageBuyOfferOp {
            selling: selling.to_xdr_object(),
            buying: selling.to_xdr_object(),
            buy_amount,
            price: xdr::Price { n, d },
            offer_id,
        });
        Ok(xdr::Operation {
            source_account: self.source.clone(),
            body,
        })
    }
}
