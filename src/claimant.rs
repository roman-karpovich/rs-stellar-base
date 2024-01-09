use stellar_strkey::ed25519::PublicKey;
// use stellar_xdr::{VecM, ClaimPredicate};
use stellar_xdr::next::{VecM, ClaimPredicate};
use crate::keypair::KeypairBehavior;
use crate::keypair::Keypair;



pub struct Claimant {
    destination: Option<String>,
    predicate: ClaimPredicate,
}

impl Claimant {
    pub fn new(destination: Option<&str>, predicate: Option<ClaimPredicate>) -> Result<Self, &'static str> {
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

    pub fn predicate_unconditional() -> ClaimPredicate {
        ClaimPredicate::Unconditional
    }

    pub fn predicate_and(left: ClaimPredicate, right: ClaimPredicate) -> ClaimPredicate {
        let cc = vec![left, right];
        
        ClaimPredicate::And(VecM::<ClaimPredicate, 2>::try_from(cc).unwrap())
    }

    pub fn predicate_or(left: ClaimPredicate, right: ClaimPredicate) -> ClaimPredicate {
        let cc = vec![left, right];
        
        ClaimPredicate::Or(VecM::<ClaimPredicate, 2>::try_from(cc).unwrap())
    }

    pub fn predicate_not(predicate: ClaimPredicate) -> ClaimPredicate {
        ClaimPredicate::Not(Some(Box::new(predicate)))
    }

    pub fn predicate_before_absolute_time(abs_before: i64) -> ClaimPredicate {
        ClaimPredicate::BeforeAbsoluteTime(abs_before)
    }

    pub fn predicate_before_relative_time(seconds_str: &str) -> ClaimPredicate {
        let seconds = seconds_str.parse::<i64>().expect("Failed to parse seconds string to i64");
        ClaimPredicate::BeforeRelativeTime(seconds)
    }

    pub fn from_xdr(claimant_xdr: stellar_xdr::next::Claimant) -> Result<Claimant, &'static str> {
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

    pub fn to_xdr_object(&self) -> stellar_xdr::next::Claimant {
        let claimant = stellar_xdr::next::ClaimantV0 {
            destination: Keypair::from_public_key(&self.destination.clone().unwrap().as_str()).unwrap().xdr_account_id(),
            predicate: self.predicate.clone(),
        };

        stellar_xdr::next::Claimant::ClaimantTypeV0(claimant)
    }

    pub fn destination(&self) -> Option<String> {
        let val = self.destination.clone();
        val
    }

    pub fn set_destination(&mut self, _value: String) {
        self.destination = Some(_value);
    }

    pub fn predicate(&self) -> &ClaimPredicate {
        &self.predicate
    }

    pub fn set_predicate(&mut self, _value: ClaimPredicate) {
        self.predicate = _value;
    }
    
}