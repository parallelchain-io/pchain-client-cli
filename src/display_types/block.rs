/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::Block to a form which can be displayed on the terminal.

use serde::Serialize;
use crate::display_types::{Transaction, Receipt, QuorumCertificate};

/// [Block] denotes a display_type equivalent of pchain_types::blockchain::Block 
#[derive(Serialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
}

impl From<pchain_types::blockchain::Block> for Block {
    fn from(block: pchain_types::blockchain::Block) -> Block {
        let txs_beautified: Vec<Transaction> = block.transactions.into_iter().map(
            From::<pchain_types::blockchain::Transaction>::from
        ).collect();
        let receipt_beautified: Vec<Receipt> = block.receipts.into_iter().map(
            |protocol_type_receipt|
            protocol_type_receipt.into_iter().map(|p|{
                From::<pchain_types::blockchain::CommandReceipt>::from(p)
            }).collect()
        ).collect();
        
        Block {
            header: From::<pchain_types::blockchain::BlockHeader>::from(block.header),
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
    pub base_fee:u64,
    pub txs_hash: String,
    pub state_hash: String,
    pub receipts_hash: String,
    pub proposer: String,
}

impl From<pchain_types::blockchain::BlockHeader> for BlockHeader {
    fn from(blockheader: pchain_types::blockchain::BlockHeader) -> BlockHeader {
        BlockHeader {
            chain_id: blockheader.chain_id,
            height: blockheader.height,
            timestamp: blockheader.timestamp,
            base_fee: blockheader.base_fee,
            justify: From::<hotstuff_rs::types::QuorumCertificate>::from(blockheader.justify),
            data_hash: base64url::encode(blockheader.data_hash),
            block_hash: base64url::encode(blockheader.hash),
            txs_hash: base64url::encode(blockheader.txs_hash),
            state_hash: base64url::encode(blockheader.state_hash),
            receipts_hash: base64url::encode(blockheader.receipts_hash),
            proposer: base64url::encode(blockheader.proposer),
        }
    }
}


