pub fn get_padding_chars(padded_vec: &[u8]) -> u8 {
    let mut is_padding = true;
    let pad_char: u8 = padded_vec[padded_vec.len()-1];
    // Assuming we don't pad padded_vec that has a length = 0 mod 16
    if pad_char < 16 {
        for i in 1..pad_char {
            if padded_vec[padded_vec.len() - i as usize] != pad_char {
                is_padding = false;
            }
        }
    }

    if is_padding {
        pad_char
    } else {
        0
    }
}
