use std::collections::HashMap;

/// Get the relative frequencies of each character/bigram/trigram/n-gram
/// Returns a hash map of type <String, f64> where the string is lowercase
pub fn n_grams(n: usize, msg: &[u8]) -> HashMap<String, f64> {
    let mut byte_frequencies = HashMap::<String, f64>::new();

    for i in 0..msg.len() - n + 1 {
        let n_gram = String::from_utf8(msg[i..i+n].to_vec()).unwrap().to_lowercase();
        *(byte_frequencies.entry(n_gram).or_insert(0.0)) += 1.0;
    }

    let total_n_grams: f64 = byte_frequencies.values().sum();
    for value in byte_frequencies.values_mut() {
        *value /= total_n_grams;
    }

    byte_frequencies
}
