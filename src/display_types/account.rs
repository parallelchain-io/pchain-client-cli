/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Data structures which convert pchain_types::rpc::Account to a form which can be displayed on the terminal.

use pchain_types::rpc::{AccountWithContract, AccountWithoutContract};
use serde::Serialize;
use crate::command::Base64Hash;

/// [Account] denotes a display_type equivalent of pchain_types::rpc::Account
#[derive(Serialize, Debug)]
pub struct Account {
    pub nonce: u64,
    pub balance: u64,
    pub cbi_version: Option<u32>,
    pub storage_hash: Option<Base64Hash>,
}

impl From<pchain_types::rpc::Account> for Account {
    fn from(account: pchain_types::rpc::Account) -> Account {
        let (nonce, balance, cbi_version, storage_hash) = match account{
            pchain_types::rpc::Account::WithContract( AccountWithContract { nonce, balance, contract:_, cbi_version, storage_hash }) => {
                (nonce, balance, cbi_version, storage_hash)
            },
            pchain_types::rpc::Account::WithoutContract(AccountWithoutContract { nonce, balance, cbi_version, storage_hash }) => {
                (nonce, balance, cbi_version, storage_hash)
            }
        };

        let storage_hash = match storage_hash{
            Some(hash) => Some(pchain_types::Base64URL::encode(hash).to_string()),
            None => None
        };

        Account{ nonce, balance, cbi_version, storage_hash }
    }
}