use stellar_xdr::*;
use crate::hashing::hash;
use crate::keypair::Keypair;

pub struct TxBase {
    network_passphrase: String,
    tx: String,
    signatures: Vec<DecoratedSignature>,
    fee: String,
}

impl TxBase {
    pub fn new(tx: String, signatures: Vec<DecoratedSignature>, fee: String, network_passphrase: String) -> Result<TxBase, Box<dyn std::error::Error>> {
        Ok(TxBase {
            network_passphrase,
            tx,
            signatures,
            fee,
        })
    }

    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }

    pub fn set_signatures(&mut self, _value: Vec<DecoratedSignature>) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::from("Transaction is immutable"))
    }

    pub fn tx(&self) -> &String {
        &self.tx
    }

    pub fn set_tx(&mut self, _value: String) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::from("Transaction is immutable"))
    }

    pub fn fee(&self) -> &String {
        &self.fee
    }

    pub fn set_fee(&mut self, _value: String) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::from("Transaction is immutable"))
    }

    pub fn network_passphrase(&self) -> &String {
        &self.network_passphrase
    }

    pub fn set_network_passphrase(&mut self, network_passphrase: String) {
        self.network_passphrase = network_passphrase;
    }
    
    pub fn add_decorated_signature(&mut self, signature: DecoratedSignature) {
        self.signatures.push(signature);
    }
    
}