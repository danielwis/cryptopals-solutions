pub fn pad_with_pkcs7(input: &[u8], block_len: u8) -> Vec<u8> {
    // A remainder of n means that we are missing block_len - n bytes.
    // E.g. block_len = 20, input.len() = 16. We need 4 (20-16) more bytes.
    //
    // Remainder must be u8 as we pad the input vector with its "inverse",
    // and block_len must also be u8 as the maximum value of rem depends on it.
    // However, the input length is usize, so we cast block_len
    // to be able to find rem, then cast the result down to a u8.
    let bytes_missing = block_len - (input.len() % block_len as usize) as u8;
    let mut output = input.to_vec();

    for _ in 0..bytes_missing {
        output.push(bytes_missing);
    }

    output
}
