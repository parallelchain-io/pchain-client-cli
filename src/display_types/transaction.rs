/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache Lice&&nse, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Transaction to a format which can be displayed on the terminal.

use dunce;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use pchain_types::{ blockchain::Command, runtime::*};

use crate::display_msg::{DisplayMsg};
use crate::command::Base64String;
use crate::display_types::{TxCommand, Receipt, Event};
use crate::keypair::{get_keypair_from_json};
use crate::config::get_keypair_path;
use crate::utils::{read_file_to_utf8string, read_file};

/// [Transaction] denotes a display_types equivalent of pchain_types::blockchain::Transaction.
#[derive(Serialize, Debug)]
pub struct Transaction {
    pub commands: Vec<Value>,
    pub signer: Base64String,
    pub priority_fee_per_gas: u64,
    pub gas_limit: u64,
    pub max_base_fee_per_gas: u64,
    pub nonce: u64,
    pub hash: Base64String,
    pub signature: Base64String,
}

impl From<pchain_types::blockchain::Transaction> for Transaction { 
    fn from(transaction: pchain_types::blockchain::Transaction) -> Transaction {
        let mut json_values = vec![];
        for command in transaction.commands {
            let v = match command {
                Command::Transfer (TransferInput{ recipient, amount }) => {
                    let tx_print = TxCommand::Transfer { 
                        recipient: base64url::encode(recipient), 
                        amount 
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::Deploy (DeployInput{contract, cbi_version} ) => {
                    let tx_print = TxCommand::Deploy { 
                        contract: format!("<contract in {} bytes>", contract.len()).to_string(), 
                        cbi_version,
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::Call (CallInput{ target, method, arguments, amount })=> {  
                    let tx_print = json!(
                        {
                            "Call": {
                                "target": base64url::encode(target),
                                "method": method,
                                "amount": amount,
                                "arguments":  serde_json::to_string(&arguments).unwrap()
                            }
                        }   
                    );
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::CreatePool (CreatePoolInput{ commission_rate }) => {
                    let tx_print = TxCommand::CreatePool {
                        commission_rate
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::SetPoolSettings (SetPoolSettingsInput{ commission_rate }) => {
                    let tx_print = TxCommand::SetPoolSettings {
                        commission_rate
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::DeletePool => {
                    let tx_print = TxCommand::DeletePool {};
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::CreateDeposit (CreateDepositInput{ operator, balance, auto_stake_rewards }) => {
                    let tx_print = TxCommand::CreateDeposit {
                        operator: base64url::encode(operator),
                        balance,
                        auto_stake_rewards
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::SetDepositSettings (SetDepositSettingsInput{ operator, auto_stake_rewards }) => {
                    let tx_print = TxCommand::SetDepositSettings {
                        operator: base64url::encode(operator),
                        auto_stake_rewards
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::TopUpDeposit (TopUpDepositInput{ operator, amount }) => {
                    let tx_print = TxCommand::TopUpDeposit {
                        operator: base64url::encode(operator),
                        amount
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::WithdrawDeposit (WithdrawDepositInput{ operator, max_amount }) => {
                    let tx_print = TxCommand::WithdrawDeposit {
                        operator: base64url::encode(operator),
                        max_amount
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::StakeDeposit (StakeDepositInput{ operator, max_amount }) => {
                    let tx_print = TxCommand::StakeDeposit {
                        operator: base64url::encode(operator),
                        max_amount
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::UnstakeDeposit (UnstakeDepositInput{ operator, max_amount }) => {
                    let tx_print = TxCommand::UnstakeDeposit {
                        operator: base64url::encode(operator),
                        max_amount
                    };
                    serde_json::to_value(tx_print).unwrap()
                },
                Command::NextEpoch => {
                    let tx_print = TxCommand::NextEpoch {};
                    serde_json::to_value(tx_print).unwrap()
                },
            };
            json_values.push(v);
        }

        Transaction {
            commands: json_values, 
            signer: base64url::encode(transaction.signer),
            priority_fee_per_gas: transaction.priority_fee_per_gas,
            gas_limit: transaction.gas_limit,
            max_base_fee_per_gas: transaction.max_base_fee_per_gas,
            nonce: transaction.nonce,
            hash: base64url::encode(transaction.hash),
            signature: base64url::encode(transaction.signature),
        }                
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubmitTx {
    pub commands: Vec<TxCommand>,
    pub nonce: u64,
    pub gas_limit: u64,
    pub max_base_fee_per_gas: u64,
    pub priority_fee_per_gas: u64,
}

impl SubmitTx {
    // `to_json_file` serializes SubmitTx into json format and write to a file
    pub fn to_json_file(&self, file_path: &str) -> Result<String, DisplayMsg> {
        let path = Path::new(&file_path);
        if path.extension() != Some(OsStr::new("json")) {
            return Err(DisplayMsg::IncorrectFilePath(String::from("transaction json"), path.to_path_buf(), String::from("Path provided should include the file name and file extension. i.e. example.json")))  
        }
        // path parent will always exist if json file is included in path.
        let path_parent = match path.parent(){
            Some(pp) => pp,
            None => return Err(DisplayMsg::IncorrectFilePath(String::from("transaction json"), path.to_path_buf(), String::from("Path provided should include the file name and file extension. i.e. example.json"))),
        };

        if !path_parent.exists() {
            std::fs::create_dir_all(path_parent).expect(&DisplayMsg::FailToCreateDir(String::from("Parallelchain Client Home"), path.to_path_buf(),  String::new()).to_string());
        }

        let file = std::fs::File::create(path).map_err(|e| DisplayMsg::FailToWriteFile(String::from("transaction"), path.to_path_buf(), e.to_string()))?;
        serde_json::to_writer_pretty(file, &self).map_err(|e| DisplayMsg::FailToWriteFile(String::from("transaction"), path.to_path_buf(), e.to_string()))?;

        Ok(dunce::canonicalize(path).unwrap().into_os_string().into_string().ok().unwrap())
    }

    // `from_json_file` accepts a path to the json file and returns a 
    // serde serializable/deserializable struct for processing submission of Transactions 
    // to ParallelChain.
    //  # Arguments
    //  * `path_to_json` - path to keypair JSON file
    //
    pub fn from_json_file(path_to_json: &str) -> Result<Self, DisplayMsg> {
        let path_to_json = Path::new(&path_to_json);

        let tx_json = if path_to_json.is_file(){
            let data = read_file_to_utf8string(path_to_json.to_path_buf()).map_err(|e| DisplayMsg::FailToOpenOrReadFile(String::from("keypair json"), path_to_json.to_path_buf(), e))?;

            match serde_json::from_str::<SubmitTx>(data.as_str()) {
                Ok(json) => json,
                Err(e) => {
                    return Err(DisplayMsg::FailToDecodeJson(String::from("transaction"), path_to_json.to_path_buf(), e.to_string()))
                },
            }
        } else {
            return Err(DisplayMsg::IncorrectFilePath(String::from("transaction"),  path_to_json.to_path_buf(), String::new()))
        };

        Ok(tx_json)
    }

    // `prepare_and_submit_signed_tx` prepapres a pchain_types::blockchain::Transaction data structure and submits it to ParallelChain.
    //  # Arguments
    //  * `commands` - vector of transaction commands 
    //  * `nonce` - committed nonce of the owner account
    //  * `gas_limit` - maximum number of Gas units that you are willing to consume on executing 
    //                  this Transaction. If this is set to low, your Transaction may not execute to completion
    //  * `priority_fee_per_gas` - XPLL/TXPLL to tip to the proposing Validator
    //  * `max_base_fee_per_gas` - XPLL/TXPLL you are willing to pay per unit Gas consumed in the execution of your 
    //                             transaction (in Grays). This needs to be greater than your Account balance for your transaction to be included in a block
    //  * `keypair_name` - Name of the keypair
    pub fn prepare_signed_tx(
        self,
        keypair_name: &str, 
    ) -> Result<pchain_types::blockchain::Transaction, DisplayMsg> {
        let keypair_json_of_given_user = match get_keypair_from_json(get_keypair_path(), keypair_name){
            Ok(Some(s)) => s,
            Ok(None) => {
                return Err(DisplayMsg::KeypairNotFound(String::from(keypair_name)))
            },
            Err(e) => {
                return Err(e);
            },          
        };
        let keypair_bs = match base64url::decode(&keypair_json_of_given_user.keypair){
            Ok(kp) => kp,
            Err(e) => {
                return Err(DisplayMsg::FailToDecodeBase64String(String::from("keypair"), keypair_json_of_given_user.keypair, e.to_string()));
            }
        };
        let keypair = match ed25519_dalek::Keypair::from_bytes(&keypair_bs) {
            Ok(kp) => kp,
            Err(e) => {
                println!("{}", DisplayMsg::InvalidEd25519Keypair(e.to_string()));
                std::process::exit(1);
            }
        };
        
        let mut commands = vec![];
        for c in self.commands{
            match Command::try_from(c){
                Ok(command) => commands.push(command),
                Err(e) => return Err(DisplayMsg::InvalidTxCommand(e))
            }
        }

        let transaction = pchain_types::blockchain::Transaction::new(
            &keypair,
            self.nonce,
            commands,
            self.gas_limit,
            self.max_base_fee_per_gas, 
            self.priority_fee_per_gas,
        );

        Ok(transaction)
    }
}



// `check_contract_exist` returns contract codeas a vector of bytes.
//  # Arguments
//  * `path` - relative or absolute path to .wasm file
//  # Return 
//  Ok result with canonicalized file path to .wasm file
//  Err if contract does not exist
pub fn check_contract_exist(path: &str) -> Result<String, DisplayMsg> {
    if path.ends_with(".wasm") {
        match dunce::canonicalize(path) {
            Ok(canonicalized_path) => Ok(canonicalized_path.into_os_string().into_string().expect(
                &DisplayMsg::IncorrectFilePath(String::from("contract"), PathBuf::from(path), String::from("The path contains invalid unicode data")).to_string()
            )),
            Err(e) => {
                Err(DisplayMsg::IncorrectFilePath(String::from("contract"), PathBuf::from(path), e.to_string()))
            },
        }
    } else {
        Err(DisplayMsg::IncorrectFilePath(
            String::from("contract"), PathBuf::from(path), String::from("Given file is not a wasm file")
        ))
    }
}

// `read_contract_code` returns contract codeas a vector of bytes.
//  # Arguments
//  * `path` - absolute path to .wasm file or contract bytecode encoded as a Base64URL encoded string
//
pub fn read_contract_code(path: &str) -> Result<Vec<u8>, DisplayMsg> {
    match check_contract_exist(path) {
        Ok(canonicalized_path ) => {
            match read_file(std::path::PathBuf::from(&canonicalized_path)){
                Ok(contract_code) => Ok(contract_code),
                Err(e) => {
                    Err(DisplayMsg::FailToOpenOrReadFile(String::from("contract"), PathBuf::from(path), e))
                }
            }
        },
        Err(e) => Err(e)
    }
}

/// [TransactionWithReceipt] is a wrapper over 
/// display_types::Transaction, Receipt and the equivalent index of the Transaction on ParallelChain.
#[derive(Serialize, Debug)]
pub struct TransactionWithReceipt {
    pub transaction: Transaction,
    pub receipt: Receipt
}

impl From<(pchain_types::blockchain::Transaction, pchain_types::blockchain::Receipt) > for TransactionWithReceipt {
    fn from((tx, receipt): (pchain_types::blockchain::Transaction, pchain_types::blockchain::Receipt)) -> TransactionWithReceipt {
        TransactionWithReceipt{
            transaction: From::<pchain_types::blockchain::Transaction>::from(tx),
            receipt: receipt.iter().map(|p|{
                From::<pchain_types::blockchain::CommandReceipt>::from(p.clone())
            }).collect()
        }
    }
}

impl From<pchain_types::blockchain::Log> for Event {
    fn from(event: pchain_types::blockchain::Log) -> Event {
        Event {
            topic: match Base64String::from_utf8(event.topic.clone()) {
                Ok(string_value) => format!("(UTF8) {}", string_value),
                Err(_) => format!("(Base64 encoded) {}", base64url::encode(&event.topic))
            },
            value: match Base64String::from_utf8(event.value.clone()) {
                Ok(string_value) => format!("(UTF8) {}", string_value),
                Err(_) => format!("(Base64 encoded) {}", base64url::encode(&event.value))
            },
        }
    }
}
