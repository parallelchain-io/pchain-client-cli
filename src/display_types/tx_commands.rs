/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::blockchain::Command to a format which can be displayed on the terminal.
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use pchain_types::{ blockchain::Command, runtime::*, cryptography::PublicAddress};
use crate::display_msg::DisplayMsg;
use crate::command::Base64String;
use crate::display_types::read_contract_code;
use crate::parser::{base64url_to_public_address, serialize_call_arguments};

/// [TxCommand] denotes a display_types equivalent of pchain_types::blockchain::Command.
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

impl TryFrom<TxCommand> for Command {
     type Error = String;
 
     fn try_from(command: TxCommand) -> Result<Self, Self::Error> {
         match command {
             TxCommand::Call{ target, method, arguments, amount } => {
                 let target: PublicAddress = match base64url_to_public_address(&target) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("target"), target, e.to_string()).to_string());
                     },
                 };
 
                 Ok(Command::Call (CallInput{
                     target, method, arguments: CallArgument::serialize_arguments(arguments), amount
                 }))    
         
             },
             TxCommand::Transfer { recipient, amount } => {
                 let recipient: PublicAddress = match base64url_to_public_address(&recipient) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("recipient"), recipient, e.to_string()).to_string());
                     },
                 };
         
                 Ok(Command::Transfer(TransferInput{
                     recipient, amount
                 }))
             },
             TxCommand::Deploy { contract, cbi_version } => {
                 let contract_code = match read_contract_code(&contract) {
                     Ok(code) => code,
                     Err(e) => {
                        return Err(e.to_string());
                     },
                 };
 
                 Ok(Command::Deploy(DeployInput{
                     contract: contract_code, cbi_version
                 }))
 
             }
             TxCommand::CreatePool { commission_rate } => {
                 Ok(Command::CreatePool(CreatePoolInput{
                     commission_rate
                 }))
             },
             TxCommand::DeletePool => {
                 Ok(Command::DeletePool)
             },
             TxCommand::SetPoolSettings { commission_rate } => {
                 Ok(Command::SetPoolSettings(SetPoolSettingsInput{
                     commission_rate
                 }))
             },
             TxCommand::CreateDeposit { operator, balance, auto_stake_rewards } => {
                 let operator: PublicAddress = match base64url_to_public_address(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e.to_string()).to_string());
                     },
                 };
         
                 Ok(Command::CreateDeposit(CreateDepositInput{
                     operator, balance, auto_stake_rewards
                 }))
             },
             TxCommand::SetDepositSettings { operator, auto_stake_rewards } => {
                 let operator: PublicAddress = match base64url_to_public_address(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e.to_string()).to_string());
                     },
                 };
         
                 Ok(Command::SetDepositSettings(SetDepositSettingsInput{
                     operator, auto_stake_rewards
                 }))
             },
             TxCommand::TopUpDeposit { operator, amount } => {
                 let operator: PublicAddress = match base64url_to_public_address(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e.to_string()).to_string());
                     },
                 };
         
                 Ok(Command::TopUpDeposit(TopUpDepositInput{
                     operator, amount
                 }))
             },
             TxCommand::WithdrawDeposit { operator, max_amount } => {
                 let operator: PublicAddress = match base64url_to_public_address(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e.to_string()).to_string());
                     },
                 };
         
                 Ok(Command::WithdrawDeposit (WithdrawDepositInput{
                     operator, max_amount
                 }))
             },
             TxCommand::StakeDeposit { operator, max_amount } => {
                 let operator: PublicAddress = match base64url_to_public_address(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e.to_string()).to_string());
                     },
                 };
         
                 Ok(Command::StakeDeposit (StakeDepositInput{
                     operator, max_amount
                 }))
             },
             TxCommand::UnstakeDeposit { operator, max_amount } => {
                 let operator: PublicAddress = match base64url_to_public_address(&operator) {
                     Ok(addr) => addr,
                     Err(e) => {
                        return Err(DisplayMsg::FailToDecodeBase64Address(String::from("operator"), operator, e.to_string()).to_string());
                     },
                 };
         
                 Ok(Command::UnstakeDeposit (UnstakeDepositInput{
                     operator, max_amount
                 }))
             },
             TxCommand::NextEpoch => {
                 Ok(Command::NextEpoch )
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
                println!("{:?}", args);
                match serialize_call_arguments(&args.argument_value, &args.argument_type) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", DisplayMsg::FailToParseCallArguments(e.to_string()));
                        std::process::exit(1);
                    }
                }
            }).collect())
     }
}