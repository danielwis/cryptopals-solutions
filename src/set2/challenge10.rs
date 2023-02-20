use crate::set1;
use crate::set2;

use openssl::symm::{encrypt, Cipher};

pub fn encrypt_with_known_key(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(key.len() == 16);
    let cipher = Cipher::aes_128_ecb();

    let res = encrypt(cipher, key, None, input).unwrap();
    dbg!(res.clone());
    res
}

pub fn encrypt_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let block_len = 16;
    println!("Len: {}", input.len());
    let input_padded: Vec<u8> = set2::challenge9::pad_with_pkcs7(input, block_len);
    println!("Len apdded: {}", input_padded.len());
    let mut output: Vec<u8> = Vec::new();
    let mut iv_for_next_block = iv.to_vec();

    // 1. XOR iv with next input block
    // 2. Encrypt XOR'd block
    // 3. Use this result as new iv
    for chunk in input_padded.chunks_exact(block_len as usize) {
        dbg!(chunk);
        let input_xor = set1::challenge2::xor_slices(chunk, &iv_for_next_block); // 1
        let ecb_res = encrypt_with_known_key(&input_xor, key); // 2
        iv_for_next_block = ecb_res.clone(); // 3

        println!("{}", output.len());
        output.extend(ecb_res);
        println!("{}",output.len());
    }

    output
}

pub fn decrypt_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    assert!(input.len() % 16 == 0);

    let block_len = 16;
    let mut output: Vec<u8> = Vec::new();
    let mut iv_for_next_block = iv.to_vec();

    // 1. Decrypt input block with ECB
    // 2. XOR with previous iv (same order as when encrypting, i.e. same initial IV)
    // 3. Use original input block as the next IV
    for chunk in input.chunks_exact(block_len) {
        let ecb_res = set1::challenge7::decrypt_with_known_key(chunk, key); // 1
        let res = set1::challenge2::xor_slices(&ecb_res, &iv_for_next_block); // 2
        iv_for_next_block = chunk.to_vec(); // 3

        output.extend(res);
    }

    output
}
