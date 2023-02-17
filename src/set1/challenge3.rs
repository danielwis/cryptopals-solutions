use crate::set1;
use std::vec;

fn find_key_one_byte(input: &[u8]) -> (u8, Vec<u8>, f64) {
    let mut max_score: f64 = 0.0;
    let mut most_probable_key: u8 = 0x0;
    let mut ans = input.clone().to_vec();

    // Try XORing with all bytes
    for i in 0..=255 {
        let v = vec![i; input.len()];
        let xor_with_i = set1::challenge2::xor_slices(&v, input);
        let score = set1::helpers::rate_text(&xor_with_i);

        if score > max_score {
            max_score = score;
            most_probable_key = i;
            ans = xor_with_i;
        }
    }

    return (most_probable_key, ans, max_score);
}

pub fn decrypt_single_byte_xor_cipher(input: &[u8]) -> (u8, Vec<u8>, f64) {
    let (key, ans, rating) = find_key_one_byte(input);
    return (key, ans, rating);
}
