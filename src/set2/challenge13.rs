use crate::{set1, set2};

const CRYPTO_KEY: [u8; 16] = [
    89, 133, 145, 209, 74, 60, 202, 175, 251, 253, 231, 8, 81, 166, 77, 15,
];

#[derive(Debug, PartialEq)]
pub enum ProfileRole {
    User,
    Admin,
}

pub struct Profile {
    email: String,
    uid: u32,
    pub role: ProfileRole,
}

impl Profile {
    fn new() -> Self {
        Profile {
            email: "".to_string(),
            uid: 0,
            role: ProfileRole::User,
        }
    }

    fn encode(self) -> String {
        format!("email={}&uid={}&role={:?}", self.email, self.uid, self.role)
    }
}

// Perhaps this should have simply been "make object" with arbitrary
// keys and values, but that would be a whole lot of extra debugging
// and searching to be able to instantiate arbitrary objects.
fn make_profile_object(encoded_str: &str) -> Profile {
    // Get key=value pairs, ignoring any other stuff on the side
    let attrs: Vec<&str> = encoded_str
        .split('&')
        .filter(|str| str.contains("="))
        .collect();
    let mut prof = Profile::new();

    for attr in attrs {
        let key = *attr
            .split('=')
            .collect::<Vec<&str>>()
            .get(0)
            .expect("Empty key???");
        let value = *attr
            .split('=')
            .collect::<Vec<&str>>()
            .get(1)
            .expect("Empty value???");

        match key {
            "email" => {
                prof.email = value.to_string();
            }
            "uid" => {
                prof.uid = value
                    .parse()
                    .expect(&format!("Invalid uid, can't convert to u32: {value}"));
            }
            "role" => {
                prof.role = match value {
                    "user" => ProfileRole::User,
                    "admin" => ProfileRole::Admin,
                    _ => panic!("Invalid role type! Got: {}.", value),
                }
            }
            _ => {
                println!(
                    "Ignoring invalid key. Expected \"email\", \"uid\" or \"role\", got {}",
                    key
                );
            }
        };
    }

    prof
}

fn profile_for(email: &str) -> String {
    // Remove metacharacters
    let email = email.replace('=', "").replace('&', "");

    make_profile_object(&format!("email={email}&uid=10&role=user")).encode()
}

fn encrypt_user_profile(input: &str) -> Vec<u8> {
    set2::challenge10::encrypt_ecb(profile_for(input).as_bytes(), &CRYPTO_KEY)
}

pub fn decrypt_and_parse_profile(input: &[u8]) -> Profile {
    let decrypted_str =
        String::from_utf8(set1::challenge7::decrypt_ecb(input, &CRYPTO_KEY)).unwrap();

    make_profile_object(&decrypted_str)
}

// Post-coding realisation: we could automate this, sending increasingly long inputs until
// we have a plaintext block ending with "&role=" and then encrypting that profile, extracting
// the block. Then do the same thing with admin. Might implement later, but this should show
// enough understanding anyway.
//
// We are only allowed to call `profile_for` (and `encrypt_user_profile`, it seems), i.e.
// get plaintext and crypto (chosen plaintext-attack?).
// We need to generate a profile with `uid=10&role=admin`. The easiest thing would be if
// we were able to get the ciphertext for a `uid=1&role=admin` query, and just paste that in
// alongside some other text to get a full admin string. Since we can't encode metacharacters
// though, we need to split them up. We only need it to decrypt to ...&role=admin..., i.e. we
// don't care what the uid or email ends up being.
// For example, if we manage to get a ciphertext block ending with `&role=`, and another
// beginning with `admin`, we can put these together and make a nice `&role=admin` query.
pub fn make_admin_profile() -> Vec<u8> {
    // This will encrypt to "email=...&uid=10&role=user", so we use an email that will end our
    // first block off with "&role=". "email=" is 6 bytes, "&uid=10" is 7, and "&role=" is 6.
    // We thus have 13 necessary bytes and can't fit the role part in the first block, meaning that
    // we need block_size - 6 bytes as padding for the second block and block_size - (6+7) bytes
    // for the first block. With block_size = 16 we get 10+3 = 13 bytes long e-mail.
    // This will give us the second block ending with "&role=".
    let clever_query_role_part = "1234567890123";

    // To start a block with "admin", we don't have to do as much (since we don't need any
    // metacharacters). We know that the "email=" part is 6 chars, so we use 10 bytes of
    // padding and then "admin".
    let clever_query_admin_part = "1234567890admin";

    // Extract the second block of the role part
    let encrypted_role_part = &encrypt_user_profile(&clever_query_role_part)[16..32];
    // ...and the second of the admin part
    let encrypted_admin_part = &encrypt_user_profile(&clever_query_admin_part)[16..32];

    // We now have "123&uid=10&role=admin&uid=10&rol", which will give us an object with
    // uid 10, role admin and email empty. We could always add an email if we wanted to
    // by making it 10 chars long ("email=..." would be one block) and prepending it to
    // this string, so let's do that to get rid of the pesky 123 at the start.
    let encrypted_extra_email_part = &encrypt_user_profile("ten_chars!")[..16];

    let mut encrypted_forgery_masterpiece = encrypted_extra_email_part.to_vec();
    encrypted_forgery_masterpiece.extend(encrypted_role_part);
    encrypted_forgery_masterpiece.extend(encrypted_admin_part);

    // Finally, we want this to be valid encryption, i.e. of a length that's a multiple
    // of block_size.
    if encrypted_forgery_masterpiece.len() % 16 != 0 {
        // Pad with arbitrary values. We don't care what comes out at the end anyway.
        for _ in 0..16 - (encrypted_forgery_masterpiece.len() % 16) {
            encrypted_forgery_masterpiece.push(0x0);
        }
    }

    encrypted_forgery_masterpiece
}
