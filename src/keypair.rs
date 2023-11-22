/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Definition of methods related to serde serializable/deserializable version of  `pchain_types::Keypair`.

use std::{path::PathBuf, fs::File};

use crate::config::{get_home_dir, get_keypair_path};
use crate::display_msg::DisplayMsg;
use crate::utils;

/// [KeypairJSON] wraps around serde serializable/deserializable 
/// representation of pchain_types::Keypair which is used for 
/// storing ParallelChain account specific infomation on your filesystem. 
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct KeypairJSON {
    pub name: String,
    pub private_key: String,
    pub public_key: String,
    pub keypair: String,
}

// `setup_keypair_file` sets up a keypair file on the defalt keypair path
// 
// if keypair file does not exist, create and empty keypair file.
// if keypair file exist and is of correct format, do nothing. 
// if keypair file exist but is of incorrect json format, wipe out the keypair file and create a new one
//  # Arguments
//  * 
pub fn setup_keypair_file() {
    let default_keypair_dir = get_home_dir();
    if !default_keypair_dir.exists() {
        std::fs::create_dir_all(&default_keypair_dir).expect(&DisplayMsg::FailToCreateDir(String::from("Parallelchain Client Home"), default_keypair_dir,  String::new()).to_string());
    }
    if !get_keypair_path().exists(){
        File::create(get_keypair_path())
                .expect(&DisplayMsg::FailToCreateFile(String::from("Parallelchain Client Home"), get_keypair_path(),  String::new()).to_string());
        return
    }
    if let Err(e) = load_existing_keypairs(get_keypair_path()){
        println!("{}", e);
        std::process::exit(1);
    }
}

// `get_keypair_from_json` accepts a path to keypair JSON and generates a keypair.
//  # Arguments
//  * `path_to_keypair_json` - path to keypair JSON file
//  * `keypair_name` - name of the keypair
//  
pub fn get_keypair_from_json(path_to_keypair_json: PathBuf, keypair_name: &str) -> Result<Option<KeypairJSON>, DisplayMsg> {
    let keypairs = match load_existing_keypairs(path_to_keypair_json){
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match keypairs.into_iter().find(|keypair_json|{
        keypair_json.name == keypair_name
    }){
        Some(kp) => {
            Ok(Some(kp))
        },                
        None => Ok(None),
    }
}

// `load_existing_keypairs` accepts a path to keypair JSON and reads the keypairs on file to a list.
//  # Arguments
//  * `path_to_keypair_json` - path to keypair JSON file
//  
pub fn load_existing_keypairs(path_to_keypair_json: PathBuf) -> Result<Vec<KeypairJSON>, DisplayMsg> {
    let keypair_base64_string = if path_to_keypair_json.is_file() {
        match utils::read_file(path_to_keypair_json.clone()) {
            Ok(encrypt_bytes) => {
                if encrypt_bytes.is_empty(){
                    return Ok(Vec::new())
                }
                let json = utils::decrypt(&encrypt_bytes)?;
                match serde_json::from_slice::<Vec<KeypairJSON>>(&json){
                    Ok(kp_json_vec) => kp_json_vec,
                    Err(e) => return Err(DisplayMsg::ParseKeypairFailure(e))
                }
            },
            Err(e) => {
                return Err(DisplayMsg::FailToOpenOrReadFile(String::from("keypair json"), path_to_keypair_json, e));
            }
        }
    } else {
        return Err(DisplayMsg::IncorrectFilePath(String::from("keypair json"), path_to_keypair_json, String::from("Provided path is not a file.")))
    };

    Ok(keypair_base64_string)
}

// `generate_keypair` generates a new serde serializable deserialzable keypair.
//  # Arguments
//  * `keypair_name` - name of the keypair saved on the JSON file
//
pub fn generate_keypair(keypair_name: &str) -> KeypairJSON {
    use rand::rngs::OsRng;
    use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
    
    let mut osrng = OsRng{};
    let mut chacha20_rng = ChaCha20Rng::from_rng(&mut osrng).unwrap();
    let keypair = pchain_types::cryptography::Keypair::generate(&mut chacha20_rng);


    let secret = keypair.as_bytes();
    let verifying = keypair.verifying_key().clone();
    let public = verifying.as_bytes();

    KeypairJSON {
        name: keypair_name.to_string(),
        private_key: base64url::encode(secret),
        public_key: base64url::encode(public),
        keypair: base64url::encode(keypair.to_keypair_bytes()),
    }
} 

// `add_keypair` restores a new serde serializable deserialzable keypair from provided arguments.
//  # Arguments
//  * `private_key` - private key of the ParallelChain account 
//  * `public_key` -  public key of the ParallelChain account 
//  * `keypair_name` - name of the keypair saved on the JSON file
//
pub fn add_keypair(private_key: &str, public_key: &str, name: &str) -> Result<KeypairJSON, DisplayMsg> {
    use std::convert::TryFrom;
    let mut sender_public_key = match base64url::decode(&public_key) {
        Ok(addr) => addr,
        Err(e) => {
            return Err(DisplayMsg::FailToDecodeBase64String(String::from("public key"), String::from(public_key), e.to_string()));
        },
    };
    let mut sender_private_key = match base64url::decode(&private_key) {
        Ok(addr) => addr,
        Err(e) => {
            return Err(DisplayMsg::FailToDecodeBase64String(String::from("private key"), String::from(public_key), e.to_string()));
        },
    };

    // Concatenate two keys together
    sender_private_key.append(&mut sender_public_key);
    let keypair = match ed25519_dalek::SigningKey::from_keypair_bytes(&<[u8; 64]>::try_from(&sender_private_key[..]).unwrap()) {
        Ok(k) => k,
        Err(e) => panic!("{}", DisplayMsg::InvalidEd25519Keypair(e.to_string())),
    };
                            

    Ok(KeypairJSON{
        public_key: String::from(public_key),
        private_key: String::from(private_key),
        keypair: base64url::encode(keypair.to_bytes()),
        name: name.to_string()
    })

}

// `append_keypair_to_json` takes a path to keypair JSON and appends a new keypair to the file.
//  # Arguments
//  * `path_to_keypair_json` - path to keypair JSON file
//  * `new_keypair` - new `Keypair` that needs to be appended to the existing list on your keypair JSON
//   
pub fn append_keypair_to_json(path_to_keypair_json: PathBuf, new_keypair: KeypairJSON) -> Result<String, DisplayMsg>{
    let mut keypairs = load_existing_keypairs(path_to_keypair_json.clone())?;
    if keypairs.iter().any(|keypair| keypair.name == new_keypair.name) {
        return Err(DisplayMsg::KeypairAlreadyExists(new_keypair.name))
    } else {
        keypairs.push(new_keypair);
    };
    let updated_keypairs = match serde_json::to_vec(&keypairs){
        Ok(data) => data,
        Err(e) => {
            return Err(DisplayMsg::FailToEncodeJson(String::from("keypair"), path_to_keypair_json, e.to_string()))
        }
    };
    let updated_keypairs_bytes = utils::encrypt(&updated_keypairs)?;

    match utils::write_file(path_to_keypair_json.clone(), &updated_keypairs_bytes) {
        Ok(_) => Ok(String::from("Success")),
        Err(e) => Err(DisplayMsg::FailToWriteFile(String::from("keypair json"), path_to_keypair_json, e))           
    }
}


