/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Methods related to subcommand `submit` in `pchain-client`.

use pchain_client::ClientV2;
use serde_json::Value;
use std::path::PathBuf;

use crate::command::{CreateTx, DepositTx, PoolTx, StakeTx, Transaction};
use crate::config::Config;
use crate::display_msg::DisplayMsg;
use crate::display_types::{check_contract_exist, SubmitTx, TxCommand};
use crate::parser::{
    base64url_to_public_address, call_arguments_from_json_array, parse_json_arguments,
};
use crate::result::{display_beautified_rpc_result, ClientResponse};
use crate::utils::read_file_to_utf8string;

// `match_submit_subcommand` matches a CLI argument to its corresponding `Submit` subcommand and processes
//  the request.
//  # Arguments
//  * `tx_subcommand` - subcommand for submitting a transaction from CLI
//  * `config` - networking config for client
//
pub async fn match_submit_subcommand(tx_subcommand: Transaction, config: Config) {
    let url = config.get_url();
    let pchain_client_v2 = ClientV2::new(url);

    match tx_subcommand {
        Transaction::Submit { file, keypair_name } => {
            let submit_tx = match SubmitTx::from_json_file(&file) {
                Ok(tx_json) => tx_json,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
            println!("submit tx: {:?}\n", submit_tx);

            let signed_tx = match submit_tx.prepare_signed_tx(&keypair_name) {
                Ok(tx) => tx,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                },
            };
            println!("signed tx: {:?}", signed_tx);
        
            let response = pchain_client_v2
                .submit_transaction(&signed_tx)
                .await;
            
            display_beautified_rpc_result(ClientResponse::SubmitTx(response, signed_tx))
        }
        Transaction::Create {
            destination,
            v1,
            v2,
            priority_fee_per_gas, 
            gas_limit, 
            max_base_fee_per_gas, 
            nonce, 
            create_tx_subcommand
        } => {
            let command = subcommand_parser(create_tx_subcommand);

            let tx = SubmitTx{
                v1,
                v2,
                commands: vec![command],
                nonce,
                gas_limit,
                max_base_fee_per_gas,
                priority_fee_per_gas,
            };

            match tx.to_json_file(&destination.unwrap_or_else(|| "tx.json".to_string())) {
                Ok(path) => println!(
                    "{}",
                    DisplayMsg::SuccessCreateFile(String::from("Transaction"), PathBuf::from(path))
                ),
                Err(e) => println!("{}", e),
            }
        }
        Transaction::Append {
            file,
            create_tx_subcommand,
        } => {
            let mut submit_tx = match SubmitTx::from_json_file(&file) {
                Ok(tx_json) => tx_json,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

            let command = subcommand_parser(create_tx_subcommand);
            submit_tx.commands.push(command);

            match submit_tx.to_json_file(&file) {
                Ok(path) => println!(
                    "{}",
                    DisplayMsg::SuccessUpdateFile(String::from("Transaction"), PathBuf::from(path))
                ),
                Err(e) => println!("{}", e),
            }
        }
    };
}

fn subcommand_parser(tx_subcommand: CreateTx) -> TxCommand {
    match tx_subcommand {
        CreateTx::Transfer {
            recipient: target_address,
            amount,
        } => {
            if let Err(e) = base64url_to_public_address(&target_address) {
                println!(
                    "{}",
                    DisplayMsg::FailToDecodeBase64Address(
                        String::from("target"),
                        target_address,
                        e.to_string()
                    )
                );
                std::process::exit(1);
            };
            TxCommand::Transfer {
                recipient: target_address,
                amount,
            }
        }
        CreateTx::Deploy {
            contract_code,
            cbi_version,
        } => {
            let contract_path = match check_contract_exist(&contract_code) {
                Ok(path) => path,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
            TxCommand::Deploy {
                contract: contract_path,
                cbi_version,
            }
        }
        CreateTx::Call {
            target: target_address,
            method,
            arguments,
            amount,
        } => {
            if let Err(e) = base64url_to_public_address(&target_address) {
                println!(
                    "{}",
                    DisplayMsg::FailToDecodeBase64Address(
                        String::from("target"),
                        target_address,
                        e.to_string()
                    )
                );
                std::process::exit(1);
            };

            let arguments = match arguments {
                Some(path) => {
                    let path_to_json = PathBuf::from(&path);
                    let arguments_json = match read_file_to_utf8string(path_to_json.clone()) {
                        Ok(result) => result,
                        Err(e) => {
                            println!(
                                "{}",
                                DisplayMsg::FailToOpenOrReadFile(
                                    String::from("call argment json"),
                                    path_to_json,
                                    e
                                )
                            );
                            std::process::exit(1);
                        }
                    };

                    let arguments: Value = match serde_json::from_str(&arguments_json) {
                        Ok(json_val) => json_val,
                        Err(e) => {
                            println!("{}", DisplayMsg::InvalidJson(e));
                            std::process::exit(1);
                        }
                    };

                    let arguments = match parse_json_arguments(&arguments).and_then(|json_arr| {
                        // Check if it can be parsed into call arguments
                        let call_arguments = call_arguments_from_json_array(&json_arr)?;
                        if call_arguments.is_empty() {
                            Ok(Vec::new())
                        } else {
                            Ok(json_arr)
                        }
                    }) {
                        Ok(values) => values,
                        Err(e) => {
                            println!(
                                "{}",
                                DisplayMsg::FailToDecodeJson(
                                    String::from("call argument"),
                                    path_to_json,
                                    e.to_string()
                                )
                            );
                            std::process::exit(1);
                        }
                    };

                    (!arguments.is_empty()).then_some(arguments)
                }
                None => None,
            };

            TxCommand::Call {
                target: target_address,
                method,
                arguments,
                amount,
            }
        }
        CreateTx::Deposit {
            deposit_tx_subcommand,
        } => match deposit_tx_subcommand {
            DepositTx::Create {
                operator,
                balance,
                auto_stake_rewards,
            } => {
                if let Err(e) = base64url_to_public_address(&operator) {
                    println!(
                        "{}",
                        DisplayMsg::FailToDecodeBase64Address(
                            String::from("operator"),
                            operator,
                            e.to_string()
                        )
                    );
                    std::process::exit(1);
                };
                TxCommand::CreateDeposit {
                    operator,
                    balance,
                    auto_stake_rewards,
                }
            }
            DepositTx::UpdateSettings {
                operator,
                auto_stake_rewards,
            } => {
                if let Err(e) = base64url_to_public_address(&operator) {
                    println!(
                        "{}",
                        DisplayMsg::FailToDecodeBase64Address(
                            String::from("operator"),
                            operator,
                            e.to_string()
                        )
                    );
                    std::process::exit(1);
                };
                TxCommand::SetDepositSettings {
                    operator,
                    auto_stake_rewards,
                }
            }
            DepositTx::TopUp { operator, amount } => {
                if let Err(e) = base64url_to_public_address(&operator) {
                    println!(
                        "{}",
                        DisplayMsg::FailToDecodeBase64Address(
                            String::from("operator"),
                            operator,
                            e.to_string()
                        )
                    );
                    std::process::exit(1);
                };
                TxCommand::TopUpDeposit { operator, amount }
            }
            DepositTx::Withdraw {
                operator,
                max_amount,
            } => {
                if let Err(e) = base64url_to_public_address(&operator) {
                    println!(
                        "{}",
                        DisplayMsg::FailToDecodeBase64Address(
                            String::from("operator"),
                            operator,
                            e.to_string()
                        )
                    );
                    std::process::exit(1);
                };
                TxCommand::WithdrawDeposit {
                    operator,
                    max_amount,
                }
            }
        },
        CreateTx::Stake {
            stake_tx_subcommand,
        } => match stake_tx_subcommand {
            StakeTx::Stake {
                operator,
                max_amount,
            } => {
                if let Err(e) = base64url_to_public_address(&operator) {
                    println!(
                        "{}",
                        DisplayMsg::FailToDecodeBase64Address(
                            String::from("operator"),
                            operator,
                            e.to_string()
                        )
                    );
                    std::process::exit(1);
                };
                TxCommand::StakeDeposit {
                    operator,
                    max_amount,
                }
            }
            StakeTx::Unstake {
                operator,
                max_amount,
            } => {
                if let Err(e) = base64url_to_public_address(&operator) {
                    println!(
                        "{}",
                        DisplayMsg::FailToDecodeBase64Address(
                            String::from("operator"),
                            operator,
                            e.to_string()
                        )
                    );
                    std::process::exit(1);
                };
                TxCommand::UnstakeDeposit {
                    operator,
                    max_amount,
                }
            }
        },
        CreateTx::Pool { pool_tx_subcommand } => match pool_tx_subcommand {
            PoolTx::Create { commission_rate } => TxCommand::CreatePool { commission_rate },
            PoolTx::UpdateSettings { commission_rate } => {
                TxCommand::SetPoolSettings { commission_rate }
            }
            PoolTx::Delete => TxCommand::DeletePool,
        },
    }
}
