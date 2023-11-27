/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Block to a form which can be displayed on the terminal.

use crate::display_types::{QuorumCertificate, Receipt, Transaction};
use serde::Serialize;

/// [Block] denotes a display_type equivalent of pchain_types::blockchain::Block
#[derive(Serialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
}

impl From<pchain_types::blockchain::BlockV1> for Block {
    fn from(block: pchain_types::blockchain::BlockV1) -> Block {
        let txs_beautified: Vec<Transaction> = block
            .transactions
            .into_iter()
            .map(From::<pchain_types::blockchain::TransactionV1>::from)
            .collect();
        let receipt_beautified: Vec<Receipt> = block
            .receipts
            .into_iter()
            .map(|protocol_type_receipt| {
                protocol_type_receipt
                    .into_iter()
                    .map(super::CommandReceipt::from)
                    .collect()
            })
            .collect();

        Block {
            header: From::<pchain_types::blockchain::BlockHeaderV1>::from(block.header),
            transactions: txs_beautified,
            receipts: receipt_beautified,
        }
    }
}

/// [BlockHeader] denotes a display_type equivalent for pchain_types::blockchain::BlockHeader
#[derive(Serialize, Debug)]
pub struct BlockHeader {
    pub chain_id: u64,
    pub block_hash: String,
    pub height: u64,
    pub justify: QuorumCertificate,
    pub data_hash: String,
    pub timestamp: u32,
    pub base_fee: u64,
    pub txs_hash: String,
    pub state_hash: String,
    pub receipts_hash: String,
    pub proposer: String,
}

impl From<pchain_types::blockchain::BlockHeaderV1> for BlockHeader {
    fn from(blockheader: pchain_types::blockchain::BlockHeaderV1) -> BlockHeader {
        BlockHeader {
            chain_id: blockheader.chain_id,
            height: blockheader.height,
            timestamp: blockheader.timestamp,
            base_fee: blockheader.base_fee_per_gas,
            justify: From::<hotstuff_rs::types::QuorumCertificate>::from(blockheader.justify),
            data_hash: base64url::encode(blockheader.data_hash),
            block_hash: base64url::encode(blockheader.hash),
            txs_hash: base64url::encode(blockheader.hash),
            state_hash: base64url::encode(blockheader.state_hash),
            receipts_hash: base64url::encode(blockheader.receipts_hash),
            proposer: base64url::encode(blockheader.proposer),
        }
    }
}
