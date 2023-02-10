use std::vec;

use super::challenge2;

// const TEXT_CHARS: [char; 52] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
const ASCII_UPPERCASE_LOWER: u8 = 0x41;
const ASCII_UPPERCASE_UPPER: u8 = 0x5A;
const ASCII_LOWERCASE_LOWER: u8 = 0x61;
const ASCII_LOWERCASE_UPPER: u8 = 0x7A;

fn is_english_char(byte: &u8) -> bool {
    (*byte >= ASCII_UPPERCASE_LOWER && *byte <= ASCII_UPPERCASE_UPPER)
        || (*byte >= ASCII_LOWERCASE_LOWER && *byte <= ASCII_LOWERCASE_UPPER)
}

fn freq_english_chars(input: &[u8]) -> f64 {
    let mut non_eng_chars = 0;
    let mut eng_chars = 0;
    for byte in input {
        match is_english_char(byte) {
            true => eng_chars += 1,
            false => non_eng_chars += 1,
        };
    }

    let total_chars = eng_chars + non_eng_chars;
    return eng_chars as f64 / total_chars as f64;
}

fn find_key_one_byte(input: &[u8]) -> (u8, Vec<u8>) {
    let mut max_score: f64 = 0.0;
    let mut most_probable_key: u8 = 0x0;
    let mut ans = vec![0u8; input.len()];

    for i in 0..128 {
        let v = vec![i;input.len()];
        let xor_with_i = challenge2::xor_slices(&v, input);
        let score = freq_english_chars(&xor_with_i);
        if score > max_score {
            max_score = score;
            most_probable_key = i;
            ans = xor_with_i;
        }
    }

    return (most_probable_key, ans);
}

pub fn decrypt_single_byte_xor_cipher(input: &[u8]) -> Vec<u8> {
    let (_, ans) = find_key_one_byte(input);
    return ans;
}
