/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Methods related to subcommand `crypto` in `pchain-client`.
use crate::command::Keys;
use crate::display_msg::DisplayMsg;
use crate::keypair::{
    add_keypair, append_keypair_to_json, generate_keypair, get_keypair_from_json,
    load_existing_keypairs,
};
use crate::{config, utils};
use ed25519_dalek::Signer;
use std::convert::TryFrom;

// `match_crypto_subcommand` matches a CLI argument to its corresponding `Crypto` subcommand and processes
//  the request.
//  # Arguments
//  * `crypto_subcommand` - crypto subcommand from CLI
//
pub fn match_crypto_subcommand(crypto_subcommand: Keys) {
    match crypto_subcommand {
        Keys::List => {
            match load_existing_keypairs(config::get_keypair_path()) {
                Ok(keypairs) => {
                    let title = "Keypair Name (First 50 char)";
                    let padding_filler = "";
                    println!(
                        "{title} {padding_filler:>len$} Public key ",
                        len = 50 - title.len()
                    );
                    println!("------------------------- {padding_filler:>len$} ------------------------- ", len = 25);

                    for kp in keypairs {
                        let padding_len = 50u32.saturating_sub(kp.name.len() as u32) as usize;
                        println!(
                            "{} {padding_filler:>padding_len$} {}",
                            &kp.name[..std::cmp::min(50, kp.name.len())],
                            kp.public_key
                        );
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        Keys::Create { name } => {
            let name = name.unwrap_or_else(utils::get_random_string);
            let keypair = generate_keypair(&name);
            let public_key = keypair.public_key.clone();

            match append_keypair_to_json(config::get_keypair_path(), keypair) {
                Ok(_) => println!("{}", DisplayMsg::SuccessCreateKey(name, public_key)),
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        Keys::Import {
            private_key,
            public_key,
            name,
        } => {
            let keypair = match add_keypair(&private_key, &public_key, &name) {
                Ok(kp) => kp,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
            if let Err(e) = append_keypair_to_json(config::get_keypair_path(), keypair) {
                println!("{}", e);
                std::process::exit(1);
            }

            println!("{}", DisplayMsg::SuccessAddKey(name));
        }
        Keys::Sign { message, name } => {
            let keypair = match get_keypair_from_json(config::get_keypair_path(), &name) {
                Ok(Some(kp)) => {
                    let keypair_bs = match base64url::decode(&kp.keypair) {
                        Ok(kp) => kp,
                        Err(e) => {
                            println!(
                                "{}",
                                DisplayMsg::FailToDecodeBase64String(
                                    String::from("keypair name"),
                                    name,
                                    e.to_string()
                                )
                            );
                            std::process::exit(1);
                        }
                    };
    
                    match ed25519_dalek::SigningKey::from_keypair_bytes(&<[u8; 64]>::try_from(&keypair_bs[..]).unwrap()) {
                        Ok(kp) => kp,
                        Err(e) => {
                            println!("{}", DisplayMsg::InvalidEd25519Keypair(e.to_string()));
                            std::process::exit(1);
                        }
                    }
                }
                Ok(None) => {
                    println!("{}", DisplayMsg::KeypairNotFound(name));
                    std::process::exit(1);
                }
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

            let encoded_ciphertext = match base64url::decode(&message) {
                Ok(serialized_credentials) => {
                    let ciphertext = keypair.sign(&serialized_credentials[..]).to_bytes();
                    base64url::encode(ciphertext)
                }
                Err(e) => {
                    println!("{}", DisplayMsg::FailToSignMessage(e.to_string()));
                    std::process::exit(1);
                }
            };

            println!("Message: {}", message);
            println!("Ciphertext: {}", encoded_ciphertext);
        }
        Keys::Export { name, destination } => {
            let keypair = match get_keypair_from_json(config::get_keypair_path(), &name) {
                Ok(Some(kp)) => kp,
                Ok(None) => {
                    println!("{}", DisplayMsg::KeypairNotFound(name));
                    std::process::exit(1);
                }
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

            let path = std::path::PathBuf::from(destination.unwrap_or(format!("{}.json", name)));
            match utils::write_file(
                path.clone(),
                serde_json::to_string_pretty(&keypair).unwrap().as_bytes(),
            ) {
                Ok(path) => println!("Keypair is saved at {}", path),
                Err(e) => {
                    println!(
                        "{}",
                        DisplayMsg::FailToWriteFile(String::from("Export keypair"), path, e)
                    );
                    std::process::exit(1);
                }
            }
        }
    };
}
