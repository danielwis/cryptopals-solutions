use std::collections::HashMap;

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
