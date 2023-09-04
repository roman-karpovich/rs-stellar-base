use core::panic;
use std::collections::HashMap;

use stellar_strkey::{ed25519::{SignedPayload, PublicKey}, PreAuthTx, HashX};
use stellar_xdr::{SignerKey as XDRSignerKey, SignerKeyEd25519SignedPayload};

pub struct SignerKey;

impl SignerKey {
    
    pub fn decode_address(address: &str) -> XDRSignerKey {
      
        let val = stellar_strkey::Strkey::from_string(address).unwrap();
        let to_return = match val {
            stellar_strkey::Strkey::SignedPayloadEd25519(x) => XDRSignerKey::Ed25519SignedPayload(SignerKeyEd25519SignedPayload { ed25519: stellar_xdr::Uint256(x.ed25519), payload: x.payload.try_into().unwrap()}),
            stellar_strkey::Strkey::PublicKeyEd25519(x) => XDRSignerKey::Ed25519(stellar_xdr::Uint256(x.0)),
            stellar_strkey::Strkey::PreAuthTx(x) => XDRSignerKey::PreAuthTx(stellar_xdr::Uint256(x.0)),
            stellar_strkey::Strkey::HashX(x) => XDRSignerKey::HashX(stellar_xdr::Uint256(x.0)),
            _ => panic!("Invalid Type"),
        };

        to_return
    }

    pub fn encode_signer_key(signer_key: &XDRSignerKey) -> String {

        match signer_key {
            XDRSignerKey::Ed25519(x) => stellar_strkey::Strkey::PublicKeyEd25519(PublicKey::from_payload(&x.0).unwrap()).to_string(),
            XDRSignerKey::PreAuthTx(x) => stellar_strkey::Strkey::PreAuthTx(PreAuthTx::from_string(&String::from_utf8(x.0.to_vec()).unwrap()).unwrap()).to_string(),
            XDRSignerKey::HashX(x) => stellar_strkey::Strkey::HashX(HashX::from_string(&String::from_utf8(x.0.to_vec()).unwrap()).unwrap()).to_string(),
            XDRSignerKey::Ed25519SignedPayload(x) => stellar_strkey::Strkey::SignedPayloadEd25519(SignedPayload { ed25519: x.ed25519.0, payload: x.payload.clone().into_vec()}).to_string(),
        }
       
    }
}
