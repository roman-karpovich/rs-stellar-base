use arrayref::array_ref;
use sha2::{Digest, Sha256};

pub fn hash(data: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    *array_ref!(result, 0, 32)
}
