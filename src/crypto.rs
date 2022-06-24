use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use orion::hazardous::hash::sha2::sha256::Sha256;

pub fn hash_bytes_sha256(message: Vec<u8>) -> Vec<u8> {
    let hash = Sha256::digest(&message).unwrap().as_ref().to_vec();
    hash
}

//encrypt file using orion aead and custom secret key
pub fn encrypt_file(
    input_file: &Path,
    output_file: &Path,
    secret_key: Vec<u8>,
) -> Result<(), io::Error> {
    let mut file = File::open(input_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut file = File::create(output_file)?;
    let cipher = orion::aead::SecretKey::from_slice(secret_key.as_slice()).unwrap();
    let mut ciphertext = orion::aead::seal(&cipher, &contents.as_bytes()).unwrap();
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
    secret_key: Vec<u8>,
) -> Result<(), io::Error> {
    let mut file = File::open(input_file)?;
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;

    let mut file = File::create(output_file)?;
    let cipher = orion::aead::SecretKey::from_slice(secret_key.as_slice()).unwrap();
    let mut plaintext = orion::aead::open(&cipher, &ciphertext).unwrap();
    file.write_all(&mut plaintext)?;

    println!(
        "Decrypted input file '{}' to output file '{}'",
        input_file.display(),
        output_file.display()
    );
    Ok(())
}
