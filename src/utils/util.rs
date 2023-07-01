
pub fn trim_end(input: String, char: char) -> String {
    let is_number = input.bytes().all(|c| c.is_ascii_digit());
    let mut string = input.to_string();

    while string.ends_with(char) {
        string.pop();
    }

    if is_number {
        string.parse().unwrap()
    } else {
        string
    }
}
