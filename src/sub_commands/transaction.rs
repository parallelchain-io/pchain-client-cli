/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Methods related to subcommand `submit` in `pchain-client`.

use pchain_client_rs::{Client,base64url_to_bytes32};
use std::path::PathBuf;

use crate::display_msg::DisplayMsg;
use crate::command::{Transaction, CreateTx, DepositTx, StakeTx, PoolTx};
use crate::display_types::{CallArgument, SubmitTx, TxCommand, check_contract_exist};
use crate::result::{ClientResponse, display_beautified_rpc_result};
use crate::config::Config;
use crate::utils::read_file_to_utf8string;

// `match_submit_subcommand` matches a CLI argument to its corresponding `Submit` subcommand and processes 
//  the request.
//  # Arguments
//  * `tx_subcommand` - subcommand for submitting a transaction from CLI
//  * `config` - networking config for client
//  
pub async fn match_submit_subcommand(tx_subcommand: Transaction, config: Config) {
    let url = config.get_url();
    let pchain_client = Client::new(url);

    match tx_subcommand {
        Transaction::Submit {file, keypair_name} => {   
            let submit_tx = match SubmitTx::from_json_file(&file) {
                Ok(tx_json) => tx_json,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                },
            };
            

            let signed_tx = match submit_tx.prepare_signed_tx(&keypair_name) {
                Ok(tx) => tx,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                },
            };
        
            let response = pchain_client
                .submit_transaction(&signed_tx)
                .await;
            
            display_beautified_rpc_result(ClientResponse::SubmitTx(response, signed_tx))
        },
        Transaction::Create{
            destination,
            priority_fee_per_gas, 
            gas_limit, 
            max_base_fee_per_gas, 
            nonce, 
            create_tx_subcommand
        } => {
            let command = subcommand_parser(create_tx_subcommand);

            let tx = SubmitTx{
                commands: vec![command],
                nonce,
                gas_limit,
                max_base_fee_per_gas,
                priority_fee_per_gas,
            };
            
            match tx.to_json_file(&destination.unwrap_or("tx.json".to_string())){
                Ok(path) => println!("{}", DisplayMsg::SuccessCreateFile(String::from("Transaction"), PathBuf::from(path))),
                Err(e) => println!("{}", e)
            }
        },
        Transaction::Append { 
            file, 
            create_tx_subcommand 
        } => {
            let mut submit_tx = match SubmitTx::from_json_file(&file) {
                Ok(tx_json) => tx_json,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                },
            };

            let command = subcommand_parser(create_tx_subcommand);
            submit_tx.commands.push(command);

            match submit_tx.to_json_file(&file){
                Ok(path) => println!("{}", DisplayMsg::SuccessUpdateFile(String::from("Transaction"), PathBuf::from(path))),
                Err(e) => println!("{}", e)
            }
        }
    };      
}

fn subcommand_parser(tx_subcommand: CreateTx) -> TxCommand{
    match tx_subcommand {
        CreateTx::Transfer { recipient: target_address, amount } => {
            if let Err(e) = base64url_to_bytes32(&target_address) {
                println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("target"), target_address, e));
                std::process::exit(1);
            };
            TxCommand::Transfer{recipient: target_address, amount}
        },
        CreateTx::Deploy { contract_code, cbi_version } => {
            let contract_path = match check_contract_exist(&contract_code) {
                Ok(path) => path,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
            TxCommand::Deploy{contract: contract_path, cbi_version}
        },
        CreateTx::Call { target: target_address, method, arguments, amount } => {
            if let Err(e) = base64url_to_bytes32(&target_address) {
                println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("target"), target_address, e));
                std::process::exit(1);
            };

            let args = match arguments {
                Some(path) => {
                    let path_to_json = PathBuf::from(&path);
                    let arguments_json = match read_file_to_utf8string(path_to_json.clone()) {
                        Ok(result) => result,
                        Err(e) => {
                            println!("{}", DisplayMsg::FailToOpenOrReadFile(String::from("call argment json"), path_to_json, e));
                            std::process::exit(1);
                        }
                    };
                    
                    let call = match pchain_client_rs::CallArguments::from_json(&arguments_json) {
                        Ok(result) => result,
                        Err(e) => {
                            println!("{}", DisplayMsg::FailToDecodeJson(String::from("call argument"), path_to_json, e));
                            std::process::exit(1);
                        }
                    };

                    if call.arguments.len() == 0 { 
                        None 
                    } else { 
                        let mut call_arguments = Vec::new();
                        for argument in call.arguments {
                                call_arguments.push(
                                    CallArgument{ argument_type: argument.0, argument_value: argument.1  }
                                );
                        }

                        Some(call_arguments)
                    }
                },
                None => None
            };

            TxCommand::Call {
                target: target_address, 
                method, 
                arguments: args,   
                amount
            }
        },
        CreateTx::Deposit { deposit_tx_subcommand } => {
            match deposit_tx_subcommand {
                DepositTx::Create { operator, balance, auto_stake_rewards } => {
                    if let Err(e) = base64url_to_bytes32(&operator) {
                        println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e));
                        std::process::exit(1);
                    };
                    TxCommand::CreateDeposit{operator, balance, auto_stake_rewards}                       
                },
                DepositTx::UpdateSettings { operator, auto_stake_rewards } => {
                    if let Err(e) = base64url_to_bytes32(&operator) {
                        println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e));
                        std::process::exit(1);
                    };
                    TxCommand::SetDepositSettings{operator, auto_stake_rewards}
                },
                DepositTx::TopUp { operator, amount } => {
                    if let Err(e) = base64url_to_bytes32(&operator) {
                        println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e));
                        std::process::exit(1);
                    };
                    TxCommand::TopUpDeposit{operator, amount}                    
                },
                DepositTx::Withdraw { operator, max_amount } => {
                    if let Err(e) = base64url_to_bytes32(&operator) {
                        println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e));
                        std::process::exit(1);
                    };
                    TxCommand::WithdrawDeposit{operator, max_amount}                      
                },
            }
        },
        CreateTx::Stake { stake_tx_subcommand } => {
            match stake_tx_subcommand {
                StakeTx::Stake { operator, max_amount } => {
                    if let Err(e) = base64url_to_bytes32(&operator) {
                        println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e));
                        std::process::exit(1);
                    };
                    TxCommand::StakeDeposit{operator, max_amount}
                },
                StakeTx::Unstake { operator, max_amount } => {
                    if let Err(e) = base64url_to_bytes32(&operator) {
                        println!("{}", DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e));
                        std::process::exit(1);
                    };
                    TxCommand::UnstakeDeposit{operator, max_amount}                     
                },
            }
        },
        CreateTx::Pool { pool_tx_subcommand } => {
            match pool_tx_subcommand {
                PoolTx::Create { commission_rate } => {
                    TxCommand::CreatePool{commission_rate}
                },
                PoolTx::UpdateSettings { commission_rate } => {
                    TxCommand::SetPoolSettings{commission_rate}
                },
                PoolTx::Delete => {
                    TxCommand::DeletePool
                },
            }
        },
    }
}
