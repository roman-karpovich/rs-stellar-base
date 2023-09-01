pub fn verify_checksum(expected: &[u8], actual: &[u8]) -> bool {
    if expected.len() != actual.len() {
        return false;
    }

    if expected.is_empty() {
        return true;
    }

    for i in 0..expected.len() {
        if expected[i] != actual[i] {
            return false;
        }
    }

    true
}
