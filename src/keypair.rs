
use std::{str::FromStr, error::Error};

use crate::hashing::hash;
use nacl::sign::{generate_keypair, signature};
use sha2::Sha512;
use stellar_strkey::{Strkey, ed25519::{PublicKey, PrivateKey}};
use stellar_xdr::{AccountId, Uint256, Uint64, WriteXdr, SignatureHint, Signature, DecoratedSignature};
use rand_core::{RngCore, OsRng};
use stellar_xdr::MuxedAccountMed25519;

use crate::signing::{generate,sign, verify};
use hex::FromHex;

#[derive(Debug)]
pub struct Keypair {
    public_key: Vec<u8>,
    secret_key: Option<Vec<u8>>,
    secret_seed: Option<Vec<u8>>,
}

impl Keypair {
    fn new_from_secret_key(secret_seed: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        if secret_seed.len() != 32 {
            return Err("secret_key length is invalid".into())
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
            return Err("public_key length is invalid".into())
        }

        Ok(Self {
            public_key,
            secret_key: None,
            secret_seed: None,
        })
    }

    pub fn from_secret_key(secret: &str) -> Result<Self,  Box<dyn Error>> {
        let raw_secret = PrivateKey::from_str(secret).unwrap().0; 
        Keypair::from_raw_ed25519_seed(&raw_secret)
    }

    pub fn from_public_key(public_key: &str) -> Result<Self,  Box<dyn Error>> {
        let decoded = PublicKey::from_str(public_key)?;
        if decoded.0.len() != 32 {
            return Err("Invalid Stellar public key".into())
        }

        Ok(Self {
            public_key: decoded.0.to_vec(),
            secret_seed: None,
            secret_key: None,
        })
    }

    pub fn from_raw_ed25519_seed(seed: &[u8]) -> Result<Self,  Box<dyn Error>> {
        Self::new_from_secret_key(seed.to_vec())
    }

    pub fn raw_secret_key(&self) -> Option<Vec<u8>> {
        self.secret_seed.clone()
    }

    pub fn raw_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn secret_key(&mut self) -> Result<String,  Box<dyn Error>> {
        match &mut self.secret_seed {
            None => return Err("no secret_key available".into()),
            Some(s) => Ok(PrivateKey::from_payload(s).unwrap().to_string()),
        }
    }

    pub fn public_key(&self) -> String {
       PublicKey::from_payload(&self.public_key).unwrap().to_string()
    }

    pub fn can_sign(&self) -> bool {
        self.secret_key.is_some()
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        if !self.can_sign() {
            return Err("cannot sign, no secret_key available".into())
        }

        if let Some(s) = &self.secret_key {
            sign(data, s);
        }

        return Err("error while signing".into())
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
        
        let account_id = AccountId(stellar_xdr::PublicKey::PublicKeyTypeEd25519(Uint256(
            PublicKey::from_payload(
                &self.public_key,
            )
            .unwrap()
            .0,
        )));

        account_id

    }
    
    pub fn xdr_public_key(&self) -> stellar_xdr::PublicKey {
        stellar_xdr::PublicKey::PublicKeyTypeEd25519(Uint256(
            PublicKey::from_payload(
                &self.public_key,
            )
            .unwrap()
            .0,
        ))
    }

    pub fn xdr_muxed_account_id(&self, id: &str) -> stellar_xdr::MuxedAccount {
        return stellar_xdr::MuxedAccount::MuxedEd25519(MuxedAccountMed25519 {
            id: Uint64::from_str(id).unwrap(),
            ed25519: Uint256(
                    PublicKey::from_payload(
                        &self.public_key,
                    )
                    .unwrap()
                    .0,
                )
        });    
    } 

    pub fn raw_pubkey(&self) -> [u8; 32] {
        let mut array: [u8; 32] = [0; 32];

        for (i, &value) in self.public_key.iter().enumerate() {
            array[i] = value;
        }

        return array;
    }

    pub fn signature_hint(&self) -> Option<Vec<u8>>  {
        let a = Self::xdr_account_id(&self).to_xdr().unwrap();
        if a.len() >= 4 {
            let start_index = a.len() - 4;
            Some(a[start_index..].to_vec())
        } else {
            None
        }
    }
  
   pub fn sign_decorated(&self, data: &[u8]) -> DecoratedSignature{
        let signature = Self::sign(&self, data).unwrap();
        let hint = Self::signature_hint(&self).unwrap();
        let mut hint_u8: [u8; 4] = [0; 4];
        hint_u8.copy_from_slice(&hint[..4]);
        let val =  SignatureHint::from(hint_u8);
        let signature_xdr = Signature::try_from(signature).unwrap();
        let decorated_signature = stellar_xdr::DecoratedSignature {
            hint: val,
            signature: signature_xdr,
        };

        return decorated_signature 
   }

}
