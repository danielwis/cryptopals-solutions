mod set1;
mod set2;

fn run_set_one() {
    // Challenge 1
    println!("Running challenge 1");
    let input_first = [
        0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f, 0x75,
        0x72, 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20,
        0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x68, 0x72,
        0x6f, 0x6f, 0x6d,
    ];
    let output_expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(set1::challenge1::hex_to_b64(&input_first), output_expected);

    // Challenge 2
    println!("Running challenge 2");
    let input1 = [
        0x1c, 0x01, 0x11, 0x00, 0x1f, 0x01, 0x01, 0x00, 0x06, 0x1a, 0x02, 0x4b, 0x53, 0x53, 0x50,
        0x09, 0x18, 0x1c,
    ];
    let input2 = [
        0x68, 0x69, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x75, 0x6c, 0x6c, 0x27, 0x73, 0x20,
        0x65, 0x79, 0x65,
    ];
    let output_expected = [
        0x74, 0x68, 0x65, 0x20, 0x6b, 0x69, 0x64, 0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x70,
        0x6c, 0x61, 0x79,
    ];
    assert_eq!(
        set1::challenge2::xor_slices(&input1, &input2),
        output_expected.to_vec()
    );

    // Challenge 3
    println!("Running challenge 3");
    let input = [
        0x1b, 0x37, 0x37, 0x33, 0x31, 0x36, 0x3f, 0x78, 0x15, 0x1b, 0x7f, 0x2b, 0x78, 0x34, 0x31,
        0x33, 0x3d, 0x78, 0x39, 0x78, 0x28, 0x37, 0x2d, 0x36, 0x3c, 0x78, 0x37, 0x3e, 0x78, 0x3a,
        0x39, 0x3b, 0x37, 0x36,
    ];
    let (_, ans, _) = set1::challenge3::decrypt_single_byte_xor_cipher(&input);
    let ans = String::from_utf8(ans).unwrap();
    assert_eq!(ans, "Cooking MC's like a pound of bacon".to_owned());

    // Challenge 4
    println!("Running challenge 4");
    let input_filename = "inputs/s1c4.input";
    let ans = set1::challenge4::find_single_char_xor_ciphertext(&input_filename);
    assert_eq!(ans, "Now that the party is jumping\n".to_owned());

    // Challenge 5
    println!("Running challenge 5");
    let input =
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
    let key = "ICE".as_bytes();
    let ans = set1::challenge5::repeating_key_xor(input, key);
    let output_expected = [
        0x0b, 0x36, 0x37, 0x27, 0x2a, 0x2b, 0x2e, 0x63, 0x62, 0x2c, 0x2e, 0x69, 0x69, 0x2a, 0x23,
        0x69, 0x3a, 0x2a, 0x3c, 0x63, 0x24, 0x20, 0x2d, 0x62, 0x3d, 0x63, 0x34, 0x3c, 0x2a, 0x26,
        0x22, 0x63, 0x24, 0x27, 0x27, 0x65, 0x27, 0x2a, 0x28, 0x2b, 0x2f, 0x20, 0x43, 0x0a, 0x65,
        0x2e, 0x2c, 0x65, 0x2a, 0x31, 0x24, 0x33, 0x3a, 0x65, 0x3e, 0x2b, 0x20, 0x27, 0x63, 0x0c,
        0x69, 0x2b, 0x20, 0x28, 0x31, 0x65, 0x28, 0x63, 0x26, 0x30, 0x2e, 0x27, 0x28, 0x2f,
    ];
    assert_eq!(ans, output_expected);

    // Challenge 6 pre-tests
    // Hamming distance
    println!("Running partial tests for challenge 6");
    assert_eq!(
        set1::challenge6::hamming_distance(
            "this is a test".as_bytes(),
            "wokka wokka!!!".as_bytes()
        ),
        37
    );

    // Break into n chunks
    assert_eq!(
        set1::challenge6::group_by_key_char(3, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        vec![vec![1, 4, 7, 10], vec![2, 5, 8], vec![3, 6, 9]]
    );

    // Challenge 6 main challenge
    println!("Running challenge 6");
    let key: Vec<u8> = set1::challenge6::solve_challenge_6("inputs/s1c6_nonb64.input");
    assert_eq!(
        key,
        [
            84, 101, 114, 109, 105, 110, 97, 116, 111, 114, 32, 88, 58, 32, 66, 114, 105, 110, 103,
            32, 116, 104, 101, 32, 110, 111, 105, 115, 101,
        ]
    );

    // Challenge 7
    println!("Running challenge 7");
    let decrypted = set1::challenge7::decrypt_with_known_key(
        "inputs/s1c7_nonb64.input",
        "YELLOW SUBMARINE".as_bytes(),
    );
    let decrypted_str = String::from_utf8(decrypted).unwrap();
    assert!(decrypted_str.contains("Supercalafragilisticexpialidocious"));

    // Challenge 8
    println!("Running challenge 8");
    let hexstrings = set1::helpers::read_input_file_as_bytes("inputs/s1c8.input");
    let mut ecb_strings_found = 0;
    for hexstr in hexstrings {
        if set1::challenge8::is_ecb(&hexstr) {
            //println!("Found it! At least one 16-byte block occurs twice in the following vector: {:?}", hexstr);
            ecb_strings_found += 1;
        }
    }
    assert!(ecb_strings_found == 1);

    println!("All trials passed!");
}

fn run_set_two() {
    // Challenge 9
    let padded_output = set2::challenge9::pad_with_pkcs7("YELLOW SUBMARINE".as_bytes(), 20);
    let expected_output = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes();
    assert_eq!(padded_output, expected_output);
}

fn main() {
    if false {
        run_set_one();
    }
    if true {
        run_set_two();
    }
}
