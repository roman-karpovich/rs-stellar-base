
use crate::hashing::hash;
use stellar_strkey::Strkey;
use crate::signing::{generate,sign, verify};


#[derive(Debug)]
pub struct Keypair {
    public_key: [u8; 32],
    secret_key: Option<[u8; 32]>,
    secret_seed: Option<[u8; 32]>,
}


impl Keypair {
    //TODO: Complete Functions required for the Keypair
}