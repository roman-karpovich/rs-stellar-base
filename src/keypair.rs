use std::{error::Error, str::FromStr};

use crate::hashing::hash;
use nacl::sign::{generate_keypair, signature};
use rand_core::{OsRng, RngCore};
use sha2::Sha512;
use stellar_strkey::{
    ed25519::{PrivateKey, PublicKey},
    Strkey,
};
use stellar_xdr::MuxedAccountMed25519;
use stellar_xdr::{
    AccountId, DecoratedSignature, Signature, SignatureHint, Uint256, Uint64, WriteXdr,
};

use crate::signing::{generate, sign, verify};
use hex::FromHex;

#[derive(Debug)]
pub struct Keypair {
    public_key: Vec<u8>,
    secret_key: Option<Vec<u8>>,
    secret_seed: Option<Vec<u8>>,
}

impl Keypair {

    fn new(public_key: Option<[u8; 32]>, secret_key: Option<[u8; 32]>) -> Result<Self, Box<dyn Error>> {

        if let Some(secret_key) = secret_key {

        let sec_seed = secret_key;
        let public_key_gen = generate(&sec_seed);
        let mut secret_key = Vec::new();
        secret_key.extend_from_slice(&sec_seed);
        secret_key.extend_from_slice(&public_key_gen);
        
        if let Some(public_key_arg) = public_key {
            if public_key_arg != public_key_gen {
                return Err("secretKey does not match publicKey".into());
            }
        }
        
        Ok(Self {
            secret_seed: Some(sec_seed.to_vec()),
            public_key: public_key_gen.to_vec(),
            secret_key: Some(secret_key),
        })

        } else {
            Ok(Self {
                secret_seed: None,
                public_key: public_key.unwrap().to_vec(),
                secret_key: None,
            })
        }
    }

    fn new_from_secret_key(secret_seed: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        if secret_seed.len() != 32 {
            return Err("secret_key length is invalid".into());
        }

        let mut cloned_secret_key = secret_seed.clone();
        let pkey = generate(&secret_seed);
        let mut pk = pkey.clone().to_vec();

        let mut secret_key = Vec::new();
        secret_key.append(&mut cloned_secret_key);
        secret_key.append(&mut pk);

        Ok(Self {
            secret_seed: Some(secret_seed),
            public_key: pkey.to_vec(),
            secret_key: Some(secret_key),
        })
    }

    fn new_from_public_key(public_key: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        if public_key.len() != 32 {
            return Err("public_key length is invalid".into());
        }

        Ok(Self {
            public_key,
            secret_key: None,
            secret_seed: None,
        })
    }

    pub fn from_secret(secret: &str) -> Result<Self, Box<dyn Error>> {
        let raw_secret = PrivateKey::from_str(secret).unwrap().0;
        Keypair::from_raw_ed25519_seed(&raw_secret)
    }

    pub fn from_public_key(public_key: &str) -> Result<Self, Box<dyn Error>> {
        let decoded = PublicKey::from_str(public_key)?;
        if decoded.0.len() != 32 {
            return Err("Invalid Stellar public key".into());
        }

        Ok(Self {
            public_key: decoded.0.to_vec(),
            secret_seed: None,
            secret_key: None,
        })
    }

    pub fn from_raw_ed25519_seed(seed: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::new_from_secret_key(seed.to_vec())
    }

    pub fn raw_secret_key(&self) -> Option<Vec<u8>> {
        self.secret_seed.clone()
    }

    pub fn raw_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn secret_key(&mut self) -> Result<String, Box<dyn Error>> {
        match &mut self.secret_seed {
            None => Err("no secret_key available".into()),
            Some(s) => Ok(PrivateKey::from_payload(s).unwrap().to_string()),
        }
    }

    pub fn public_key(&self) -> String {
        PublicKey::from_payload(&self.public_key)
            .unwrap()
            .to_string()
    }

    pub fn can_sign(&self) -> bool {
        self.secret_key.is_some()
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        if !self.can_sign() {
            return Err("cannot sign, no secret_key available".into());
        }

        if let Some(s) = &self.secret_key {
            sign(data, s);
        }

        Err("error while signing".into())
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        verify(signature, data, &self.public_key)
    }

    pub fn random() -> Result<Self, Box<dyn Error>> {
        let mut secret_seed = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut secret_seed);
        Self::new_from_secret_key(secret_seed.to_vec())
    }

    pub fn master(network_passphrase: Option<&str>) -> Result<Self, Box<dyn Error>> {
        if let Some(passphrase) = network_passphrase {
            Ok(Self::from_raw_ed25519_seed(&hash(passphrase)).unwrap())
        } else {
            Err("No network selected. Please pass a network argument, e.g. `Keypair::master(Some(Networks::PUBLIC))`.".into())
        }
    }
    pub fn xdr_account_id(&self) -> AccountId {
        AccountId(stellar_xdr::PublicKey::PublicKeyTypeEd25519(Uint256(
            PublicKey::from_payload(&self.public_key).unwrap().0,
        )))
    }

    pub fn xdr_public_key(&self) -> stellar_xdr::PublicKey {
        stellar_xdr::PublicKey::PublicKeyTypeEd25519(Uint256(
            PublicKey::from_payload(&self.public_key).unwrap().0,
        ))
    }

    pub fn xdr_muxed_account_id(&self, id: &str) -> stellar_xdr::MuxedAccount {
        stellar_xdr::MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
            id: Uint64::from_str(id).unwrap(),
            ed25519: Uint256(PublicKey::from_payload(&self.public_key).unwrap().0),
        })
    }

    pub fn raw_pubkey(&self) -> [u8; 32] {
        let mut array: [u8; 32] = [0; 32];

        for (i, &value) in self.public_key.iter().enumerate() {
            array[i] = value;
        }

        array
    }

    pub fn signature_hint(&self) -> Option<Vec<u8>> {
        let a = Self::xdr_account_id(self).to_xdr().unwrap();
        if a.len() >= 4 {
            let start_index = a.len() - 4;
            Some(a[start_index..].to_vec())
        } else {
            None
        }
    }

    pub fn sign_decorated(&self, data: &[u8]) -> DecoratedSignature {
        let signature = Self::sign(self, data).unwrap();
        let hint = Self::signature_hint(self).unwrap();
        let mut hint_u8: [u8; 4] = [0; 4];
        hint_u8.copy_from_slice(&hint[..4]);
        let val = SignatureHint::from(hint_u8);
        let signature_xdr = Signature::try_from(signature).unwrap();
        stellar_xdr::DecoratedSignature {
            hint: val,
            signature: signature_xdr,
        }
    }

    pub fn sign_payload_decorated(&self, data: &[u8]) -> DecoratedSignature {
        let signature = Self::sign(self, data).unwrap();
        let hint = Self::signature_hint(self).unwrap();
        let mut key_hint_u8: [u8; 4] = [0; 4];
        key_hint_u8.copy_from_slice(&hint[..4]);
        let val = SignatureHint::from(key_hint_u8);
        let signature_xdr = Signature::try_from(signature).unwrap();
        let mut hint: [u8; 4] = [0; 4];

        if data.len() >= 4 {
            hint.copy_from_slice(&data[data.len() - 4..]);
        } else {
            hint[..data.len()].copy_from_slice(data);
            for i in data.len()..4 {
                hint[i] = 0;
            }
        }

        for i in 0..4 {
            hint[i] ^= key_hint_u8[i];
        }

        let val = SignatureHint::from(hint);

        stellar_xdr::DecoratedSignature {
            hint: val,
            signature: signature_xdr,
        }
    }
}

#[cfg(test)]
mod tests {

    use lazy_static::__Deref;

    use super::*;
    
    #[test]
    fn keypair_constructor_fails_when_secret_key_does_not_match_public_key() {
        let secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let kp = Keypair::from_secret(secret).unwrap();
        let mut secret_key = kp.raw_secret_key().unwrap();
        let c = secret_key.as_slice();
        let mut public_key = PublicKey::from_str(kp.public_key().as_str()).unwrap().0;
        public_key[0] = 0; // Make public key invalid
        let keypair = Keypair::new(Some(public_key), Some(c.try_into().unwrap()));
        assert!(keypair.is_err());
        assert_eq!(keypair.err().unwrap().to_string(), "secretKey does not match publicKey")

    }
    
  
}