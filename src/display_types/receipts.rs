/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Receipt to a format which can be displayed on the terminal.

use pchain_types::blockchain::{CommandReceiptV2, ExitCodeV2};
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

pub type Receipt = Vec<CommandReceipt>;

#[derive(Serialize, Debug)]
pub struct CallReceipt {
    pub exit_code: String,
    pub gas_used: u64,
    pub return_values: String,
    pub logs: Vec<Event>,
}

impl From<pchain_types::blockchain::CommandReceiptV1> for CallReceipt {
    fn from(receipt: pchain_types::blockchain::CommandReceiptV1) -> CallReceipt {
        let events_beautified: Vec<Event> = receipt
            .logs
            .into_iter()
            .map(|pchain_types_event|{
                From::<pchain_types::blockchain::Log>::from(pchain_types_event)
            })
            .collect();

        let exit_code = format!("{:?}", receipt.exit_code);

        CallReceipt {
            exit_code,
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

impl From<pchain_types::blockchain::CommandReceiptV2> for CallReceipt {
    fn from(receipt: pchain_types::blockchain::CommandReceiptV2) -> CallReceipt {

        if let CommandReceiptV2::Call(receipt) = receipt {
            let exit_code = format!("{:?}", receipt.exit_code);

            let beautified_events: Vec<Event> = receipt
                .logs
                .into_iter()
                .map(|pchain_types_event| {
                    From::<pchain_types::blockchain::Log>::from(pchain_types_event)
                })
                .collect(); 

            CallReceipt {
                exit_code,
                gas_used: receipt.gas_used,
                return_values: if !receipt.return_value.is_empty() {
                    format!(
                        "(Base64 encoded) {}",
                        base64url::encode(&receipt.return_value)
                    )
                } else {
                    "".to_string()
                },
                logs: beautified_events,
            }
        } else {
            todo!("some kind of error");

        }

    }
}


#[derive(Serialize, Debug)]
pub struct CommonReceipt {
    pub exit_code: String,
    pub gas_used: u64,
}

#[derive(Serialize, Debug)]
pub struct DepositReceipt {
    pub exit_code: String,
    pub gas_used: u64,
    pub amount: u64
}

#[derive(Serialize, Debug)]
pub struct Receipt2 {
    pub exit_code: String,
    pub gas_used: u64,
    pub amount: Option<u64>,
    pub return_values: Option<String>,
    pub logs: Option<Vec<Event>>,
}

impl From<pchain_types::blockchain::CommandReceiptV1> for Receipt2 {
    fn from(receipt: pchain_types::blockchain::CommandReceiptV1) -> Receipt2 {
        let events_beautified: Vec<Event> = receipt
            .logs
            .into_iter()
            .map(|pchain_types_event|{
                From::<pchain_types::blockchain::Log>::from(pchain_types_event)
            })
            .collect();

        let exit_code = format!("{:?}", receipt.exit_code);

        Receipt2 {
            exit_code,
            gas_used: receipt.gas_used,
            return_values: if !receipt.return_values.is_empty() {
                Some(format!(
                    "(Base64 encoded) {}",
                    base64url::encode(&receipt.return_values)
                ))
            } else {
                Some("".to_string())
            },
            logs: Some(events_beautified),
            amount: None,
        }
    }
}

impl From<pchain_types::blockchain::CommandReceiptV2> for Receipt2 {
    fn from(receipt: pchain_types::blockchain::CommandReceiptV2) -> Receipt2 {

        let receipt: Receipt2 = match receipt {
            CommandReceiptV2::Transfer(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::Deploy(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::CreatePool(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::SetPoolSettings(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::DeletePool(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::CreateDeposit(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::SetDepositSettings(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::TopUpDeposit(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::NextEpoch(r) => to_common_receipt(r.exit_code, r.gas_used),
            CommandReceiptV2::Call(r) => to_call_receipt(r),
            CommandReceiptV2::WithdrawDeposit(r) => to_deposit_receipt(r.exit_code, r.gas_used, r.amount_withdrawn),
            CommandReceiptV2::StakeDeposit(r) => to_deposit_receipt(r.exit_code, r.gas_used, r.amount_staked),
            CommandReceiptV2::UnstakeDeposit(r) => to_deposit_receipt(r.exit_code, r.gas_used, r.amount_unstaked),
        };

        receipt
    }
}

fn to_common_receipt(exit_code: ExitCodeV2, gas_used: u64) -> Receipt2 {
    Receipt2 {
        exit_code: format!("{:?}", exit_code),
        gas_used,
        amount: None,
        return_values: None,
        logs: None,
    }
}

fn to_call_receipt(receipt: pchain_types::blockchain::CallReceipt) -> Receipt2 {

    let return_values = if !receipt.return_value.is_empty() {
        format!(
            "(Base64 encoded) {}",
            base64url::encode(&receipt.return_value)
        )
    } else {
        "".to_string()
    };

    let beautified_events: Vec<Event> = receipt
        .logs
        .into_iter()
        .map(|pchain_types_event| {
            From::<pchain_types::blockchain::Log>::from(pchain_types_event)
        })
        .collect(); 

    Receipt2 {
        exit_code: format!("{:?}", receipt.exit_code),
        gas_used: receipt.gas_used,
        amount: None,
        return_values: Some(return_values),
        logs: Some(beautified_events),
    }
}

fn to_deposit_receipt(exit_code: ExitCodeV2, gas_used: u64, amount: u64) -> Receipt2 {
    Receipt2 {
        exit_code: format!("{:?}", exit_code),
        gas_used,
        amount: Some(amount),
        return_values: None,
        logs: None,
    }
}