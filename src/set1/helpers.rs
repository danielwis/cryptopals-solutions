use std::collections::HashMap;

const ASCII_UPPERCASE_LOWER: u8 = 0x41;
const ASCII_UPPERCASE_UPPER: u8 = 0x5A;
const ASCII_LOWERCASE_LOWER: u8 = 0x61;
const ASCII_LOWERCASE_UPPER: u8 = 0x7A;
const ASCII_SPACE: u8 = 0x20;

const BASE_64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

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

/*
for byte in bytes {
    let byte_b64_value =

}
*/

// Take four 6-bit b64 values and squash them into a 24-bit value
fn b64_chunk_to_bits(chunk: &[u8]) -> u32 {
    let chunk_bits = 6;
    let first_value_offset = 3 * chunk_bits; // 3*6, next offsets 2*6, 1*6 and 0*6
    let mut chunk_as_bits: u32;
    chunk_as_bits = (chunk[0] as u32) << first_value_offset;

    // OR the bits into the u32. If the chunk is smaller than 4 sextets/bytes, the last
    // bits will be left at zero
    for i in 0..chunk.len() {
        chunk_as_bits |= (chunk[i] as u32) << first_value_offset - (i * chunk_bits);
    }

    chunk_as_bits
}

// 1. Find position of sextet (encoded as 8-bit char)
// 2. This is our first six bits
// 3. Repeat for all four chars
fn push_bytes(output_bytes: &mut Vec<u8>, chunk_to_push: &[u8]) {
    let bits: u32 = b64_chunk_to_bits(chunk_to_push);
    let mut bits_converted: u32 = 0;

    // Build an u32 from the four sextets, to then be broken down into three bytes
    for i in 0..4 {
        // Get the current sextet char, i.e. what we're looking to convert back to six bits
        let sextet_char = chunk_to_push[i];
        // Find the index of this char
        let character_position = BASE_64_TABLE
            .iter()
            .position(|&b| b == char::from_digit(sextet_char as u32, 2).unwrap())
            .unwrap();
        // This index represents the first six bits, so shift up and mask
        let character_position = (character_position << 2) & 0x3f;
        // Build the u32
        bits_converted |= (character_position << (18 - i * 6)) as u32;
    }

    // Push each byte, starting with the one at idx 24-17 (or 23-16)
    for i in 0..3 {
        let byte = (bits_converted >> ((16 - i * 8) & 0xff)) as u8;
        // Don't push null bytes (padding)
        if byte != 0x0 {
            output_bytes.push(byte);
        }
    }
}

// Input length should always be divisible by 4
pub fn base64_to_hex(input: &str) -> Vec<u8> {
    // Remove padding '=' characters
    let input = input.trim_end_matches('=');

    let bytes = input.as_bytes();
    let chunks = bytes.chunks_exact(4);
    let remaining_bytes = bytes.len() % 4;
    let mut remaining_chunk = &bytes[bytes.len() - remaining_bytes..];
    let mut output = Vec::<u8>::new();

    for chunk in chunks {
        push_bytes(&mut output, chunk);
    }

    push_bytes(&mut output, remaining_chunk);

    output
}
