use std::{fs::File, io::Read};

use openssl::symm::{decrypt, Cipher};

pub fn decrypt_with_known_key(filename: &str, key: &[u8]) -> Vec<u8> {
    assert!(key.len() == 16);

    let mut input_file = File::open(filename).unwrap();
    let mut data = Vec::<u8>::new();

    let cipher = Cipher::aes_128_ecb();
    input_file.read_to_end(&mut data).unwrap();

    let res = decrypt(cipher, key, None, &data).unwrap();

    res
}
