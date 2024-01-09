//! Utility Sha256 Hash Function
use arrayref::array_ref;
use sha2::{Digest, Sha256};

// Define a trait for generic hashing behavior
pub trait HashingBehavior {
    fn hash<T: AsRef<[u8]>>(data: T) -> [u8; 32];
}

// Implement the trait for a struct representing a Sha256 hasher
pub struct Sha256Hasher;

impl HashingBehavior for Sha256Hasher {
    /// Hash Function using SHA-256
    fn hash<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data.as_ref());
        let result = hasher.finalize();
        *array_ref!(result, 0, 32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let msg = "hello world";
        let expected_hex =
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_owned();
        let actual_hash = Sha256Hasher::hash(msg);
        let actual_hex = hex::encode(actual_hash);
        assert_eq!(actual_hex, expected_hex);
    }

    #[test]
    fn test_hash_buffer() {
        let msg = "hello world".as_bytes();
        let expected_hex =
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_owned();

        let actual_hash = Sha256Hasher::hash(msg);
        let actual_hex = hex::encode(actual_hash);

        assert_eq!(actual_hex, expected_hex);
    }

    #[test]
    fn test_hash_byte_array() {
        let msg: [u8; 11] = [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
        let expected_hex =
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_owned();
        let actual_hash = Sha256Hasher::hash(&msg);
        let actual_hex = hex::encode(actual_hash);
        assert_eq!(actual_hex, expected_hex);
    }
}
