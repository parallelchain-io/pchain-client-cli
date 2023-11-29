/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Definition of methods related to processing results and displaying them in beauified format.

use crate::command::Base64String;
use crate::display_msg::DisplayMsg;
use crate::display_types::{
    Block, BlockHeader, CommandReceipt, Deposit, Pool, Receipt, Stake, Transaction,
    TransactionWithReceipt, ValidatorSet, CallReceipt, Receipt2,
};
use crate::utils::write_file;
use pchain_types::blockchain::CommandReceiptV2;
use pchain_types::rpc::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// `display_beautified_rpc_result` translates the return result from Fullnode RPC/Chain Scanner
///  endpoints to beautified readable content.
/// # Arguments
///  * `response` - `ClientResponse` from the corresponding Fullnode/Chain Scanner provider
///
pub fn display_beautified_rpc_result(response: ClientResponse) {
    match response {
        ClientResponse::SubmitTx(result, signed_tx) => {
            println!("result:{:?}", result);
            match result {
                Ok(res) => {
                    match res.error {
                        Some(error) => {
                            println!("{}", DisplayMsg::FailSubmitTx(error));
                            std::process::exit(1);
                        }
                        None => {
                            let mut tx = Vec::new();

                            // if transaction contains `Deploy` command, print the contract address to console
                            match signed_tx {
                                TransactionV1OrV2::V1(txn) => {
                                    if txn.commands.iter().any(|command| matches!(command, pchain_types::blockchain::Command::Deploy(_))
                                    ) {
                                        let contract_address = base64url::encode(
                                            pchain_types::cryptography::contract_address_v1(&txn.signer, txn.nonce)
                                        );
                                        tx.push(("Contract Address: ", serde_json::to_value(contract_address).unwrap()));
                                    }

                                    let tx_print: Transaction = From::<pchain_types::blockchain::TransactionV1>::from(txn);
                                    tx.push(("Response: ", serde_json::to_value(DisplayMsg::SuccessSubmitTx.to_string()).unwrap()));
                                    tx.push(("Command(s): ", serde_json::Value::Array(tx_print.commands)));
                                    tx.push(("Transaction Hash: ", serde_json::to_value(tx_print.hash).unwrap()));
                                    tx.push(("Signature: ", serde_json::to_value(tx_print.signature).unwrap()));
                                    display_beautified_json(tx);
                                },
                                TransactionV1OrV2::V2(txn) => {

                                    for (index, command) in txn.commands.iter().enumerate() {
                                        if let pchain_types::blockchain::Command::Deploy(_) = command {
                                            let contract_address = base64url::encode(
                                                pchain_types::cryptography::contract_address_v2(&txn.signer, txn.nonce, index as u32)
                                            );
                                            tx.push(("Contract Address: ", serde_json::to_value(contract_address).unwrap()));
                                        }
                                    }
                                        
                                    let tx_print: Transaction = From::<pchain_types::blockchain::TransactionV2>::from(txn);
                                    tx.push(("Response: ", serde_json::to_value(DisplayMsg::SuccessSubmitTx.to_string()).unwrap()));
                                    tx.push(("Command(s): ", serde_json::Value::Array(tx_print.commands)));
                                    tx.push(("Transaction Hash: ", serde_json::to_value(tx_print.hash).unwrap()));
                                    tx.push(("Signature: ", serde_json::to_value(tx_print.signature).unwrap()));
                                    display_beautified_json(tx);

                                },
                            };
                        }
                    }
                }
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                },
            }
        }
        ClientResponse::Block(result) => {

            match result {
                Ok(BlockResponseV2{ block: Some(block)}) => {
                    match block {
                        BlockV1ToV2::V1(block) => {
                            let block_print: Block = From::<pchain_types::blockchain::BlockV1>::from(block);
                            println!("{:#}", serde_json::to_value(block_print).unwrap())
                        },
                        BlockV1ToV2::V2(block) => {
                            let block_print: Block = From::<pchain_types::blockchain::BlockV2>::from(block);
                            println!("{:#}", serde_json::to_value(block_print).unwrap())
                        },
                    }
                },
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                },
                _ => {
                    println!("{}", DisplayMsg::CannotFindRelevantBlock);
                    std::process::exit(1);
                }
            }
        }
        ClientResponse::BlockHeader(result) => {

            match result {
                Ok(BlockHeaderResponseV2 { block_header: Some(bh) }) => {
                    match bh {
                        BlockHeaderV1ToV2::V1(bh) => {
                            let header_print: BlockHeader = 
                                From::<pchain_types::blockchain::BlockHeaderV1>::from(bh);
                            
                            println!("{:#}", serde_json::to_value(header_print).unwrap())
                        },
                        BlockHeaderV1ToV2::V2(bh) => {
                            let header_print: BlockHeader =
                                From::<pchain_types::blockchain::BlockHeaderV2>::from(bh);
                            println!("{:#}", serde_json::to_value(header_print).unwrap())
                        },
                    }
                },
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);  
                },
                _ => {
                    println!("{}", DisplayMsg::CannotFindRelevantBlock);
                    std::process::exit(1);
                }
            }
        }
        ClientResponse::Transaction(result) => {

            match result {
                Ok(TransactionResponseV2{transaction: Some(transaction), receipt, block_hash: _, position: _}) => {

                    match transaction {
                        TransactionV1ToV2::V1(txn) => {
                            match receipt {
                                Some(ReceiptV1ToV2::V1(receipt)) => {
                                    let tx_print: TransactionWithReceipt = From::<(pchain_types::blockchain::TransactionV1, pchain_types::blockchain::ReceiptV1)>::from((txn, receipt));
                                    println!("{:#}", serde_json::to_value(tx_print).unwrap())     
                                }, 
                                None => {
                                    let tx_print: Transaction = From::<pchain_types::blockchain::TransactionV1>::from(txn);
                                    println!("{:#}", serde_json::to_value(tx_print).unwrap())
                                },
                                _ => todo!()
                            }
                        },
                        TransactionV1ToV2::V2(txn) => {
                            match receipt {
                                Some(ReceiptV1ToV2::V2(receipt)) => {
                                    let tx_print: TransactionWithReceipt = From::<(pchain_types::blockchain::TransactionV2, pchain_types::blockchain::ReceiptV2)>::from((txn, receipt));
                                    println!("{:#}", serde_json::to_value(tx_print).unwrap())
                                },
                                None => {
                                    let tx_print: Transaction = From::<pchain_types::blockchain::TransactionV2>::from(txn);
                                    println!("{:#}", serde_json::to_value(tx_print).unwrap())
                                },
                                _ => todo!(),
                            }
                        },
                    }
                },
                Err(e) => { 
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                },
                _ => {
                    println!("{}", DisplayMsg::CannotFindRelevantTransaction);
                    std::process::exit(1);
                },
            }

        },
        ClientResponse::Receipt(result) => {
            let receipt: pchain_types::rpc::ReceiptV1ToV2 = match result {
                Ok(ReceiptResponseV2 {
                    transaction_hash: _,
                    receipt: Some(receipt),
                    block_hash: _,
                    position: _,
                }) => receipt,
                Err(_) => {
                    std::process::exit(1);
                }
                _ => {
                    println!("{}", DisplayMsg::CannotFindRelevantReceipt);
                    std::process::exit(1);
                }
            };

            match receipt {
                ReceiptV1ToV2::V1(r) => {
                    let receipt_print: Receipt = r.into_iter().map(CommandReceipt::from).collect();
                    println!("{:#}", serde_json::to_value(receipt_print).unwrap())
                },
                ReceiptV1ToV2::V2(r) => {
                    let receipt_print: Vec<Receipt2> = r.command_receipts.into_iter().map(|command_receipt| {
                        From::<CommandReceiptV2>::from(command_receipt)
                    }).collect();
                    println!("{:#}", serde_json::to_value(receipt_print).unwrap())
                },
            }

        }
        ClientResponse::Contract(result, destination) => match result {
            Ok(StateResponse {
                accounts,
                storage_tuples: _,
                block_hash: _,
            }) => {
                let account = accounts.into_values().next().unwrap();

                if let Account::WithContract(AccountWithContract {
                    nonce: _,
                    balance: _,
                    ref contract,
                    cbi_version: _,
                    storage_hash: _,
                }) = account
                {
                    if let Some(code) = contract {
                        let path =
                            PathBuf::from(&destination.unwrap_or_else(|| "code.wasm".to_string()));
                        match write_file(path.clone(), code) {
                            Ok(full_path) => println!(
                                "{}",
                                DisplayMsg::SuccessCreateFile(
                                    String::from("contract"),
                                    PathBuf::from(full_path)
                                )
                            ),
                            Err(e) => println!(
                                "{}",
                                DisplayMsg::FailToWriteFile(String::from("contract"), path, e)
                            ),
                        }
                    } else {
                        println!("{}", DisplayMsg::CannotFindRelevantContractCode);
                    }
                };
            }
            Err(e) => {
                println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                std::process::exit(1);
            }
        },
        ClientResponse::State(result) => {
            let state = match result {
                Ok(StateResponse {
                    accounts: _,
                    storage_tuples,
                    block_hash: _,
                }) => {
                    if let Some(state_key_value_pairs) = storage_tuples.into_values().next() {
                        state_key_value_pairs.into_values().next()
                    } else {
                        unreachable!()
                    }
                }
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
            };

            let stringify_state: Base64String = state.map_or(String::new(), base64url::encode);
            println!("{:#}", serde_json::to_value(stringify_state).unwrap())
        }
        ClientResponse::Balance(result) => {
            let balance = match result {
                Ok(StateResponse {
                    accounts,
                    storage_tuples: _,
                    block_hash: _,
                }) => {
                    let account_state_to_value_pairs = accounts.into_values().next();
                    if let Some(Account::WithoutContract(AccountWithoutContract {
                        nonce: _,
                        balance,
                        cbi_version: _,
                        storage_hash: _,
                    })) = account_state_to_value_pairs
                    {
                        balance
                    } else {
                        unreachable!()
                    }
                }
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
            };

            println!("{:#}", serde_json::to_value(balance).unwrap())
        }
        ClientResponse::Nonce(result) => {
            let nonce = match result {
                Ok(StateResponse {
                    accounts,
                    storage_tuples: _,
                    block_hash: _,
                }) => {
                    let account_state_to_value_pairs = accounts.into_values().next();
                    if let Some(Account::WithoutContract(AccountWithoutContract {
                        nonce,
                        balance: _,
                        cbi_version: _,
                        storage_hash: _,
                    })) = account_state_to_value_pairs
                    {
                        nonce
                    } else {
                        unreachable!()
                    }
                }
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
            };

            println!("{:#}", serde_json::to_value(nonce).unwrap())
        }
        ClientResponse::PreviousValidatorSet(result)
        | ClientResponse::CurrentValidatorSet(result)
        | ClientResponse::NextValidatorSet(result) => {
            let validator_set: Option<pchain_types::rpc::ValidatorSet> = match result {
                Ok(ValidatorSetsResponse {
                    previous_validator_set: Some(None),
                    current_validator_set: None,
                    next_validator_set: None,
                    block_hash: _,
                }) => None,
                Ok(ValidatorSetsResponse {
                    previous_validator_set: Some(Some(vs)),
                    current_validator_set: None,
                    next_validator_set: None,
                    block_hash: _,
                })
                | Ok(ValidatorSetsResponse {
                    previous_validator_set: None,
                    current_validator_set: Some(vs),
                    next_validator_set: None,
                    block_hash: _,
                })
                | Ok(ValidatorSetsResponse {
                    previous_validator_set: None,
                    current_validator_set: None,
                    next_validator_set: Some(vs),
                    block_hash: _,
                }) => Some(vs),
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
                _ => unreachable!(),
            };

            if let Some(vs) = validator_set {
                let vs_print: ValidatorSet = From::<pchain_types::rpc::ValidatorSet>::from(vs);
                println!("{:#}", serde_json::to_value(vs_print).unwrap())
            } else {
                println!("{}", DisplayMsg::CannotFindValidatorSet);
                std::process::exit(1);
            }
        }
        ClientResponse::StakePower(result) => {
            let stake = match result {
                Ok(StakesResponse {
                    stakes,
                    block_hash: _,
                }) => {
                    if let Some(stake) = stakes.into_values().next() {
                        stake
                    } else {
                        unreachable!()
                    }
                }
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
            };

            if let Some(s) = stake {
                let stake_print: Stake = From::<pchain_types::rpc::Stake>::from(s);
                println!("{:#}", serde_json::to_value(stake_print).unwrap())
            } else {
                println!("{}", DisplayMsg::CannotFindOperatorOwnerPair);
                std::process::exit(1);
            }
        }
        ClientResponse::Pool(result) => {
            let pool = match result {
                Ok(PoolsResponse {
                    pools,
                    block_hash: _,
                }) => {
                    if let Some(p) = pools.into_values().next() {
                        p
                    } else {
                        unreachable!()
                    }
                }
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
            };

            if let Some(p) = pool {
                let pool_print: Pool = From::<pchain_types::rpc::Pool>::from(p);
                println!("{:#}", serde_json::to_value(pool_print).unwrap())
            } else {
                println!("{}", DisplayMsg::CannotFindOperatorOwnerPair);
                std::process::exit(1);
            }
        }
        ClientResponse::Deposit(result) => {
            let deposit = match result {
                Ok(DepositsResponse {
                    deposits,
                    block_hash: _,
                }) => {
                    if let Some(d) = deposits.into_values().next() {
                        d
                    } else {
                        unreachable!()
                    }
                }
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
            };

            if let Some(d) = deposit {
                let deposit_print: Deposit = From::<pchain_types::rpc::Deposit>::from(d);
                println!("{:#}", serde_json::to_value(deposit_print).unwrap())
            } else {
                println!("{}", DisplayMsg::CannotFindOperatorOwnerPair);
                std::process::exit(1);
            }
        }
        ClientResponse::View(result) => {

            let receipt_print: CallReceipt = match result {
                Ok(ViewResponseV2 { command_receipt }) => {
                    match command_receipt {
                        CommandReceiptV1ToV2::V1(r) => {
                            From::<pchain_types::blockchain::CommandReceiptV1>::from(r)
                        },
                        CommandReceiptV1ToV2::V2(r) => {
                            From::<pchain_types::blockchain::CommandReceiptV2>::from(r)
                        },
                    }
                },
                Err(e) => {
                    println!("{}", DisplayMsg::RespnoseWithHTTPError(e));
                    std::process::exit(1);
                }
            };
            println!("{:#}", serde_json::to_value(receipt_print).unwrap())
        }
    }
}

// `display_beautified_json` converts the response of a CLI command
//  to a human readble prettified JSON serde-deserializable string
// # Arguments
// * `response` - A serde serializable/deserializable response from diaplay_types
//
pub fn display_beautified_json(response: Vec<(&str, Value)>) {
    let mut response_map = BTreeMap::new();
    for field in response {
        response_map.insert(field.0.to_string(), field.1);
    }
    let beautified_json: Value =
        serde_json::from_str(&serde_json::to_string_pretty(&response_map).unwrap()).unwrap();

    println!("{:#}", beautified_json);
}

// `display_beautified_json_array` converts the response of a CLI command
//  to a human readble prettified JSON serde-deserializable string
// # Arguments
// * `response` - A serde serializable/deserializable response from diaplay_types
//
pub fn display_beautified_json_array(response: Vec<(&str, Value)>) {
    let mut response_array = Vec::new();
    for field in response {
        let mut array_item = BTreeMap::new();
        array_item.insert(field.0.to_string(), field.1);
        response_array.push(array_item);
    }
    let beautified_json: Value =
        serde_json::from_str(&serde_json::to_string_pretty(&response_array).unwrap()).unwrap();

    println!("{:#}", beautified_json);
}

// [ClientResponse] defines types that are used by the result module to process
// different kinds of responses sent by the pchain_client library to the CLI.
pub enum ClientResponse {
    SubmitTx(
        Result<SubmitTransactionResponseV2, ErrorResponse>,
        pchain_types::rpc::TransactionV1OrV2
    ),
    Balance(Result<StateResponse, ErrorResponse>),
    Nonce(Result<StateResponse, ErrorResponse>),
    Contract(Result<StateResponse, ErrorResponse>, Option<Destination>),
    Block(Result<BlockResponseV2, ErrorResponse>),
    BlockHeader(Result<BlockHeaderResponseV2, ErrorResponse>),
    Transaction(Result<TransactionResponseV2, ErrorResponse>),
    Receipt(Result<ReceiptResponseV2, ErrorResponse>),
    State(Result<StateResponse, ErrorResponse>),
    PreviousValidatorSet(Result<ValidatorSetsResponse, ErrorResponse>),
    CurrentValidatorSet(Result<ValidatorSetsResponse, ErrorResponse>),
    NextValidatorSet(Result<ValidatorSetsResponse, ErrorResponse>),
    Pool(Result<PoolsResponse, ErrorResponse>),
    Deposit(Result<DepositsResponse, ErrorResponse>),
    StakePower(Result<StakesResponse, ErrorResponse>),
    View(Result<ViewResponseV2, ErrorResponse>),
}

type ErrorResponse = String;
type Destination = String;
