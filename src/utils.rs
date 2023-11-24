/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

use crate::config::get_hash_path;
use crate::display_msg::DisplayMsg;
use age::secrecy::Secret;
use argon2;
use rand::{distributions::Alphanumeric, rngs::OsRng, thread_rng, Rng, RngCore};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

// `login` read password from console to compute encoded string for keypair file decryption.
// This function computes the argon2 hash of the encoded password and verify with the record saved before.
//  # Arguments
//  *
pub(crate) fn login() -> Result<String, DisplayMsg> {
    let argon2_config = argon2::Config::default();
    let mut salt = read_file(get_hash_path()).map_err(|e| {
        DisplayMsg::FailToOpenOrReadFile(String::from("hash file"), get_hash_path(), e)
    })?;
    let hash = salt.split_off(32);

    // try to decrypt with empty_pasword by default
    let encoded_empty_pasword = base64url::encode("");
    if let Ok(true) = argon2::verify_raw(
        encoded_empty_pasword.as_bytes(),
        &salt,
        &hash,
        &argon2_config,
    ) {
        return Ok(encoded_empty_pasword);
    }

    let password = rpassword::prompt_password("password: ")
        .unwrap()
        .trim()
        .to_string();
    let encoded_password = base64url::encode(password);

    match argon2::verify_raw(encoded_password.as_bytes(), &salt, &hash, &argon2_config) {
        Ok(true) => Ok(encoded_password),
        Ok(false) => Err(DisplayMsg::WrongPassword),
        Err(_) => Err(DisplayMsg::PasswordFilesContaminated),
    }
}

// `setup_password` get user input password and hashed with argon2. The salt used in here is 32 random bytes.
// The salt and output hash would be concatenated and save to a file. The password hash is used to verfy user
// password later.
//  # Arguments
//  *
pub(crate) fn setup_password() -> Result<(), DisplayMsg> {
    println!("First time to use ParallelChain Client CLI. Please setup password to protect you keypairs.");
    let password1 =
        rpassword::prompt_password("Your password: (press enter to skip password protection.)")
            .unwrap()
            .trim()
            .to_string();
    if !password1.is_empty() {
        let password2 = rpassword::prompt_password("Re-enter your password: ")
            .unwrap()
            .trim()
            .to_string();
        if password1 != password2 {
            return Err(DisplayMsg::PasswordNotMatch);
        }
    }

    let encoded_password = base64url::encode(password1);
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);

    let argon2_config = argon2::Config::default();
    let key = argon2::hash_raw(encoded_password.as_bytes(), &salt, &argon2_config)
        .map_err(|e| DisplayMsg::FailToSetupPassword(e.to_string()))?;

    let mut data = salt.to_vec();
    data.extend_from_slice(&key);
    match write_file(get_hash_path(), &data) {
        Ok(_) => {
            println!("{}", DisplayMsg::SuccessSetupPassword);
            Ok(())
        }
        Err(e) => Err(DisplayMsg::FailToWriteFile(
            String::from("hash file"),
            get_hash_path(),
            e,
        )),
    }
}

// `encrypt` implement data encryption to create an age file.
//  # Arguments
//  * `source` - raw data in bytes
pub(crate) fn encrypt(source: &[u8]) -> Result<Vec<u8>, DisplayMsg> {
    let encoded_passphrase = login()?;

    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(encoded_passphrase));

        let mut encrypted = vec![];
        let mut writer = encryptor
            .wrap_output(&mut encrypted)
            .map_err(|e| DisplayMsg::FailtoEncrypt(e.to_string()))?;
        writer
            .write_all(source)
            .map_err(|e| DisplayMsg::FailtoEncrypt(e.to_string()))?;
        writer
            .finish()
            .map_err(|e| DisplayMsg::FailtoEncrypt(e.to_string()))?;

        encrypted
    };

    Ok(encrypted)
}

// `decrypt` implement data decryption from age file to original bytes.
//  # Arguments
//  * `source` - encrypted data in bytes
pub(crate) fn decrypt(source: &[u8]) -> Result<Vec<u8>, DisplayMsg> {
    let encoded_passphrase = login()?;

    let decrypted = {
        let decryptor = match age::Decryptor::new(source)
            .map_err(|e| DisplayMsg::FailtoDecrypt(e.to_string()))?
        {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };

        let mut decrypted = vec![];
        let mut reader = decryptor
            .decrypt(&Secret::new(encoded_passphrase), None)
            .map_err(|e| DisplayMsg::FailtoDecrypt(e.to_string()))?;
        let _ = reader.read_to_end(&mut decrypted);

        decrypted
    };

    Ok(decrypted)
}

// `read_file_to_utf8string` reads json File into stringified JSON
// # Arguments
// * `path_to_json` - absolute path to the JSON file
pub(crate) fn read_file_to_utf8string(path: PathBuf) -> Result<String, String> {
    let data = read_file(path)?;
    match String::from_utf8(data) {
        Ok(stringified_file) => Ok(stringified_file),
        Err(e) => Err(format!("File content is utf8 invalid. {}", e)),
    }
}

// `read_file` is a helper which reads a file to a vector of bytes from the path provided
// # Arguments
// * `path_to_file` - absolute path to keypair.json file
pub(crate) fn read_file(path_to_file: PathBuf) -> Result<Vec<u8>, String> {
    if path_to_file.is_file() {
        match std::fs::read(&path_to_file) {
            Ok(data) => Ok(data),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err(String::from("Provided path is not a file"))
    }
}

// `write_file` is a helper which write a vector of bytes to the provide provided
pub fn write_file(path_to_file: PathBuf, content: &[u8]) -> Result<String, String> {
    if path_to_file.is_dir() {
        return Err(String::from("Providede path is a directory."));
    }

    let mut file = std::fs::File::create(path_to_file.clone()).map_err(|e| e.to_string())?;
    file.write(content).map_err(|e| e.to_string())?;

    Ok(dunce::canonicalize(path_to_file)
        .unwrap()
        .into_os_string()
        .into_string()
        .ok()
        .unwrap())
}

// get_random_string generates a rndom string.
// for naming the docker container.
//  # Arguments
//  *
pub fn get_random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}
