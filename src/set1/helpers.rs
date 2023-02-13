use std::collections::HashMap;

const ASCII_UPPERCASE_LOWER: u8 = 0x41;
const ASCII_UPPERCASE_UPPER: u8 = 0x5A;
const ASCII_LOWERCASE_LOWER: u8 = 0x61;
const ASCII_LOWERCASE_UPPER: u8 = 0x7A;
const ASCII_SPACE: u8 = 0x20;

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

/// Get the relative frequencies of each character/bigram/trigram/n-gram
/// Returns a hash map of type <String, f64> where the string is lowercase
pub fn n_grams_str(n: usize, msg: &str) -> HashMap<String, f64> {
    let mut byte_frequencies = HashMap::<String, f64>::new();

    for i in 0..msg.len() - n + 1 {
        let n_gram = msg[i..i + n].to_string().to_lowercase();
        *(byte_frequencies.entry(n_gram).or_insert(0.0)) += 1.0;
    }

    let total_n_grams: f64 = byte_frequencies.values().sum();
    for value in byte_frequencies.values_mut() {
        *value /= total_n_grams;
    }

    byte_frequencies
}

fn is_english_char(byte: &u8) -> bool {
    (*byte >= ASCII_UPPERCASE_LOWER && *byte <= ASCII_UPPERCASE_UPPER)
        || (*byte >= ASCII_LOWERCASE_LOWER && *byte <= ASCII_LOWERCASE_UPPER)
        || (*byte == ASCII_SPACE)
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

pub fn rate_text(bytes: &[u8]) -> f64 {
    let mut rating = 0.0;
    let char_freqs = n_grams_bytes(1, bytes);
    let digram_freqs = n_grams_bytes(2, bytes);
    let trigram_freqs = n_grams_bytes(3, bytes);

    /*
    let common_char_list = ["a", "e", "i", "t", "o", "r", "s"];
    let uncommon_char_list = ["z", "q", "j", "k", "x"];
    */
    let common_bigrams_list = ["th", "he", "in", "er", "an", "re"];
    let common_trigrams_list = ["the", "and", "tha", "ent", "ing", "ion"];

    if freq_english_chars(bytes) < 0.9 {
        return 0.0;
    }

    /*
    // Rate common chars
    for ch in common_char_list {
        rating += char_freqs.get(ch).unwrap_or(&0.0);
    }
    // Deduct uncommon chars
    for ch in uncommon_char_list {
        rating -= char_freqs.get(ch).unwrap_or(&0.0);
    }
    */
    // Many spaces is more important
    rating += char_freqs.get(" ").unwrap_or(&0.0) * 10.0;

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
