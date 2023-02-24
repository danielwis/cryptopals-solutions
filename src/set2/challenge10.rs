use crate::set1;
use crate::set2;

use aes::cipher::BlockEncrypt;
use aes::cipher::KeyInit;
use aes::{cipher::generic_array::GenericArray, Aes128};

pub fn encrypt_ecb_block(input: &mut [u8; 16], key: &[u8; 16]) {
    let cipher = Aes128::new(&GenericArray::from(*key));

    cipher.encrypt_block(input.into());
}

pub fn encrypt_ecb(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(key.len() == 16);
    let input_padded: Vec<u8> = set2::challenge9::pad_with_pkcs7(input, 16);
    let mut output = Vec::new();
    let key_strict: [u8; 16] = key.try_into().expect("Incorrect key size");

    // Decrypt each block
    for chunk in input_padded.chunks_exact(16) {
        let mut arr: [u8; 16] = chunk
            .try_into()
            .expect("Internal error: Incorrect block size");
        encrypt_ecb_block(&mut arr, &key_strict);

        output.extend(arr);
    }

    output
}

pub fn encrypt_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let block_size = 16;
    let input_padded: Vec<u8> = set2::challenge9::pad_with_pkcs7(input, block_size);
    let mut output: Vec<u8> = Vec::with_capacity(input.len() + block_size as usize);
    let mut iv_for_next_block = iv.to_vec();

    let key_strict = key.try_into().expect("Incorrect key length");

    // 1. XOR iv with next input block
    // 2. Encrypt XOR'd block
    // 3. Use this result as new iv
    for chunk in input_padded.chunks_exact(block_size as usize) {
        let mut input_xor: [u8; 16] = set1::challenge2::xor_slices(chunk, &iv_for_next_block)
            .try_into()
            .expect("Internal error: Incorrect block size."); // 1
        encrypt_ecb_block(&mut input_xor, key_strict); // 2
        iv_for_next_block = input_xor.to_vec(); // 3

        output.extend(input_xor);
    }

    output
}

pub fn decrypt_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    assert!(input.len() % 16 == 0);
    let block_size = 16;
    let mut output: Vec<u8> = Vec::new();
    let mut iv_for_next_block = iv.to_vec();

    let key_strict = key.try_into().expect("Incorrect key length");

    // 1. Decrypt input block with ECB
    // 2. XOR with previous iv (same order as when encrypting, i.e. same initial IV)
    // 3. Use original input block as the next IV
    for chunk in input.chunks_exact(block_size as usize) {
        let mut decrypted_chunk = chunk
            .clone()
            .try_into()
            .expect("Internal error: Incorrect block size.");
        set1::challenge7::decrypt_ecb_block(&mut decrypted_chunk, &key_strict); // 1
        let res = set1::challenge2::xor_slices(&decrypted_chunk, &iv_for_next_block); // 2
        iv_for_next_block = chunk.to_vec(); // 3

        output.extend(res);
    }

    // Handle padding
    let padding_chars = set2::helpers::get_padding_chars(&output).unwrap();

    output[..output.len() - padding_chars as usize].to_vec()
}
