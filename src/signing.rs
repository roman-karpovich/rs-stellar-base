use lazy_static::lazy_static;
use hex_literal::hex;

lazy_static! {
    static ref ACTUAL_METHODS: ActualMethods = check_fast_signing();
}

struct ActualMethods {
    generate: fn(&[u8]) -> [u8; 32],
    sign: fn(&[u8], &[u8]) -> [u8; 64],
    verify: fn(&[u8], &[u8], &[u8]) -> bool,
}

pub fn sign(data: &[u8], secret_key: &[u8]) -> [u8; 64] {
    (ACTUAL_METHODS.sign)(data, secret_key)
}

pub fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    (ACTUAL_METHODS.verify)(data, signature, public_key)
}

pub fn generate(secret_key: &[u8]) -> [u8; 32] {
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
        let nacl_keys = nacl::sign::generate_keypair(secret_key_u8);
        nacl_keys.pkey
    }

    fn sign(data: &[u8], secret_key: &[u8]) -> [u8; 64] {
        let mut data_u8 = data;
        let mut secret_key_u8 = secret_key;
        let signed_msg = nacl::sign::signature(&mut data_u8, &mut secret_key_u8).unwrap();
        let mut signature = [0u8; 64];

        for i in 0..signed_msg.len() {
            signature[i] = signed_msg[i];
        }
        signature
    }

    fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        let data_u8 = data;
        let signature_u8 = signature;
        let public_key_u8 = public_key;
        nacl::sign::verify(data_u8, signature_u8, public_key_u8).unwrap()
    }

    ActualMethods {
        generate,
        sign,
        verify,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn check_fast_signing_native() -> ActualMethods {
    use libsodium_sys::crypto_sign_detached;
    use libsodium_sys::crypto_sign_seed_keypair;

    unsafe {
        libsodium_sys::sodium_init();
    };

    fn generate(secret_key: &[u8]) -> [u8; 32] {
        let secret_key_u8: &[u8; 32] = secret_key.try_into().unwrap();
        let nacl_keys = nacl::sign::generate_keypair(secret_key_u8);
        nacl_keys.pkey
        // unsafe {
        //     let mut pk = [0u8; libsodium_sys::crypto_sign_PUBLICKEYBYTES as usize];
        //     let mut sk = [0u8; libsodium_sys::crypto_sign_SECRETKEYBYTES as usize];

        //     libsodium_sys::crypto_sign_seed_keypair(
        //         pk.as_mut_ptr(),
        //         sk.as_mut_ptr(),
        //         secret_key.as_ptr() as *const _,
        //     );

        //     pk
        // }
    }

    fn sign(data: &[u8], secret_key: &[u8]) -> [u8; 64] {
        let mut data_u8 = data;
        let mut secret_key_u8 = secret_key;
        let signed_msg = nacl::sign::signature(&mut data_u8, &mut secret_key_u8).unwrap();
        let mut signature = [0u8; 64];

        for i in 0..signed_msg.len() {
            signature[i] = signed_msg[i];
        }
        signature
        // unsafe {
        //     unsafe {
        //         let mut signature = [0u8; libsodium_sys::crypto_sign_BYTES as usize];

        //         libsodium_sys::crypto_sign_detached(
        //             signature.as_mut_ptr(),
        //             std::ptr::null_mut(),
        //             data.as_ptr(),
        //             data.len() as u64,
        //             secret_key.as_ptr(),
        //         );

        //         signature
        //     }
        // }
    }

    fn verify(data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {

        let data_u8 = data;
        let signature_u8 = signature;
        let public_key_u8 = public_key;
        nacl::sign::verify(signature_u8, data_u8, public_key_u8).unwrap()

        // unsafe {
        //     let val = libsodium_sys::crypto_sign_verify_detached(
        //         signature.as_ptr(),
        //         data.as_ptr(),
        //         data.len() as u64,
        //         public_key.as_ptr(),
        //     );

        //     // print!("Value of verification {}", val);
        //     // panic!("{}");
        //     // true
        //     libsodium_sys::crypto_sign_verify_detached(
        //         signature.as_ptr(),
        //         data.as_ptr(),
        //         data.len() as u64,
        //         public_key.as_ptr(),
        //     ) == 0
        // }
    }

    ActualMethods {
        generate,
        sign,
        verify,
    }
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_hash_string() {
        let data = b"hello world";
        let expected_sig = hex!(
            "587d4b472eeef7d07aafcd0b049640b0bb3f39784118c2e2b73a04fa2f64c9c538b4b2d0f5335e968a480021fdc23e98c0ddf424cb15d8131df8cb6c4bb58309"
        );
        let actual_sig = sign(data, &hex!(
            "1123740522f11bfef6b3671f51e159ccf589ccf8965262dd5f97d1721d383dd4ffbdd7ef9933fe7249dc5ca1e7120b6d7b7b99a7a367e1a2fc6cb062fe420437"
        ));
        assert_eq!(expected_sig, actual_sig);
    }
    #[test]
    fn test_verify_string() {
        let data = "hello world".as_bytes();
        let sig = hex!(
            "587d4b472eeef7d07aafcd0b049640b0bb3f39784118c2e2b73a04fa2f64c9c538b4b2d0f5335e968a480021fdc23e98c0ddf424cb15d8131df8cb6c4bb58309"
        );
        let bad_sig = hex!(
            "687d4b472eeef7d07aafcd0b049640b0bb3f39784118c2e2b73a04fa2f64c9c538b4b2d0f5335e968a480021fdc23e98c0ddf424cb15d8131df8cb6c4bb58309"
        );
        let public_key = hex!(
            "ffbdd7ef9933fe7249dc5ca1e7120b6d7b7b99a7a367e1a2fc6cb062fe420437"
        );

        assert!(verify(data, &sig, &public_key));
        assert!(!verify(b"corrupted", &sig, &public_key));
        assert!(!verify(data, &bad_sig, &public_key));
    }
}