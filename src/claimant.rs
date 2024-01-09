use stellar_strkey::ed25519::PublicKey;
// use stellar_xdr::{VecM, ClaimPredicate};
use stellar_xdr::next::{VecM, ClaimPredicate};
use crate::keypair::KeypairBehavior;
use crate::keypair::Keypair;



pub struct Claimant {
    destination: Option<String>,
    predicate: ClaimPredicate,
}

// Define a trait for Claimant behavior
pub trait ClaimantBehavior {
    fn new(destination: Option<&str>, predicate: Option<ClaimPredicate>) -> Result<Self, &'static str> where Self: Sized;
    fn predicate_unconditional() -> ClaimPredicate;
    fn predicate_and(left: ClaimPredicate, right: ClaimPredicate) -> ClaimPredicate;
    fn predicate_or(left: ClaimPredicate, right: ClaimPredicate) -> ClaimPredicate;
    fn predicate_not(predicate: ClaimPredicate) -> ClaimPredicate;
    fn predicate_before_absolute_time(abs_before: i64) -> ClaimPredicate;
    fn predicate_before_relative_time(seconds_str: &str) -> ClaimPredicate;
    fn from_xdr(claimant_xdr: stellar_xdr::next::Claimant) -> Result<Self, &'static str> where Self: Sized;
    fn to_xdr_object(&self) -> stellar_xdr::next::Claimant;
    fn destination(&self) -> Option<String>;
    fn set_destination(&mut self, value: String);
    fn predicate(&self) -> &ClaimPredicate;
    fn set_predicate(&mut self, value: ClaimPredicate);
}

impl ClaimantBehavior for Claimant {
    fn new(destination: Option<&str>, predicate: Option<ClaimPredicate>) -> Result<Self, &'static str> {
        let key = PublicKey::from_string(destination.unwrap());

        if key.is_err() {
            return Err("accountId is invalid".into());
        }

        let actual_predicate = match predicate {
            Some(pred) => pred,
            None => ClaimPredicate::Unconditional,
        };

        Ok(Claimant {
            destination: destination.map(String::from),
            predicate: actual_predicate,
        })
    }

    fn predicate_unconditional() -> ClaimPredicate {
        ClaimPredicate::Unconditional
    }

    fn predicate_and(left: ClaimPredicate, right: ClaimPredicate) -> ClaimPredicate {
        let cc = vec![left, right];
        
        ClaimPredicate::And(VecM::<ClaimPredicate, 2>::try_from(cc).unwrap())
    }

    fn predicate_or(left: ClaimPredicate, right: ClaimPredicate) -> ClaimPredicate {
        let cc = vec![left, right];
        
        ClaimPredicate::Or(VecM::<ClaimPredicate, 2>::try_from(cc).unwrap())
    }

    fn predicate_not(predicate: ClaimPredicate) -> ClaimPredicate {
        ClaimPredicate::Not(Some(Box::new(predicate)))
    }

    fn predicate_before_absolute_time(abs_before: i64) -> ClaimPredicate {
        ClaimPredicate::BeforeAbsoluteTime(abs_before)
    }

    fn predicate_before_relative_time(seconds_str: &str) -> ClaimPredicate {
        let seconds = seconds_str.parse::<i64>().expect("Failed to parse seconds string to i64");
        ClaimPredicate::BeforeRelativeTime(seconds)
    }

    fn from_xdr(claimant_xdr: stellar_xdr::next::Claimant) -> Result<Claimant, &'static str> {
        match claimant_xdr {
            stellar_xdr::next::Claimant::ClaimantTypeV0(value) => {
                let destination_key = value.destination.0;
                let val = match destination_key {
                    stellar_xdr::next::PublicKey::PublicKeyTypeEd25519(x) => x.to_string(),
                };


                Ok(Claimant {
                    destination: Some(val),
                    predicate: value.predicate,
                })
            }
            _ => Err("Invalid claimant type"),
        }
    }

    fn to_xdr_object(&self) -> stellar_xdr::next::Claimant {
        let claimant = stellar_xdr::next::ClaimantV0 {
            destination: Keypair::from_public_key(&self.destination.clone().unwrap().as_str()).unwrap().xdr_account_id(),
            predicate: self.predicate.clone(),
        };

        stellar_xdr::next::Claimant::ClaimantTypeV0(claimant)
    }

    fn destination(&self) -> Option<String> {
        let val = self.destination.clone();
        val
    }

    fn set_destination(&mut self, _value: String) {
        self.destination = Some(_value);
    }

    fn predicate(&self) -> &ClaimPredicate {
        &self.predicate
    }

    fn set_predicate(&mut self, _value: ClaimPredicate) {
        self.predicate = _value;
    }
    
}