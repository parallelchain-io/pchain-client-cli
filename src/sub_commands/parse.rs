/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Methods related to subcommand `setup` in `pchain-client`.

use serde_json::Value;

use crate::{
    command::{ContractAddressVersion, Parse},
    display_msg::DisplayMsg,
    parser::{
        base64url_to_public_address, parse_call_result_from_data_type,
        parse_call_result_from_schema,
    },
    utils::read_file_to_utf8string,
};

// `match_parse_subcommand` matches a CLI argument to its corresponding `Parse` subcommand and processes
//  the request.
//  # Arguments
//  * `parse_subcommand` - parse subcommand from CLI
//
pub fn match_parse_subcommand(parse_subcommand: Parse) {
    match parse_subcommand {
        Parse::Base64Encoding {
            encode,
            decode,
            value,
        } => {
            // if one and only one of the encode / decode argument is true
            if encode ^ decode {
                if encode {
                    match serde_json::from_str::<Vec<u8>>(&value) {
                        Ok(d) => println!("{}", base64url::encode(d)),
                        Err(_) => {
                            println!(
                                "{}",
                                DisplayMsg::IncorrectFormatForSuppliedArgument(String::from(
                                    "vector"
                                ))
                            );
                        }
                    };
                }

                if decode {
                    match base64url::decode(&value) {
                        Ok(d) => println!("{:?}", d),
                        Err(e) => println!(
                            "{}",
                            DisplayMsg::FailToDecodeBase64String(
                                String::from("provided string"),
                                value,
                                e.to_string()
                            )
                        ),
                    };
                }
            } else {
                println!(
                    "{}",
                    DisplayMsg::IncorrectCombinationOfIdentifiers(String::from("encode, decode"))
                );
            }
        }

        Parse::CallResult {
            value,
            data_type,
            schema_file,
        } => {
            let value = base64url::decode(&value).unwrap_or_else(|_| {
                panic!(
                    "{}",
                    DisplayMsg::FailToDecodeBase64String(
                        String::from("call return result"),
                        value,
                        String::new()
                    )
                    .to_string()
                )
            });

            if let Some(data_type) = data_type {
                match parse_call_result_from_data_type(&value, data_type) {
                    Ok(result) => println!("{}", result),
                    Err(e) => {
                        println!("{}", DisplayMsg::FailToParseCallResult(e.to_string()));
                    }
                }
            } else if let Some(schema_file) = schema_file {
                let schema = match read_file_to_utf8string(schema_file.clone()) {
                    Ok(result) => result,
                    Err(e) => {
                        println!(
                            "{}",
                            DisplayMsg::FailToOpenOrReadFile(
                                String::from("schema json"),
                                schema_file,
                                e
                            )
                        );
                        std::process::exit(1);
                    }
                };

                let schema: Value = match serde_json::from_str(&schema) {
                    Ok(json_val) => json_val,
                    Err(e) => {
                        println!("{}", DisplayMsg::InvalidJson(e));
                        std::process::exit(1);
                    }
                };

                let result = match parse_call_result_from_schema(&value, &schema) {
                    Ok(result) => result,
                    Err(e) => {
                        println!("{}", DisplayMsg::FailToParseCallResult(e.to_string()));
                        std::process::exit(1);
                    }
                };

                for (name, value) in result {
                    println!("{name}: {value}");
                }
            }
        }
        Parse::ContractAddress { version } => match version {
            ContractAddressVersion::V1 { address, nonce } => {
                match base64url_to_public_address(&address) {
                    Ok(sender_address) => {
                        println!(
                            "Contract Address: {}",
                            base64url::encode(pchain_types::cryptography::contract_address_v1(
                                &sender_address,
                                nonce
                            ))
                        )
                    }
                    Err(_) => {
                        println!(
                            "{}",
                            DisplayMsg::IncorrectCombinationOfIdentifiers(String::from("v1, v2"))
                        );
                        std::process::exit(1);
                    }
                };
            }
            ContractAddressVersion::V2 {
                address,
                nonce,
                index,
            } => {
                match base64url_to_public_address(&address) {
                    Ok(sender_address) => {
                        println!(
                            "Contract Address: {}",
                            base64url::encode(pchain_types::cryptography::contract_address_v2(
                                &sender_address,
                                nonce,
                                index
                            ))
                        )
                    }
                    Err(_) => {
                        println!(
                            "{}",
                            DisplayMsg::IncorrectCombinationOfIdentifiers(String::from("v1, v2"))
                        );
                        std::process::exit(1);
                    }
                };
            }
        },
    };
    std::process::exit(1);
}
