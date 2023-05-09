/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Receipt to a format which can be displayed on the terminal.

use pchain_types::Base64URL;
use serde::Serialize;

/// [Event] denotes a display_types equivalent of
/// pchain_types::Log.
#[derive(Serialize, Debug)]
pub struct Event {
    pub topic: String,
    pub value: String
}

/// [CommandReceipt] denotes a display_types equivalent 
/// pchain_types::CommandReceipt.
#[derive(Serialize, Debug)]
pub struct CommandReceipt {
    pub status_code: String,
    pub gas_used: u64,
    pub return_values: String,
    pub logs: Vec<Event>,
}

impl From<pchain_types::CommandReceipt> for CommandReceipt {
    fn from(receipt: pchain_types::transaction::CommandReceipt) -> CommandReceipt {
        let events_beautified: Vec<Event> = receipt.logs.into_iter().map(
            |pchain_types_event|{
                From::<pchain_types::transaction::Log>::from(pchain_types_event)
        }).collect();

        let status_code = format!("{:?}", receipt.exit_status);
        CommandReceipt {
            status_code,
            gas_used: receipt.gas_used,
            return_values: if !receipt.return_values.is_empty(){
                format!("(Base64 encoded) {}", Base64URL::encode(&receipt.return_values).to_string())
            } else { "".to_string() },
            logs: events_beautified,
        }
    }
}

pub type Receipt = Vec<CommandReceipt>;




