use crate::set1;

pub fn repeating_key_xor(input: &str, key: &str) -> Vec<u8> {
    input
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(i, k)| i ^ k)
        .collect()
}
