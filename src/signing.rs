
use lazy_static::lazy_static;

lazy_static! {
    static ref ACTUAL_METHODS: ActualMethods = check_fast_signing_rs();
}

struct ActualMethods {
    generate: fn(&[u8]) -> [u8; 32],
    sign: fn(&[u8], &[u8]) -> [u8; 64],
    verify: fn(&[u8], &[u8], &[u8]) -> bool,
}

fn sign(data: &[u8], secret_key: &[u8]) -> [u8; 64] {
    (ACTUAL_METHODS.sign)(data, secret_key)
}

fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    (ACTUAL_METHODS.verify)(data, signature, public_key)
}

fn generate(secret_key: &[u8]) -> [u8; 32] {
    (ACTUAL_METHODS.generate)(secret_key)
}



#[cfg(not(target_arch = "wasm32"))]
fn check_fast_signing_rs() -> ActualMethods {
    use libsodium_sys::{crypto_sign_seed_keypair};
    use libsodium_sys::crypto_sign_detached;
    
    unsafe { libsodium_sys::sodium_init(); };
    
    fn generate(secret_key: &[u8]) -> [u8; 32] {

        unsafe {
            let mut pk = [0u8; libsodium_sys::crypto_sign_PUBLICKEYBYTES as usize];
            let mut sk = [0u8; libsodium_sys::crypto_sign_SECRETKEYBYTES as usize];
    
            libsodium_sys::crypto_sign_seed_keypair(
                pk.as_mut_ptr(),
                sk.as_mut_ptr(),
                secret_key.as_ptr() as *const _,
            );
    
            pk
        }
    }

    fn sign(data: &[u8], secret_key: &[u8]) -> [u8; 64] {
        unsafe {
            unsafe {
                let mut signature = [0u8; libsodium_sys::crypto_sign_BYTES as usize];
        
                libsodium_sys::crypto_sign_detached(
                    signature.as_mut_ptr(),
                    std::ptr::null_mut(),
                    data.as_ptr(),
                    data.len() as u64,
                    secret_key.as_ptr(),
                );
        
                signature
            }
        }
    }

    fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        unsafe {
            libsodium_sys::crypto_sign_verify_detached(
                signature.as_ptr(),
                data.as_ptr(),
                data.len() as u64,
                public_key.as_ptr(),
            ) == 0
        }
    }

    ActualMethods {
        generate,
        sign,
        verify,
    }
}