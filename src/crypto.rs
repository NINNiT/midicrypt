use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use orion::{
    hazardous::hash::sha2::sha256::Sha256,
    kdf::{self, SecretKey},
};

pub fn kdf_argon2(message: Vec<u8>) -> SecretKey {
    const SALT_BYTES: [u8; 16] = [
        227, 104, 185, 184, 220, 28, 170, 74, 171, 172, 137, 57, 177, 118, 32, 125,
    ];

    let pw = kdf::Password::from_slice(&message.as_slice()).unwrap();
    let salt: kdf::Salt = kdf::Salt::from_slice(&SALT_BYTES).unwrap();
    let derived_key = kdf::derive_key(&pw, &salt, 3, 1 << 16, 32).unwrap();
    derived_key
}

pub fn _hash_bytes_sha256(message: Vec<u8>) -> Vec<u8> {
    let hash = Sha256::digest(&message).unwrap().as_ref().to_vec();
    hash
}

//encrypt file using orion aead and custom secret key
pub fn encrypt_file(
    input_file: &Path,
    output_file: &Path,
    secret_key: &SecretKey,
) -> Result<(), io::Error> {
    let mut file = File::open(input_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut file = File::create(output_file)?;
    let mut ciphertext = orion::aead::seal(&secret_key, &contents.as_bytes()).unwrap();
    file.write_all(&mut ciphertext)?;

    println!(
        "Encrypted input file '{}' to output file '{}'",
        input_file.display(),
        output_file.display()
    );
    Ok(())
}

pub fn decrypt_file(
    input_file: &Path,
    output_file: &Path,
    secret_key: &SecretKey,
) -> Result<(), io::Error> {
    let mut file = File::open(input_file)?;
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    let mut file = File::create(output_file)?;
    let mut plaintext = orion::aead::open(&secret_key, &ciphertext).unwrap();
    file.write_all(&mut plaintext)?;

    println!(
        "Decrypted input file '{}' to output file '{}'",
        input_file.display(),
        output_file.display()
    );
    Ok(())
}
