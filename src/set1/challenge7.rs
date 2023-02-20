use std::{fs::File, io::Read};

use openssl::symm::{decrypt, Cipher};

pub fn decrypt_with_known_key(input: &[u8], key: &[u8]) -> Vec<u8> {
    dbg!(input);
    assert!(key.len() == 16);
    assert!(input.len() % 16 == 0);
    let cipher = Cipher::aes_128_ecb();

    println!("Decrypting. Length: {}", input.len());
    let res = decrypt(cipher, key, None, input).unwrap();

    res
}

pub fn read_input_from_file(filename: &str) -> Vec<u8> {
    let mut input_file = File::open(filename).unwrap();
    let mut data = Vec::<u8>::new();
    input_file.read_to_end(&mut data).unwrap();

    data
}
