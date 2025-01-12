use stellar_strkey::ed25519::PublicKey;
// use stellar_xdr::{xdr::VecM, xdr::ClaimPredicate};
use crate::keypair::Keypair;
use crate::keypair::KeypairBehavior;
use crate::xdr;

pub struct Claimant {
    destination: Option<String>,
    predicate: xdr::ClaimPredicate,
}

// Define a trait for Claimant behavior
pub trait ClaimantBehavior {
    fn new(
        destination: Option<&str>,
        predicate: Option<xdr::ClaimPredicate>,
    ) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn predicate_unconditional() -> xdr::ClaimPredicate;
    fn predicate_and(left: xdr::ClaimPredicate, right: xdr::ClaimPredicate) -> xdr::ClaimPredicate;
    fn predicate_or(left: xdr::ClaimPredicate, right: xdr::ClaimPredicate) -> xdr::ClaimPredicate;
    fn predicate_not(predicate: xdr::ClaimPredicate) -> xdr::ClaimPredicate;
    fn predicate_before_absolute_time(abs_before: i64) -> xdr::ClaimPredicate;
    fn predicate_before_relative_time(seconds_str: &str) -> xdr::ClaimPredicate;
    fn from_xdr(claimant_xdr: xdr::Claimant) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn to_xdr_object(&self) -> xdr::Claimant;
    fn destination(&self) -> Option<String>;
    fn set_destination(&mut self, value: String);
    fn predicate(&self) -> &xdr::ClaimPredicate;
    fn set_predicate(&mut self, value: xdr::ClaimPredicate);
}

impl ClaimantBehavior for Claimant {
    fn new(
        destination: Option<&str>,
        predicate: Option<xdr::ClaimPredicate>,
    ) -> Result<Self, &'static str> {
        let key = PublicKey::from_string(destination.unwrap());

        if key.is_err() {
            return Err("accountId is invalid");
        }

        let actual_predicate = match predicate {
            Some(pred) => pred,
            None => xdr::ClaimPredicate::Unconditional,
        };

        Ok(Claimant {
            destination: destination.map(String::from),
            predicate: actual_predicate,
        })
    }

    fn predicate_unconditional() -> xdr::ClaimPredicate {
        xdr::ClaimPredicate::Unconditional
    }

    fn predicate_and(left: xdr::ClaimPredicate, right: xdr::ClaimPredicate) -> xdr::ClaimPredicate {
        let cc = vec![left, right];

        xdr::ClaimPredicate::And(xdr::VecM::<xdr::ClaimPredicate, 2>::try_from(cc).unwrap())
    }

    fn predicate_or(left: xdr::ClaimPredicate, right: xdr::ClaimPredicate) -> xdr::ClaimPredicate {
        let cc = vec![left, right];

        xdr::ClaimPredicate::Or(xdr::VecM::<xdr::ClaimPredicate, 2>::try_from(cc).unwrap())
    }

    fn predicate_not(predicate: xdr::ClaimPredicate) -> xdr::ClaimPredicate {
        xdr::ClaimPredicate::Not(Some(Box::new(predicate)))
    }

    fn predicate_before_absolute_time(abs_before: i64) -> xdr::ClaimPredicate {
        xdr::ClaimPredicate::BeforeAbsoluteTime(abs_before)
    }

    fn predicate_before_relative_time(seconds_str: &str) -> xdr::ClaimPredicate {
        let seconds = seconds_str
            .parse::<i64>()
            .expect("Failed to parse seconds string to i64");
        xdr::ClaimPredicate::BeforeRelativeTime(seconds)
    }

    fn from_xdr(claimant_xdr: xdr::Claimant) -> Result<Claimant, &'static str> {
        match claimant_xdr {
            xdr::Claimant::ClaimantTypeV0(value) => {
                let destination_key = value.destination.0;
                let val = match destination_key {
                    xdr::PublicKey::PublicKeyTypeEd25519(x) => x.to_string(),
                };

                Ok(Claimant {
                    destination: Some(val),
                    predicate: value.predicate,
                })
            }
            _ => Err("Invalid claimant type"),
        }
    }

    fn to_xdr_object(&self) -> xdr::Claimant {
        let claimant = xdr::ClaimantV0 {
            destination: Keypair::from_public_key(self.destination.clone().unwrap().as_str())
                .unwrap()
                .xdr_account_id(),
            predicate: self.predicate.clone(),
        };

        xdr::Claimant::ClaimantTypeV0(claimant)
    }

    fn destination(&self) -> Option<String> {
        self.destination.clone()
    }

    fn set_destination(&mut self, _value: String) {
        self.destination = Some(_value);
    }

    fn predicate(&self) -> &xdr::ClaimPredicate {
        &self.predicate
    }

    fn set_predicate(&mut self, _value: xdr::ClaimPredicate) {
        self.predicate = _value;
    }
}
