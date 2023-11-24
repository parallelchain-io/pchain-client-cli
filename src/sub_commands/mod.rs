/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

/// `keys` houses methods which process subcommands related to cryptographic operations
/// on ParallelChain, like generating keypairs, signing keypairs etc.
pub(crate) mod keys;
pub use keys::*;

/// `parse` houses methods which process subcommands related to parsing
/// and encoding pchain_types::CallData for submission of transactions to ParallelChain,
/// generating contract addresses etc.
pub(crate) mod parse;
pub use parse::*;

/// `query` houses methods which process subcommands related to querying ParallelChain
/// Fullnode RPC and Chain Scanner providers.
pub(crate) mod query;
pub use query::*;

/// `transaction` houses methods  which process subcommands related to submitting transactions
/// and update/modifing stake pools on ParallelChain.
pub(crate) mod transaction;
pub use transaction::*;

/// `config_command` houses methods which process subcommands related to setting up RPC url
pub(crate) mod config_command;
pub use config_command::*;
