const BASE_64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn chunk_to_bits(chunk: &[u8]) -> u32 {
    let mut chunk_as_bits: u32;
    chunk_as_bits = (chunk[0] as u32) << 16;
    for i in 0..chunk.len() {
        chunk_as_bits |= (chunk[i] as u32) << 16-(i*8);
    }

    chunk_as_bits
}

fn push_as_b64(output: &mut String, chunk: &[u8]) {
    // Merge chunk into one 24, 16, or 8-bit word (as u32 for ease of use)
    let chunk_as_bits = chunk_to_bits(chunk);

    // Match chunk len to how much we shift
    let num_output_chars = match chunk.len() {
        3 => 4,
        2 => 3,
        1 => 2,
        _ => unreachable!(),
    };

    // Push each of the characters to the string
    // The first one will be bits 24-19, then 18-13, 12-7 and 5-1 respectively
    // and which one we start with depends on num_output_chars.
    // Bitwise AND with 0x3f = 0b00111111, i.e. zero the top two bits as
    // we are only interested in six at a time, not eight.
    for i in 0..num_output_chars {
        let shift_amt = 18 - i * 6;
        let idx = ((chunk_as_bits >> shift_amt) & 0x3f) as usize;
        output.push(BASE_64_TABLE[idx]);
    }

    // Deal with any non-filled (but remaining) positions
    for _ in 0..4 - num_output_chars {
        output.push('=');
    }
}

pub fn hex_to_b64(input: &[u8]) -> String {
    // 3 octets -> 4 sextets
    let mut output: String = String::new();
    let chunks = input.chunks_exact(3);
    // Get the last chunk and the size of it
    let remaining_bytes = input.len() % 3;
    let remaining_chunk = &input[input.len() - remaining_bytes..];

    // Process all the "complete" chunks
    for chunk in chunks {
        push_as_b64(&mut output, chunk);
    }

    // Deal with remainder
    if remaining_bytes > 0 {
        push_as_b64(&mut output, remaining_chunk);
    }

    output
}
