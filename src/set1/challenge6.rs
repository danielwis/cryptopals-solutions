use std::{fs::File, io::Read};

use crate::set1;

fn hamming_distance_bitwise(a: u8, b: u8) -> u8 {
    let mut dist = 0;

    for i in 0..8 {
        let b1 = (a >> i) & 0x1;
        let b2 = (b >> i) & 0x1;
        if b1 != b2 {
            dist += 1;
        }
    }

    dist
}

// Calculate the Hamming distance between the two byte strings
// i.e. how many bits differ
pub fn hamming_distance(str1: &[u8], str2: &[u8]) -> u32 {
    let length_diff = if str1.len() > str2.len() {
        str1.len() - str2.len()
    } else {
        str2.len() - str1.len()
    } as u32;
    let mut dist = length_diff * 8; // One char diff in length => 8 bits diff

    for (c1, c2) in str1.iter().zip(str2.iter()) {
        dist += hamming_distance_bitwise(*c1, *c2) as u32;
    }

    dist
}

// Try a keysize for solving the Vigenère cipher. The "score" is the normalised
// Hamming distance between two blocks of size `keysize`, i.e. `dist / keysize`.
// The input must be at least `keysize * 2` elements long, or this function
// will panic.
fn try_keysize(keysize: usize, input: &[u8]) -> f64 {
    let fst_chunk = &input[..keysize];
    let snd_chunk = &input[keysize..keysize * 2];

    // If we have enough characters in the input, get a more accurate score
    // by taking the average of two normalised edit distances
    if input.len() >= keysize * 4 {
        let thrd_chunk = &input[keysize * 2..keysize * 3];
        let frth_chunk = &input[keysize * 3..keysize * 4];

        let dist12 = hamming_distance(fst_chunk, snd_chunk) as f64 / keysize as f64;
        let dist34 = hamming_distance(thrd_chunk, frth_chunk) as f64 / keysize as f64;
        let dist24 = hamming_distance(snd_chunk, frth_chunk) as f64 / keysize as f64;
        let dist13 = hamming_distance(fst_chunk, thrd_chunk) as f64 / keysize as f64;
        (dist12 + dist34 + dist24 + dist13) / 4.0
    } else {
        hamming_distance(fst_chunk, snd_chunk) as f64 / keysize as f64
    }
}

fn find_vigenere_keysize_candidates(input: &[u8]) -> Vec<u32> {
    let mut score_size_vec = Vec::<(f64, u32)>::new();

    for keysize in 2..=40 {
        let score = try_keysize(keysize, input);

        score_size_vec.push((score, keysize as u32));
    }

    // Sort by tuple first element in ascending order
    score_size_vec.sort_by(|a, b| a.0.total_cmp(&b.0));

    score_size_vec[..3] // Return the three best (lowest edit dist.) scores
        .to_vec()
        .into_iter()
        .map(|(_, b)| b)
        .collect()
}

pub fn group_by_key_char(keysize: usize, input: &[u8]) -> Vec<Vec<u8>> {
    let mut grouped_blocks: Vec<Vec<u8>> = Vec::with_capacity(keysize);
    for i in 0..keysize {
        let group: Vec<u8> = input
            .to_vec() // &[u8] -> Vec<u8>
            .into_iter() // Vec<u8> -> iterator over u8. `iter()` yields one over &u8.
            .skip(i) // skip the first `i` elements
            .step_by(keysize) // only care about every `keysize`-th element.
            .collect(); // collect all iterator elements into a vector

        grouped_blocks.push(group);
    }

    grouped_blocks
}

pub fn break_vigenere(input: &[u8]) -> Vec<u8> {
    let potential_keysizes = find_vigenere_keysize_candidates(input);
    let mut best_score = 0.0;
    let mut best_key = Vec::<u8>::new();
    // For each of the N best looking keysizes, divide the input text
    // into groups based on the keysize and treat it as a single byte
    // XOR cipher. The one with the best final score is the one we
    // return. A good score in this case is high.
    for pot_size in potential_keysizes {
        let mut cur_size_score = 0.0;
        let mut key = Vec::<u8>::with_capacity(pot_size as usize);

        let single_char_xors = group_by_key_char(pot_size as usize, input);

        for block in single_char_xors {
            let (key_byte, _, score) = set1::challenge3::decrypt_single_byte_xor_cipher(&block);
            cur_size_score += score;
            key.push(key_byte);
        }

        if cur_size_score > best_score {
            best_score = cur_size_score;
            best_key = key;
        };
    }

    best_key
}

pub fn solve_challenge_6(filepath: &str) -> Vec<u8> {
    let mut input_file = File::open(filepath).unwrap();
    let mut buf = Vec::<u8>::new();
    input_file.read_to_end(&mut buf).unwrap();

    let key = break_vigenere(&buf);

    key
}
