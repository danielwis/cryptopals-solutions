use std::{fs::File, io::Read};

use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit},
    Aes128,
};

use crate::set2;

pub fn decrypt_ecb_block(input: &mut [u8; 16], key: &[u8; 16]) {
    let cipher = Aes128::new(&GenericArray::from(*key));

    cipher.decrypt_block(input.into());
}

pub fn decrypt_ecb(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(key.len() == 16);
    let mut output = Vec::new();
    let key_strict: [u8; 16] = key.try_into().expect("Incorrect key size");

    // Decrypt each block
    for chunk in input.chunks_exact(16) {
        // Convert to [u8; 16]
        let mut arr = chunk
            .try_into()
            .expect("Internal error: Incorrect key size");
        decrypt_ecb_block(&mut arr, &key_strict);

        output.extend(arr);
    }

    // Handle padding
    let padding_chars = set2::helpers::get_padding_chars(&output);

    output[..output.len() - padding_chars as usize].to_vec()
}

pub fn read_input_from_file(filename: &str) -> Vec<u8> {
    let mut input_file = File::open(filename).unwrap();
    let mut data = Vec::<u8>::new();
    input_file.read_to_end(&mut data).unwrap();

    data
}
