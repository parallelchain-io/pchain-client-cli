/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Command to a format which can be displayed on the terminal.
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use pchain_client_rs::base64url_to_bytes32;
use crate::display_msg::DisplayMsg;
use crate::command::Base64String;
use crate::display_types::read_contract_code;

/// [TxCommand] denotes a display_types equivalent of
/// pchain_types::Command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxCommand {
     Call {
          target: Base64String,
          method: String,
          arguments: Option<Vec<CallArgument>>,
          amount: Option<u64>
     },
     Deploy {
          contract: String,
          cbi_version: u32,
     },
     Transfer {
          recipient: Base64String,
          amount: u64
     },
     CreatePool {
          commission_rate: u8
     },
     DeletePool,
     SetPoolSettings {
          commission_rate: u8
     },
     CreateDeposit {
          operator: Base64String,
          balance: u64,
          auto_stake_rewards: bool,
     },
     SetDepositSettings {
          operator: Base64String,
          auto_stake_rewards: bool,
     },
     TopUpDeposit {
          operator: Base64String,
          amount: u64,
     },
     WithdrawDeposit {
          operator: Base64String,
          max_amount: u64,
     },
     StakeDeposit {
          operator: Base64String,
          max_amount: u64,
     },
     UnstakeDeposit {
          operator: Base64String,
          max_amount: u64,
     },
     NextEpoch,
}

impl TryFrom<TxCommand> for pchain_types::Command {
     type Error = String;
 
     fn try_from(command: TxCommand) -> Result<Self, Self::Error> {
         match command {
             TxCommand::Call{ target, method, arguments, amount } => {
                 let target: pchain_types::PublicAddress = match base64url_to_bytes32(&target) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("target"), target, e).to_string());
                     },
                 };
 
                 Ok(pchain_types::Command::Call {
                     target, method, arguments: CallArgument::serialize_arguments(arguments), amount
                 })    
         
             },
             TxCommand::Transfer { recipient, amount } => {
                 let recipient: pchain_types::PublicAddress = match base64url_to_bytes32(&recipient) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("recipient"), recipient, e).to_string());
                     },
                 };
         
                 Ok(pchain_types::Command::Transfer{
                     recipient, amount
                 })
             },
             TxCommand::Deploy { contract, cbi_version } => {
                 let contract_code = match read_contract_code(&contract) {
                     Ok(code) => code,
                     Err(e) => {
                        return Err(e.to_string());
                     },
                 };
 
                 Ok(pchain_types::Command::Deploy{
                     contract: contract_code, cbi_version
                 })
 
             }
             TxCommand::CreatePool { commission_rate } => {
                 Ok(pchain_types::Command::CreatePool{
                     commission_rate
                 })
             },
             TxCommand::DeletePool => {
                 Ok(pchain_types::Command::DeletePool{})
             },
             TxCommand::SetPoolSettings { commission_rate } => {
                 Ok(pchain_types::Command::SetPoolSettings{
                     commission_rate
                 })
             },
             TxCommand::CreateDeposit { operator, balance, auto_stake_rewards } => {
                 let operator: pchain_types::PublicAddress = match base64url_to_bytes32(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e).to_string());
                     },
                 };
         
                 Ok(pchain_types::Command::CreateDeposit{
                     operator, balance, auto_stake_rewards
                 })
             },
             TxCommand::SetDepositSettings { operator, auto_stake_rewards } => {
                 let operator: pchain_types::PublicAddress = match base64url_to_bytes32(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e).to_string());
                     },
                 };
         
                 Ok(pchain_types::Command::SetDepositSettings{
                     operator, auto_stake_rewards
                 })
             },
             TxCommand::TopUpDeposit { operator, amount } => {
                 let operator: pchain_types::PublicAddress = match base64url_to_bytes32(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e).to_string());
                     },
                 };
         
                 Ok(pchain_types::Command::TopUpDeposit{
                     operator, amount
                 })
             },
             TxCommand::WithdrawDeposit { operator, max_amount } => {
                 let operator: pchain_types::PublicAddress = match base64url_to_bytes32(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e).to_string());
                     },
                 };
         
                 Ok(pchain_types::Command::WithdrawDeposit {
                     operator, max_amount
                 })
             },
             TxCommand::StakeDeposit { operator, max_amount } => {
                 let operator: pchain_types::PublicAddress = match base64url_to_bytes32(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e).to_string());
                     },
                 };
         
                 Ok(pchain_types::Command::StakeDeposit {
                     operator, max_amount
                 })
             },
             TxCommand::UnstakeDeposit { operator, max_amount } => {
                 let operator: pchain_types::PublicAddress = match base64url_to_bytes32(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e).to_string());
                     },
                 };
         
                 Ok(pchain_types::Command::UnstakeDeposit {
                     operator, max_amount
                 })
             },
             TxCommand::NextEpoch => {
                 Ok(pchain_types::Command::NextEpoch {})
             },
         }
     }
 
 }

/// [CallArgument] defines type and value 
/// of arguments to a method defined in a smart contract 
/// depoyed on ParallelChain.
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallArgument {
     pub argument_type: String,
     pub argument_value: String
}

impl CallArgument{    
     // `serialize_arguments` serializes arguments attribute from pchain_types::Call in protocol.
     //  # Arguments
     //  * `arguments` - arguments of a pchain_types::Call type transaction
     pub fn serialize_arguments(arguments: Option<Vec<CallArgument>>) -> Option<Vec<Vec<u8>>> {
            if arguments.is_none() {
                return None;
            }
        
            Some(arguments.unwrap().into_iter().map(|args| {
                match pchain_client_rs::serialize_call_arguments(&args.argument_value, &args.argument_type) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", DisplayMsg::FailToParseCallArguments(e));
                        std::process::exit(1);
                    }
                }
            }).collect())
     }
}