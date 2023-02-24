use rand::Rng;

pub fn get_padding_chars(padded_vec: &[u8]) -> Result<u8, &str> {
    let mut is_padding = true;
    let pad_char: u8 = padded_vec[padded_vec.len() - 1];
    // Assuming we don't pad padded_vec that has a length = 0 mod 16
    if pad_char < 16 {
        for i in 1..=pad_char {
            if padded_vec[padded_vec.len() - i as usize] != pad_char {
                is_padding = false;
            }
        }

        // We have padding
        if is_padding {
            return Ok(pad_char);
        } else {
            // We either have too big a char for it to be padding,
            // or it doesn't fit the specification for pkcs7.
            return Err("Invalid padding");
        }
    }

    Ok(0)
}

pub fn generate_random_bytes(n: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..=255)).collect()
}
