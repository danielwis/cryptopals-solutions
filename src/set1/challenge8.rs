pub fn is_ecb(input: &[u8]) -> bool {
    assert!(input.len() % 16 == 0);

    // Check if two blocks are the same (i.e. two input sequences are the same)
    let chunks = input.len() / 16;
    for i in 0..chunks-1 {
        for j in i+1..chunks-1 {
            if input[i*16..(i+1)*16] == input[j*16..(j+1)*16] {
                return true;
            }
        }
    }

    return false;
}
