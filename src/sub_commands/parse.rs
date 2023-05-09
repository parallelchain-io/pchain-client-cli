/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Methods related to subcommand `setup` in `pchain-client`.

use crate::{command::Parse, display_msg::DisplayMsg};
use pchain_client_rs::{
    base64url_to_bytes32, compute_contract_address, call_result_to_data_type
};

// `match_parse_subcommand` matches a CLI argument to its corresponding `Parse` subcommand and processes 
//  the request.
//  # Arguments
//  * `parse_subcommand` - parse subcommand from CLI
//  
pub fn match_parse_subcommand(parse_subcommand: Parse) {
    match parse_subcommand {
        Parse::Base64Encoding { encode, decode, value } => {
            // if one and only one of the encode / decode argument is true
            if encode ^ decode {
                if encode {
                    match serde_json::from_str::<Vec<u8>>(&value){
                        Ok(d) =>  println!("{}", *pchain_types::Base64URL::encode(d)),
                        Err(_) => {                    
                            println!("{}", DisplayMsg::IncorrectFormatForSuppliedArgument(String::from("vector")));
                        }
                    };
                }

                if decode {
                    match  pchain_types::Base64URL::decode(&value) {
                        Ok(d) => println!("{:?}", d),
                        Err(e) => println!("{}", DisplayMsg::FailToDecodeBase64String(String::from("provided string"), value, e.to_string()))
                    };
                }

            } else {
                println!("{}", DisplayMsg::IncorrectCombinationOfIdentifiers(String::from("encode, decode")));
            }            
        },

        Parse::CallResult { value, data_type } => {
            let value = pchain_types::Base64URL::decode(&value)
                    .expect(&DisplayMsg::FailToDecodeBase64String(String::from("call return result"), value, String::new()).to_string());

            match call_result_to_data_type(&value, data_type) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    println!("{}", DisplayMsg::FailToParseCallResult(e));
                }
            }
        },
        Parse::ContractAddress { address, nonce } => {
            match base64url_to_bytes32(&address) {
                Ok(sender_address) => println!("Contract Address: {}", pchain_types::Base64URL::encode(&compute_contract_address(&sender_address, nonce)).to_string()),
                Err(_) => {
                    println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("address"), address, String::new()));
                }
            };
        }
    };
    std::process::exit(1);

}