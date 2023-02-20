use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Read a file of hexstrings (e.g. 4a556f8b) into actual bytestrings.
/// Does this line by line, so each line will yield a new bytestring.
pub fn read_input_file_as_bytes(filename: &str) -> Vec<Vec<u8>> {
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


/// Get the relative frequencies of each character/bigram/trigram/n-gram
/// Returns a hash map of type <String, f64> where the string is lowercase
pub fn n_grams_bytes(n: usize, msg: &[u8]) -> HashMap<String, f64> {
    let mut byte_frequencies = HashMap::<String, f64>::new();

    for i in 0..msg.len() - n + 1 {
        let n_gram = String::from_utf8(msg[i..i + n].to_vec())
            .unwrap_or("".to_owned())
            .to_lowercase();
        *(byte_frequencies.entry(n_gram).or_insert(0.0)) += 1.0;
    }

    let total_n_grams: f64 = byte_frequencies.values().sum();
    for value in byte_frequencies.values_mut() {
        *value /= total_n_grams;
    }

    byte_frequencies
}

pub fn rate_text(bytes: &[u8]) -> f64 {
    let mut rating = 0.0;
    let char_freqs = n_grams_bytes(1, bytes);
    let digram_freqs = n_grams_bytes(2, bytes);
    let trigram_freqs = n_grams_bytes(3, bytes);

    // Character frequencies, from lowest to highest
    let char_freq_list = vec![
        "z", "x", "q", "j", "k", "v", "b", "y", "p", "g", "f", "m", "w", "u", "c", "l", "d", "r",
        "h", "s", "n", "i", "o", "a", "t", "e", " ",
    ];
    let common_bigrams_list = ["th", "he", "in", "er", "an", "re"];
    let common_trigrams_list = ["the", "and", "tha", "ent", "ing", "ion"];

    for (i, c) in char_freq_list.iter().enumerate() {
        rating += char_freqs.get(*c).unwrap_or(&0.0) * i as f64;
    }

    // Rate bigrams highly
    for bgr in common_bigrams_list {
        rating += digram_freqs.get(bgr).unwrap_or(&0.0) * 25.0;
    }
    // Rate trigrams higher
    for tgr in common_trigrams_list {
        rating += trigram_freqs.get(tgr).unwrap_or(&0.0) * 50.0;
    }

    rating
}
