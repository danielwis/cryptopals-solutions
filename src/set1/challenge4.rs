use crate::set1;

/// Find the single-char XOR:ed ciphertext amongst a bunch of other potential ciphertexts
pub fn find_single_char_xor_ciphertext(filename: &str) -> String {
    let input_bytes = set1::helpers::read_input_file_as_bytes(filename);
    let mut most_probably_english = vec![0u8; 60];
    let mut max_rating = 0.0;

    for input_string in input_bytes {
        let (_, decrypted, rating) =
            set1::challenge3::decrypt_single_byte_xor_cipher(&input_string);

        if rating > max_rating {
            most_probably_english = decrypted;
            max_rating = rating;
        }
    }

    String::from_utf8(most_probably_english).unwrap()
}
