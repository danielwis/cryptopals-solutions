use crate::{set1, set2};

use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;

fn prepend_random_encrypt_ecb_and_append_unknown_string(input: &[u8], key: &[u8], random_bytes: &[u8]) -> Vec<u8> {
    let unknown_string_b64 = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let unknown_string_as_bytes = general_purpose::STANDARD_NO_PAD
        .decode(unknown_string_b64)
        .unwrap();

    // Pad with 5-10 random bytes before, and the same number of
    // (most likely different) random bytes afterwards.
    let mut processed_text = random_bytes.to_vec();
    processed_text.extend(input);
    processed_text.extend(unknown_string_as_bytes);

    set2::challenge10::encrypt_ecb(&processed_text, key)
}

fn calculate_padding_length(block_size: usize, key: &[u8], random_bytes: &[u8]) -> usize {
    let mut two_identical_blocks = set2::helpers::generate_random_bytes(block_size as u32);
    two_identical_blocks.extend(two_identical_blocks.clone()); // Add it to itself

    for i in 0..block_size {
        // input_vec is of the shape [0,0,...,0,identical_block,identical_block]
        let mut input_vec = vec![0x0; i];
        input_vec.extend(&two_identical_blocks);

        if let Some(idx) = set1::challenge8::is_ecb(
            &prepend_random_encrypt_ecb_and_append_unknown_string(&input_vec, key, random_bytes),
        ) {
            // Block `idx` is the first equal block, so we get block `idx - 1` as the last padded
            // block. This has `i` bytes of extra padding by us, and so the random padding is of
            // length block_size * idx - i.
            return block_size * idx - i;
        }
    }

    // If we get here, we're most likely not using ECB (or something else is wrong)
    unreachable!("Could not find any repetitions despite two identical blocks sent in. Are you sure the oracle is using ECB?");
}

fn ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

// ----------
// This is heavily based on the function from challenge12; refer to the comments
// in the decrypt_the_unknown_string function there for further explanations.
// ----------
pub fn decrypt_the_unknown_string_with_random_padding(key: &[u8], random_bytes: &[u8]) -> Vec<u8> {
    // Get the block size of the oracle function's cipher,
    // as well as the size of the secret text
    let mut i = 0;
    let (crypto_block_size, secret_text_and_padding_length) = loop {
        let curr_size = prepend_random_encrypt_ecb_and_append_unknown_string(&vec![0u8; i], key, random_bytes).len();
        // Test next size
        i += 1;
        let next_size = prepend_random_encrypt_ecb_and_append_unknown_string(&vec![0u8; i], key, random_bytes).len();

        // Secret text length is no longer curr_size - (i-1) as in challenge 12, since we now have
        // extra padding to think about as well. curr_size - (i-1) gives the lengths of the secret
        // text and padding combined.
        if curr_size != next_size {
            break (next_size - curr_size, curr_size - (i - 1));
        }
    };

    // Check that the encrypted string is properly detected as ECB.
    // This means that the oracle function is using ECB.
    assert!(
        set1::challenge8::is_ecb(&prepend_random_encrypt_ecb_and_append_unknown_string(
            &vec![0x61; 48], key, random_bytes
        )) != None
    );

    // Get the length of the random padding at the beginning.
    // Now, we just need to "ignore" it, which can be done by padding
    // our input so that we always have `n` blocks of padding, then just
    // changing the block_to_cmp_idx to ignore those blocks
    let random_padding_len = calculate_padding_length(crypto_block_size, key, random_bytes);
    let secret_text_length = secret_text_and_padding_length - random_padding_len;
    // Padding should complement the random-prefix so that the padding has its own blocks
    let padding = vec![0x0; crypto_block_size - (random_padding_len % crypto_block_size)];
    let blocks_to_ignore = ceil(random_padding_len, crypto_block_size);

    let mut secret_text = Vec::<u8>::with_capacity(secret_text_length);
    for revealed_bytes in 0..secret_text_length {
        let block_to_cmp_idx = (revealed_bytes / crypto_block_size) + blocks_to_ignore;

        // Make our n_bytes_short vector take the padding into account.
        let mut n_bytes_short = padding.clone();
        n_bytes_short.extend(vec![
            0x61;
            crypto_block_size
                - ((revealed_bytes % crypto_block_size) + 1)
        ]);

        let mut n_bytes_short_filled = n_bytes_short.clone();
        n_bytes_short_filled.extend(&secret_text);

        let mut output_values_for_bytes = HashMap::<u8, Vec<u8>>::new();
        for i in 0..=255 {
            let mut potential_match = n_bytes_short_filled.clone();
            potential_match.push(i);
            output_values_for_bytes.insert(
                i,
                prepend_random_encrypt_ecb_and_append_unknown_string(&potential_match, key, random_bytes),
            );
        }

        // Get the "real" output
        let oracle_output = prepend_random_encrypt_ecb_and_append_unknown_string(&n_bytes_short, key, random_bytes);

        // Compare the real output to all our potential ones.
        // TODO: Padding of 0 makes every single byte match...
        for (k, v) in output_values_for_bytes {
            let our_block = &v
                [block_to_cmp_idx * crypto_block_size..(block_to_cmp_idx + 1) * crypto_block_size];
            let oracle_block = &oracle_output
                [block_to_cmp_idx * crypto_block_size..(block_to_cmp_idx + 1) * crypto_block_size];

            if our_block == oracle_block {
                secret_text.push(k);
            }
        }
    }

    secret_text
}
