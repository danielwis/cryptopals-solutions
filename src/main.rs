use std::io::Write;

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
    let input_text = set1::challenge7::read_input_from_file("inputs/s1c7_nonb64.input");
    let decrypted = set1::challenge7::decrypt_ecb(&input_text, "YELLOW SUBMARINE".as_bytes());
    let decrypted_str = String::from_utf8(decrypted).unwrap();
    assert!(decrypted_str.contains("Supercalafragilisticexpialidocious"));

    // Challenge 8
    println!("Running challenge 8");
    let hexstrings = set1::helpers::read_input_file_as_bytes("inputs/s1c8.input");
    let mut ecb_strings_found = 0;
    for hexstr in hexstrings {
        if set1::challenge8::is_ecb(&hexstr) != None {
            ecb_strings_found += 1;
        }
    }
    assert!(ecb_strings_found == 1);

    println!("All trials passed for set 1!");
}

fn run_set_two() {
    // Challenge 9
    println!("Running challenge 9");
    let padded_output = set2::challenge9::pad_with_pkcs7("YELLOW SUBMARINE".as_bytes(), 20);
    let expected_output = "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes();
    assert_eq!(padded_output, expected_output);

    // Challenge 10 pre-tests
    println!("Running challenge 10 pre-tests");
    // ECB encrypt
    let text = "HELLO".as_bytes();
    let key = "YELLOW SUBMARINE".as_bytes();
    let enc_text = set2::challenge10::encrypt_ecb(text, key);
    let dec_enc_text = set1::challenge7::decrypt_ecb(&enc_text, key);
    assert_eq!(
        enc_text,
        vec![
            0x81, 0x15, 0x2c, 0xe9, 0x4b, 0x72, 0x62, 0x00, 0xb7, 0x27, 0x91, 0x43, 0xe6, 0xd8,
            0xf1, 0xc5
        ]
    );
    assert_eq!(dec_enc_text, text);

    // CBC encrypt/decrypt
    let text = "ugpasgjf asdvafgjsfdih gaf fd gf dh gfdioghdfihg idfhuga8fgajds fuid sfdsiu fds fdg ifdsu fgd gifdgs fgdsai".as_bytes();
    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = vec![0u8; 16];
    let enc_text = set2::challenge10::encrypt_cbc(text, key, &iv);
    //assert_eq!(expected_output, enc_text);
    //assert_eq!(expected_output_2, enc_text_2);
    let dec_enc_text = set2::challenge10::decrypt_cbc(&enc_text, key, &iv);
    assert_eq!(dec_enc_text, text);

    // Challenge 10
    println!("Running challenge 10");
    let input = set1::challenge7::read_input_from_file("inputs/s2c10_nonb64.input");
    let decrypted =
        set2::challenge10::decrypt_cbc(&input, "YELLOW SUBMARINE".as_bytes(), &vec![0u8; 16]);
    let decrypted_str = String::from_utf8(decrypted).unwrap();
    assert!(decrypted_str.contains("You're weakenin' fast, YO! and I can tell it"));

    // Challenge 11
    println!("Running challenge 11");
    // The ECB detection relies on there being multiple identical blocks,
    // i.e. multiple identical input blocks. Since the oracle pads with 5-10
    // bytes on each side, we need to have at least two blocks worth of the
    // same character, but also enough to fill potentially 15 extra bytes
    // (since adding one char as padding will render that whole block
    // "unusable" - we have no way of making the following block identical.
    // This means that we need 15 bytes + 32 bytes minimum, which is 47.
    let plaintext = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    assert!(plaintext.len() == 47);
    let encrypted_text = set2::challenge11::encrypt_ecb_or_cbc(plaintext);
    println!(
        "Detected mode: {}",
        if set1::challenge8::is_ecb(&encrypted_text) != None {
            "ECB"
        } else {
            "CBC"
        }
    );

    // Challenge 12
    println!("Running challenge 12");
    let unknown_decrypted = set2::challenge12::decrypt_the_unknown_string();
    let expected_output = "Rollin' in my 5.0\nWith my rag-top down so my hair can blow\nThe girlies on standby waving just to say hi\nDid you stop? No, I just drove by\n";
    let output = String::from_utf8(unknown_decrypted).unwrap();
    assert_eq!(expected_output, output);

    // Challenge 13
    println!("Running challenge 13");
    let encrypted_profile = set2::challenge13::make_admin_profile();
    let prof = set2::challenge13::decrypt_and_parse_profile(&encrypted_profile);
    assert!(prof.role == set2::challenge13::ProfileRole::Admin);

    // Challenge 14
    println!("Running challenge 14");
    for i in 1..16 {
        print!("\rTesting with random padding of size: {}", i);
        std::io::stdout().flush().expect("some error message");
        let random_bytes = set2::helpers::generate_random_bytes(i);
        let key = set2::helpers::generate_random_bytes(16);
        let unknown_decrypted = set2::challenge14::decrypt_the_unknown_string_with_random_padding(&key, &random_bytes);
        let expected_output = "Rollin' in my 5.0\nWith my rag-top down so my hair can blow\nThe girlies on standby waving just to say hi\nDid you stop? No, I just drove by\n";
        let output = String::from_utf8(unknown_decrypted).unwrap(); assert_eq!(expected_output, output);
    }
    println!();


    // Challenge 15
    println!("Running challenge 15");
    let valid_padding = b"ICE ICE BABY\x04\x04\x04\x04";
    let invalid_padding_1 = b"ICE ICE BABY\x05\x05\x05\x05";
    let invalid_padding_2 = b"ICE ICE BABY\x01\x02\x03\x04";
    assert!(set2::helpers::get_padding_chars(valid_padding) == Ok(4));
    assert_eq!(set2::helpers::get_padding_chars(invalid_padding_1), Err("Invalid padding"));
    assert_eq!(set2::helpers::get_padding_chars(invalid_padding_2), Err("Invalid padding"));


    println!("All trials passed for set 2!");
}

fn main() {
    if true {
        run_set_one();
    }
    if true {
        run_set_two();
    }
}
