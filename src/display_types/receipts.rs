/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Receipt to a format which can be displayed on the terminal.

use serde::Serialize;

/// [Event] denotes a display_types equivalent of pchain_types::blockchain::Log.
#[derive(Serialize, Debug)]
pub struct Event {
    pub topic: String,
    pub value: String,
}

/// [CommandReceipt] denotes a display_types equivalent pchain_types::blockchain::CommandReceipt.
#[derive(Serialize, Debug)]
pub struct CommandReceipt {
    pub status_code: String,
    pub gas_used: u64,
    pub return_values: String,
    pub logs: Vec<Event>,
}

impl From<pchain_types::blockchain::CommandReceiptV1> for CommandReceipt {
    fn from(receipt: pchain_types::blockchain::CommandReceiptV1) -> CommandReceipt {
        let events_beautified: Vec<Event> = receipt
            .logs
            .into_iter()
            .map(|pchain_types_event|{
                From::<pchain_types::blockchain::Log>::from(pchain_types_event)
            })
            .collect();

        let status_code = format!("{:?}", receipt.exit_code);
        CommandReceipt {
            status_code,
            gas_used: receipt.gas_used,
            return_values: if !receipt.return_values.is_empty() {
                format!(
                    "(Base64 encoded) {}",
                    base64url::encode(&receipt.return_values)
                )
            } else {
                "".to_string()
            },
            logs: events_beautified,
        }
    }
}

// // todo!() - different implementation of this function because they have different structure
// impl From<pchain_types::blockchain::CommandReceiptV2> for CommandReceipt {
//     fn from(receipt: pchain_types::blockchain::CommandReceiptV2) -> CommandReceipt {
//         let events_beautified: Vec<Event> = receipt.logs.into_iter().map(
//             |pchain_types_event|{
//                 From::<pchain_types::blockchain::Log>::from(pchain_types_event)
//         }).collect();

//         let status_code = format!("{:?}", receipt);
//         CommandReceipt {
//             status_code,
//             gas_used: receipt.gas_used,
//             return_values: if !receipt.return_values.is_empty(){
//                 format!("(Base64 encoded) {}", base64url::encode(&receipt.return_values))
//             } else { "".to_string() },
//             logs: events_beautified,
//         }
//     }
// }


pub type Receipt = Vec<CommandReceipt>;
