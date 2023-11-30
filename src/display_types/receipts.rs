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

pub type Receipt = Vec<CommandReceipt>;

#[derive(Serialize, Debug)]
pub enum CommandReceipt {
    V1(V1Receipt),
    V2(V2Receipt),
}

#[derive(Serialize, Debug)]
pub struct V1Receipt {
    pub exit_code: String,
    pub gas_used: u64,
    pub return_values: String,
    pub logs: Vec<Event>,
}

impl From<pchain_types::blockchain::CommandReceiptV1> for CommandReceipt {
    fn from(receipt: pchain_types::blockchain::CommandReceiptV1) -> CommandReceipt {
        let events_beautified: Vec<Event> = receipt
            .logs
            .into_iter()
            .map(|pchain_types_event| {
                From::<pchain_types::blockchain::Log>::from(pchain_types_event)
            })
            .collect();

        let exit_code = format!("{:?}", receipt.exit_code);

        CommandReceipt::V1(V1Receipt {
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
        })
    }
}

#[derive(Serialize, Debug)]
pub struct V2Receipt {
    pub exit_code: String,
    pub gas_used: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_values: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<Event>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
}

impl V2Receipt {
    fn new(exit_code: ExitCodeV2, gas_used: u64) -> Self {
        Self {
            exit_code: format!("{:?}", exit_code),
            gas_used,
            return_values: None,
            logs: None,
            amount: None,
        }
    }

    fn return_values(mut self, return_values: Vec<u8>) -> Self {
        let str = if !return_values.is_empty() {
            format!("(Base64 encoded) {}", base64url::encode(&return_values))
        } else {
            "".to_string()
        };

        self.return_values = Some(str);
        self
    }

    fn logs(mut self, logs: Vec<pchain_types::blockchain::Log>) -> Self {
        let beautified_events: Vec<Event> = logs
            .into_iter()
            .map(|pchain_types_event| {
                From::<pchain_types::blockchain::Log>::from(pchain_types_event)
            })
            .collect();

        self.logs = Some(beautified_events);
        self
    }

    fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }
}

impl From<pchain_types::blockchain::CommandReceiptV2> for CommandReceipt {
    fn from(receipt: pchain_types::blockchain::CommandReceiptV2) -> CommandReceipt {
        let receipt: V2Receipt = match receipt {
            CommandReceiptV2::Transfer(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::Deploy(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::CreatePool(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::SetPoolSettings(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::DeletePool(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::CreateDeposit(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::SetDepositSettings(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::TopUpDeposit(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::NextEpoch(r) => V2Receipt::new(r.exit_code, r.gas_used),
            CommandReceiptV2::Call(r) => V2Receipt::new(r.exit_code, r.gas_used)
                .return_values(r.return_value)
                .logs(r.logs),
            CommandReceiptV2::WithdrawDeposit(r) => {
                V2Receipt::new(r.exit_code, r.gas_used).amount(r.amount_withdrawn)
            }
            CommandReceiptV2::StakeDeposit(r) => {
                V2Receipt::new(r.exit_code, r.gas_used).amount(r.amount_staked)
            }
            CommandReceiptV2::UnstakeDeposit(r) => {
                V2Receipt::new(r.exit_code, r.gas_used).amount(r.amount_unstaked)
            }
        };

        CommandReceipt::V2(receipt)
    }
}
