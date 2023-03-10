use rand::Rng;

use crate::set2;

pub fn encrypt_ecb_or_cbc(input: &[u8]) -> Vec<u8> {
    let key = set2::helpers::generate_random_bytes(16);
    let mut rng = rand::thread_rng();
    let num_rand_bytes = rng.gen_range(5..=10);

    // Pad with 5-10 random bytes before, and the same number of
    // (most likely different) random bytes afterwards.
    let mut processed_text = set2::helpers::generate_random_bytes(num_rand_bytes);
    processed_text.extend(input);
    processed_text.extend(set2::helpers::generate_random_bytes(num_rand_bytes));

    let use_ecb = rng.gen_bool(0.5);
    if use_ecb {
        println!("Oracle mode: ECB");
        set2::challenge10::encrypt_ecb(&processed_text, &key)
    } else {
        println!("Oracle mode: CBC");
        set2::challenge10::encrypt_cbc(&processed_text, &key, &set2::helpers::generate_random_bytes(16))
    }
}
