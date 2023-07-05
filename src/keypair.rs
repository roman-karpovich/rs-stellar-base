
//! `Keypair` represents public (and secret) keys of the account.
//!
//! Currently `Keypair` only supports ed25519 but in the future, this class can be an abstraction layer for other
//! public-key signature systems.
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
use std::str;

use crate::signing::{generate, sign, verify};
use hex::FromHex;

#[derive(Debug, Clone)]
pub struct Keypair {
    public_key: Vec<u8>,
    secret_key: Option<Vec<u8>>,
    secret_seed: Option<Vec<u8>>,
}



impl Keypair {

    pub fn new(public_key: Option<[u8; 32]>, secret_key: Option<[u8; 32]>) -> Result<Self, Box<dyn Error>> {

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
        // println!("key val {:?}", public_key);

        let decoded = PublicKey::from_str(public_key)?;
        // println!("Decoded String {}", decoded.to_string());
        if decoded.0.len() != 32 {
            return Err("Invalid Stellar public key".into());
        }

        // println!("Decoded Vec {:?}", decoded.0);
        Ok(Self {
            public_key: decoded.0.to_vec(),
            secret_seed: None,
            secret_key: None,
        })
    }

    pub fn from_raw_ed25519_seed(seed: &[u8]) -> Result<Self, Box<dyn Error>> {
        if seed.len()>=33 {
            return Err("Invalid seed length".into());
        }
        Self::new_from_secret_key(seed.to_vec())
    }

    pub fn raw_secret_key(&self) -> Option<Vec<u8>> {
        self.secret_seed.clone()
    }

    pub fn raw_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn secret_key(&self) -> Result<String, Box<dyn Error>> {
        match &self.secret_seed {
            None => Err("no secret_key available".into()),
            Some(s) => Ok(PrivateKey::from_payload(s).unwrap().clone().to_string()),
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
        println!("Key {:?}", &self.secret_key);

        if let Some(s) = &self.secret_key {
            return Ok(sign(data, s).to_vec());
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
        println!("Data {:?}", data);
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

    use hex_literal::hex;
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

    #[test]
    fn test_create_keypair_from_secret() {
        let secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let expected_public_key = "GDFQVQCYYB7GKCGSCUSIQYXTPLV5YJ3XWDMWGQMDNM4EAXAL7LITIBQ7";
        let keypair = Keypair::from_secret(secret).unwrap();
        assert_eq!(keypair.public_key().as_str(), expected_public_key);
        assert_eq!(keypair.secret_key().unwrap().as_str(), secret);
    }

    #[test]
    #[should_panic]

    fn test_create_keypair_from_invalid_secret() {
        let invalid_secrets = vec![
            "hel0",
            "SBWUBZ3SIPLLF5CCXLWUB2Z6UBTYAW34KVXOLRQ5HDAZG4ZY7MHNBWJ1",
            "masterpassphrasemasterpassphrase",
            "gsYRSEQhTffqA9opPepAENCr2WG6z5iBHHubxxbRzWaHf8FBWcu",
        ];
        Keypair::from_secret(invalid_secrets[0]).unwrap();
        Keypair::from_secret(invalid_secrets[1]).unwrap();
        Keypair::from_secret(invalid_secrets[2]).unwrap();
        Keypair::from_secret(invalid_secrets[3]).unwrap();
    }

    #[test]
    fn test_create_keypair_from_raw_ed25519_seed() {
        let seed = "masterpassphrasemasterpassphrase";
        let expected_public_key = "GAXDYNIBA5E4DXR5TJN522RRYESFQ5UNUXHIPTFGVLLD5O5K552DF5ZH";
        let expected_secret = "SBWWC43UMVZHAYLTONYGQ4TBONSW2YLTORSXE4DBONZXA2DSMFZWLP2R";
        let expected_raw_public_key = hex!("2e3c35010749c1de3d9a5bdd6a31c12458768da5ce87cca6aad63ebbaaef7432");
        let keypair = Keypair::from_raw_ed25519_seed(seed.as_bytes()).unwrap();

        assert_eq!(keypair.public_key(), expected_public_key);
        assert_eq!(keypair.secret_key().unwrap().as_str(), expected_secret);
        assert_eq!(keypair.raw_public_key().as_slice(), expected_raw_public_key);
    }

    #[test]
    fn test_create_keypair_invalid_raw_ed25519_seed() {
            Keypair::from_raw_ed25519_seed(b"masterpassphrasemasterpassphras").is_err();
            Keypair::from_raw_ed25519_seed(b"masterpassphrasemasterpassphrase1").is_err();
            Keypair::from_raw_ed25519_seed(b"").is_err();
            Keypair::from_raw_ed25519_seed(b"\0").is_err();
    }

    #[test]
    fn test_create_keypair_from_public_key() {
        let public_key = "GAXDYNIBA5E4DXR5TJN522RRYESFQ5UNUXHIPTFGVLLD5O5K552DF5ZH";
        let expected_public_key = "GAXDYNIBA5E4DXR5TJN522RRYESFQ5UNUXHIPTFGVLLD5O5K552DF5ZH";
        let expected_raw_public_key = hex!("2e3c35010749c1de3d9a5bdd6a31c12458768da5ce87cca6aad63ebbaaef7432");

        let keypair = Keypair::from_public_key(public_key).unwrap();

        assert_eq!(keypair.public_key().as_str(), expected_public_key);
        assert_eq!(keypair.raw_public_key().as_slice(), expected_raw_public_key);
    }
    
    #[test]
    fn test_create_keypair_from_invalid_public_key() {
        let invalid_public_keys = vec![
            "hel0",
            "masterpassphrasemasterpassphrase",
            "sfyjodTxbwLtRToZvi6yQ1KnpZriwTJ7n6nrASFR6goRviCU3Ff",
        ];

        Keypair::from_public_key(invalid_public_keys[0]).is_err();
        Keypair::from_public_key(invalid_public_keys[1]).is_err();
        Keypair::from_public_key(invalid_public_keys[2]).is_err();
    }   

    #[test]
    fn test_create_random_keypair() {
        let keypair = Keypair::random().unwrap();
    }

    #[test]
    fn test_xdr_muxed_account_with_ed25519_key_type() {
        let public_key = "GAXDYNIBA5E4DXR5TJN522RRYESFQ5UNUXHIPTFGVLLD5O5K552DF5ZH";
        let keypair = Keypair::from_public_key(public_key).unwrap();
        let muxed = keypair.xdr_muxed_account_id("1");
    }


    //TODO: Sign Decorated Signature Tests
}