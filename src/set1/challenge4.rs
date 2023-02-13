use crate::set1;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input_file_as_bytes(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut bytestrings = Vec::<Vec<u8>>::new();

    for line in reader.lines() {
        let mut bytestring = Vec::<u8>::new();
        let mut nibbles = [0u8; 2];
        // Set each nibble of the byte to the matched value, then OR the nibbles
        // together to create a byte
        for (i, ch) in line.unwrap().chars().enumerate() {
            let nibble = i % 2;
            let shift_amt = 4 - (4 * nibble);
            nibbles[nibble] = match ch {
                '0' => 0x0 << shift_amt,
                '1' => 0x1 << shift_amt,
                '2' => 0x2 << shift_amt,
                '3' => 0x3 << shift_amt,
                '4' => 0x4 << shift_amt,
                '5' => 0x5 << shift_amt,
                '6' => 0x6 << shift_amt,
                '7' => 0x7 << shift_amt,
                '8' => 0x8 << shift_amt,
                '9' => 0x9 << shift_amt,
                'a' => 0xa << shift_amt,
                'b' => 0xb << shift_amt,
                'c' => 0xc << shift_amt,
                'd' => 0xd << shift_amt,
                'e' => 0xe << shift_amt,
                'f' => 0xf << shift_amt,
                _ => panic!("Non-supported character: {}", ch),
            };
            if nibble == 1 {
                bytestring.push(nibbles[0] | nibbles[1]);
            }
        }

        bytestrings.push(bytestring);
    }

    bytestrings
}

/// Find the single-char XOR:ed ciphertext amongst a bunch of other potential ciphertexts
pub fn find_single_char_xor_ciphertext(filename: &str) -> String {
    let input_bytes = read_input_file_as_bytes(filename);
    let mut most_probably_english = vec![0u8; 60];
    let mut max_rating = 0.0;

    for input_string in input_bytes {
        let (decrypted, rating) = set1::challenge3::decrypt_single_byte_xor_cipher(&input_string);

        if rating > max_rating {
            most_probably_english = decrypted;
            max_rating = rating;
        }
    }

    String::from_utf8(most_probably_english).unwrap()
}
