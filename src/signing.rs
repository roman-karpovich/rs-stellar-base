
use lazy_static::lazy_static;

lazy_static! {
    static ref ACTUAL_METHODS: ActualMethods = check_fast_signing();
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

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn check_fast_signing() -> ActualMethods {
    check_fast_signing_browser()
}

#[cfg(not(target_arch = "wasm32"))]
fn check_fast_signing() -> ActualMethods {
    check_fast_signing_native()
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn check_fast_signing_browser() -> ActualMethods {
    fn generate(secret_key: &[u8]) -> [u8; 32] {
        let secret_key_u8: &[u8; 32] = secret_key.try_into().unwrap();
        let nacl_keys = tweetnacl::sign::keypair_from_seed(secret_key_u8);
        nacl_keys.public_key
    }

    fn sign(data: &[u8], secret_key: &[u8]) -> [u8; 64] {
        let data_u8: &[u8; N] = data.try_into().unwrap();
        let secret_key_u8: &[u8; N] = secret_key.try_into().unwrap();
        tweetnacl::sign::detached(data_u8, secret_key_u8)
    }

    fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        let data_u8: &[u8; N] = data.try_into().unwrap();
        let signature_u8: &[u8; N] = signature.try_into().unwrap();
        let public_key_u8: &[u8; N] = public_key.try_into().unwrap();
        tweetnacl::sign::detached_verify(data_u8, signature_u8, public_key_u8)
    }

    ActualMethods {
        generate,
        sign,
        verify,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn check_fast_signing_native() -> ActualMethods {
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