/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Definition of commands and their arguments served by `pchain-client`.

use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand};

pub type Base64Address = String;
pub type Base64Hash = String;
pub type Base64String = String;

/// A CLI for submitting Transactions to, and querying data from, the ParallelChain.  
#[derive(Debug, Parser)]
#[clap(name = "ParallelChain Client CLI", about = "ParallelChain Client CLI (`pchain_client`) is a command-line tool for you to connect and interact with the ParallelChain Mainnet/Testnet.", author = "<ParallelChain Lab>", long_about = None, version)]
pub(crate) enum PChainCLI {
    /// Construct and submit Transactions to ParallelChain network.
    #[clap(display_order = 1)]
    Transaction {
        #[clap(subcommand)]
        tx_subcommand: Transaction,
    },

    /// Query blockchain and world state information from ParallelChain network.
    #[clap(display_order = 2)]
    Query {
        #[clap(subcommand)]
        query_subcommand: Query,
    },

    /// Locally store and manage account keypairs you created. (Password required)
    #[clap(display_order = 3)]
    Keys {
        #[clap(subcommand)]
        crypto_subcommand: Keys,
    },

    /// Utility functions to deserialize return values in CommandReceipt, and compute contract address.
    #[clap(display_order = 4)]
    Parse {
        #[clap(subcommand)]
        parse_subcommand: Parse,
    },

    /// Get and set Fullnode RPC url to interact with ParallelChain.
    #[clap(display_order = 5)]
    Config {
        #[clap(subcommand)]
        config_subcommand: ConfigCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum Transaction {
    /// Create new Transaction with command and save to a JSON file.
    /// You are required to specify the transaction version.
    #[clap(display_order = 1)]
    #[clap(group(ArgGroup::new("version").required(true).multiple(false).args(&["v1", "v2"])))]
    Create {
        /// [Optional] Destination path of the output Transaction file. If not provided, default save file to current directory with filename `tx.json`.
        /// File with same name will be OVERWRITTEN. Directory provided has to exist.
        #[clap(long = "destination", display_order = 1)]
        destination: Option<String>,

        /// [One of] Specify this flag when submitting TransactionV1.
        #[clap(long = "v1", display_order = 2)]
        v1: bool,

        /// [One of] Specify this flag when submitting TransactionV2.
        #[clap(long = "v2", display_order = 3)]
        v2: bool,

        /// Number of Transactions originating from the Account so far in the ParallelChain network.
        #[clap(long = "nonce", display_order = 4)]
        nonce: u64,

        /// The maximum number of gas units that can be used in executing this transaction.
        #[clap(long = "gas-limit", display_order = 5)]
        gas_limit: u64,

        /// The maximum number of Grays that you are willing to burn for the gas unit used in this transaction.
        #[clap(long = "max-base-fee-per-gas", display_order = 6)]
        max_base_fee_per_gas: u64,

        /// The number of Grays that you are willing to pay the block proposer for including this transaction in a block.
        #[clap(long = "priority-fee-per-gas", display_order = 7)]
        priority_fee_per_gas: u64,

        #[clap(subcommand)]
        create_tx_subcommand: CreateTx,
    },
    /// Append additional command to existing Transaction file
    #[clap(display_order = 2)]
    Append {
        /// Relative/absolute path to a JSON file of Transaction.
        #[clap(long = "file", display_order = 1)]
        file: String,

        #[clap(subcommand)]
        create_tx_subcommand: CreateTx,
    },
    /// Submit a Transaction to ParallelChain by json file. (Password required)
    #[clap(arg_required_else_help = true, display_order = 3)]
    Submit {
        /// Relative/absolute path to a JSON file of Transaction.
        #[clap(long = "file", display_order = 1)]
        file: String,

        /// Name of the keypair. You can use existing keypair or generate new keypair with your preferred name using `./pchain_client keys create --keypair-name <NAME>`.
        /// This is used to sign the transaction as it proves 'you' are authorized to make this transaction.
        #[clap(long = "keypair-name", display_order = 2)]
        keypair_name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Query {
    /// Query an Account's balance (in Grays).
    #[clap(arg_required_else_help = true, display_order = 1)]
    Balance {
        /// Address of the External or Contract Account you'd like to query.
        #[clap(long = "address", display_order = 1, allow_hyphen_values(true))]
        address: Base64Address,
    },

    /// Query the number of Transactions originating from an External Account that has been included on ParallelChain (a.k.a., the nonce).
    #[clap(arg_required_else_help = true, display_order = 2)]
    Nonce {
        /// Address of the External Account you'd like to query.
        #[clap(long = "address", display_order = 1, allow_hyphen_values(true))]
        address: Base64Address,
    },

    /// Query a Contract Account's Contract Byte Code (Base64 encoded).
    #[clap(arg_required_else_help = true, display_order = 3)]
    Contract {
        /// Address of the External Account you'd like to query.
        #[clap(long = "address", display_order = 1, allow_hyphen_values(true))]
        address: Base64Address,

        /// [Optional] Destination path of the output contract code file. If not provided, default save file to current directory with filename `code.wasm`.
        /// File with same name will be OVERWRITTEN. Directory provided has to exist.
        #[clap(long = "destination", display_order = 3)]
        destination: Option<String>,
    },

    /// Query Key stored in Contract Account storage on world state.
    #[clap(arg_required_else_help = true, display_order = 4)]
    Storage {
        /// Address of interested contract
        #[clap(long = "address", display_order = 1, allow_hyphen_values(true))]
        address: Base64Address,

        /// Key of world state. BASE64 encoded of key defined in contract
        #[clap(long = "key", display_order = 2)]
        key: Base64String,
    },

    /// Trigger the Contract's view method.
    #[clap(arg_required_else_help = true, display_order = 5)]
    View {
        /// The address of the target contract
        #[clap(long = "target", display_order = 1, allow_hyphen_values(true))]
        target: Base64Address,

        /// The name of the method to be invoked
        #[clap(long = "method", display_order = 2)]
        method: String,

        /// [Optional] Relative / absolute path of the JSON file that specifies arguments to be supplied to the invoked method.
        #[clap(long = "arguments", display_order = 3)]
        arguments: Option<String>,
    },

    /// Query block information. Search the block either by block height, block hash or tx hash.
    /// You are required to specify one of the optional parameter.
    #[clap(arg_required_else_help = true, display_order = 6)]
    #[clap(group(ArgGroup::new("block").required(true).multiple(false).args(&["block-height", "block-hash", "tx-hash", "latest"])))]
    Block {
        /// [Optional] Block height of the Block you'd like to query.
        #[clap(long = "block-height", display_order = 1)]
        block_height: Option<u64>,

        /// [Optional]: Block hash of the Block you'd like to query.
        #[clap(long = "block-hash", display_order = 2, allow_hyphen_values(true))]
        block_hash: Option<Base64Hash>,

        /// [Optional]: Hash of the Transaction you'd like to query the containing Block of.
        #[clap(long = "tx-hash", display_order = 3, allow_hyphen_values(true))]
        tx_hash: Option<Base64Hash>,

        /// [Optional]: Specify this flag to query from the latest block
        #[clap(long = "latest", display_order = 4)]
        latest: bool,
    },

    /// Query block header only. Search the block either by block height, block hash or tx hash.
    /// You are required to specify one of the optional parameter.
    #[clap(arg_required_else_help = true, display_order = 7)]
    #[clap(group(ArgGroup::new("blockheader").required(true).multiple(false).args(&["block-height", "block-hash", "tx-hash", "latest"])))]
    BlockHeader {
        /// [Optional] Block height of the Block you'd like to query.
        #[clap(long = "block-height", display_order = 1)]
        block_height: Option<u64>,

        /// [Optional] Block hash of the Block you'd like to query.
        #[clap(long = "block-hash", display_order = 2, allow_hyphen_values(true))]
        block_hash: Option<Base64Hash>,

        /// [Optional] Hash of the Transaction you'd like to query the containing Block of.
        #[clap(long = "tx-hash", display_order = 3, allow_hyphen_values(true))]
        tx_hash: Option<Base64Hash>,

        /// [Optional] Specify this flag to query from the latest block
        #[clap(long = "latest", display_order = 4)]
        latest: bool,
    },

    /// Query Transaction information by specifying tx hash. Optional parameter to include receipt in result.
    #[clap(arg_required_else_help = true, display_order = 8)]
    Tx {
        /// Transaction hash of the Transaction you'd like to query.
        #[clap(long = "hash", display_order = 1, allow_hyphen_values(true))]
        tx_hash: Base64Hash,
    },

    /// Query Transaction Receipt by tx hash.
    #[clap(arg_required_else_help = true, display_order = 9)]
    Receipt {
        /// Transaction hash of the Transaction you'd like to query.
        #[clap(long = "hash", display_order = 2, allow_hyphen_values(true))]
        tx_hash: Base64Hash,
    },

    /// Query information related to Deposit
    #[clap(arg_required_else_help = true, display_order = 10)]
    Deposit {
        /// Address of the operator account of a stake pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// Address of the owner account that submitted a stake.
        #[clap(long = "owner", display_order = 2, allow_hyphen_values(true))]
        owner: Base64Address,
    },

    /// Query information related to Pools
    #[clap(arg_required_else_help = true, display_order = 11)]
    Pool {
        /// Address of the operator account of a stake pool.
        #[clap(long = "operator", display_order = 1)]
        operator: Base64Address,

        /// [Optional] Include stakes in result.
        #[clap(long = "with-stakes", display_order = 2)]
        with_stakes: bool,
    },

    /// Query information related to Stakes
    #[clap(arg_required_else_help = true, display_order = 12)]
    Stake {
        /// Address of the operator account of a stake pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// Address of the owner account that submitted a stake.
        #[clap(long = "owner", display_order = 2, allow_hyphen_values(true))]
        owner: Base64Address,
    },

    /// Query Validator Sets
    #[clap(arg_required_else_help = true, display_order = 13)]
    Validators {
        #[clap(subcommand)]
        validator_subcommand: Validators,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Set up RPC url configuration.
    #[clap(arg_required_else_help = true, display_order = 2)]
    Setup {
        /// The HTTP/HTTPS URL of Fullnode RPC to submit and query information from ParallelChain.
        #[clap(long = "url", required = true, display_order = 1)]
        url: String,
    },
    /// Show RPC url configuration with status.
    #[clap(display_order = 3)]
    List,
}

#[derive(Debug, Subcommand)]
pub enum Keys {
    /// List the Keypairs that you added to pchain_client.
    #[clap(arg_required_else_help = false, display_order = 1)]
    List,

    /// Generate and save an ed25519 Keypair.
    #[clap(display_order = 2)]
    Create {
        /// [Optional] The name to identify the Keypair that you are generating.
        #[clap(long = "keypair-name", display_order = 1)]
        keypair_name: Option<String>,
    },

    /// Import an existing keypair.
    #[clap(arg_required_else_help = true, display_order = 3)]
    Import {
        /// The private key of your ParallelChain account.
        #[clap(long = "private", display_order = 1, allow_hyphen_values(true))]
        private_key: Base64Address,

        /// The public key of your ParallelChain account.
        #[clap(long = "public", display_order = 2, allow_hyphen_values(true))]
        public_key: Base64Address,

        /// The name to identify the Keypair.
        #[clap(long = "keypair-name", display_order = 3, allow_hyphen_values(true))]
        keypair_name: String,
    },

    /// Export existing keypair to JSON file
    #[clap(arg_required_else_help = true, display_order = 4)]
    Export {
        /// The name to identify the Keypair.
        #[clap(long = "keypair-name", display_order = 1)]
        keypair_name: String,

        /// [Optional] Destination path of the output transaction file. If not provided, default save json file to current directory with filename of the keypair name.
        /// File with same name will be OVERWRITTEN. Directory provided has to exist.
        #[clap(long = "destination", display_order = 2)]
        destination: Option<String>,
    },

    /// Sign a message using registered Keypair and return Base64 encoded ciphertext.
    #[clap(arg_required_else_help = true, display_order = 5)]
    Sign {
        /// A message to sign, encoded in Base64.
        #[clap(long = "message", display_order = 1, allow_hyphen_values(true))]
        message: String,

        /// The name to identify the Keypair.
        #[clap(long = "keypair-name", display_order = 2)]
        keypair_name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Parse {
    /// Encode / decode the provided array / string
    #[clap(arg_required_else_help = true, display_order = 1)]
    #[clap(group(ArgGroup::new("encoding").required(true).multiple(false).args(&["encode", "decode"])))]
    Base64Encoding {
        /// [One Of] Base64 Encode Mode: encode array to Base64 string
        #[clap(long = "encode", display_order = 1)]
        encode: bool,

        /// [One Of] Base64 Decode Mode: decode Base64 string to array
        #[clap(long = "decode", display_order = 2)]
        decode: bool,

        /// The Base64 string to decode. / The byte array to encode. Please wrap value with quotation marks like "[8,8,8]" or "AAAA"
        #[clap(long = "value", display_order = 3, allow_hyphen_values(true))]
        value: String,
    },

    /// Parse the return value from a Contract call and display them in human-readable form.
    #[clap(arg_required_else_help = true, display_order = 2)]
    CallResult {
        /// The returned Base64 string from result of contract call.
        #[clap(long = "value", display_order = 1)]
        value: String,

        /// Common acceptable data types includes i32, i64, u8, u32, u64, bool, String, [u8;32], [u8;64].
        /// Details of supported data types can be found on https://github.com/parallelchain-io/pchain-client-cli/blob/master/example/arguments.json
        /// Notes:
        /// - Example values in Vec or slice: [0,1,2].
        /// - When decoding [u8;32] or [u8;64], one should wrap the type with quotation marks like "[u8;32]"
        ///
        /// This argument cannot be used together with "schema-file".
        #[clap(
            long = "data-type",
            display_order = 2,
            group = "gp-data-type",
            required = true
        )]
        data_type: Option<String>,

        /// Path to schema file for decoding the call result.
        ///
        /// This argument cannot be used together with "data-type".
        #[clap(
            long = "schema-file",
            display_order = 3,
            group = "gp-data-type",
            required = true
        )]
        schema_file: Option<PathBuf>,
    },

    /// Compute the contract address of a Contract in transaction.
    #[clap(arg_required_else_help = true, display_order = 3)]
    ContractAddress {
        #[clap(subcommand)]
        version: ContractAddressVersion,
    },
}

pub enum Base64Encode {
    Encode,
    Decode,
}

#[derive(Debug, Subcommand)]
pub enum Validators {
    /// Get validator set in previous epoch.
    #[clap(arg_required_else_help = false, display_order = 1)]
    Previous {
        /// [Optional] Include delegator set in result.
        #[clap(long = "with-delegator", display_order = 1)]
        with_delegator: bool,
    },

    /// Get validator set in current epoch.
    #[clap(arg_required_else_help = false, display_order = 2)]
    Current {
        /// [Optional] Include delegator set in result.
        #[clap(long = "with-delegator", display_order = 1)]
        with_delegator: bool,
    },

    /// Get validator set in next epoch.
    #[clap(arg_required_else_help = false, display_order = 3)]
    Next {
        /// [Optional] Include delegator set in result.
        #[clap(long = "with-delegator", display_order = 1)]
        with_delegator: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum ContractAddressVersion {
    /// Parse the first version of contract address which is defined in ParallelChain Protocol V0.4.
    #[clap(arg_required_else_help = false, display_order = 1)]
    V1 {
        /// Address of the signer account.
        #[clap(long = "address", display_order = 1, allow_hyphen_values(true))]
        address: Base64Address,

        /// Nonce of the signer account when deploying the contract.
        #[clap(long = "nonce", display_order = 2)]
        nonce: u64,
    },

    /// Parse the second version of contract address which is defined in ParallelChain Protocol V0.5.
    #[clap(arg_required_else_help = false, display_order = 2)]
    V2 {
        /// Address of the signer account.
        #[clap(long = "address", display_order = 1, allow_hyphen_values(true))]
        address: Base64Address,

        /// Nonce of the signer account when deploying the contract.
        #[clap(long = "nonce", display_order = 2)]
        nonce: u64,

        /// Index of the deploy command in the transaction.
        #[clap(long = "deploy_cmd_index", display_order = 3)]
        index: u32,
    },
}

#[derive(Debug, Subcommand)]
pub enum CreateTx {
    /// Transfer Balance from transaction signer to recipient.
    #[clap(arg_required_else_help = true, display_order = 1)]
    Transfer {
        /// Address of the Recipient you'd like to transfer to.
        #[clap(long = "recipient", display_order = 1, allow_hyphen_values(true))]
        recipient: Base64Address,

        /// The amount of XPLL/TXPLL (in Grays) transferring to the specified target address.
        #[clap(long = "amount", display_order = 2)]
        amount: u64,
    },

    /// Deploy smart contract to the state of the blockchain.
    #[clap(arg_required_else_help = true, display_order = 2)]
    Deploy {
        /// Relative / absolute path of smart contract in format of WASM bytecode ('.wasm').
        #[clap(long = "contract-code", display_order = 1)]
        contract_code: String,

        /// Version of Contract Binary Interface.
        #[clap(long = "cbi-version", display_order = 2)]
        cbi_version: u32,
    },

    /// Trigger method call of a deployed smart contract.
    #[clap(arg_required_else_help = true, display_order = 3)]
    Call {
        /// The address of the target contract.
        #[clap(long = "target", display_order = 1, allow_hyphen_values(true))]
        target: Base64Address,

        /// The name of the method to be invoked.
        #[clap(long = "method", display_order = 2)]
        method: String,

        /// [Optional] Relative / absolute path of the JSON file that specifies arguments to be supplied to the invoked method.
        #[clap(long = "arguments", display_order = 3)]
        arguments: Option<String>,

        /// [Optional] The amount of XPLL/TXPLL (in Grays) sending to the target contract.
        #[clap(long = "amount", display_order = 4)]
        amount: Option<u64>,
    },

    /// Deposit balance into a network account.
    #[clap(display_order = 4)]
    Deposit {
        #[clap(subcommand)]
        deposit_tx_subcommand: DepositTx,
    },

    /// Stake to a particular Pool.
    #[clap(display_order = 5)]
    Stake {
        #[clap(subcommand)]
        stake_tx_subcommand: StakeTx,
    },

    /// Create and manage Pool
    #[clap(display_order = 6)]
    Pool {
        #[clap(subcommand)]
        pool_tx_subcommand: PoolTx,
    },
}

#[derive(Debug, Subcommand)]
pub enum PoolTx {
    /// Instantiation of a Pool in the network account.
    #[clap(arg_required_else_help = true, display_order = 1)]
    Create {
        /// The percentage (0-100%) of the epoch’s issuance rewarded to the pool that will go towards the operator’s stake.
        #[clap(long = "commission-rate", display_order = 1)]
        commission_rate: u8,
    },

    /// Update settings of an existing Pool.
    #[clap(arg_required_else_help = true, display_order = 2)]
    UpdateSettings {
        /// The percentage (0-100%) of the epoch’s issuance rewarded to the pool that will go towards the operator’s stake.
        #[clap(long = "commission-rate", display_order = 1)]
        commission_rate: u8,
    },

    /// Delete an existing Pool in the network account.
    #[clap(arg_required_else_help = false, display_order = 3)]
    Delete,
}

#[derive(Debug, Subcommand)]
pub enum DepositTx {
    /// Instantiation of a Deposit in an existing Pool.
    #[clap(arg_required_else_help = true, display_order = 1)]
    Create {
        /// The address of operator of the target pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// The deposit amount in Grays
        #[clap(long = "balance", display_order = 2)]
        balance: u64,

        /// Flag to indicate whether the received reward in epoch transaction should be automatically staked to the pool. Default is false.
        #[clap(long = "auto-stake-rewards", display_order = 3)]
        auto_stake_rewards: bool,
    },

    /// Increase balance of an existing Deposit.
    #[clap(arg_required_else_help = true, display_order = 2)]
    TopUp {
        /// The address of operator of the target pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// The amount (in Grays) added to Deposit's Balance.
        #[clap(long = "amount", display_order = 2)]
        amount: u64,
    },

    /// Withdraw balance from an existing Deposit.
    #[clap(arg_required_else_help = true, display_order = 3)]
    Withdraw {
        /// The address of operator of the target pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// The amount of deposits (in Grays) that the stake owner wants to withdraw. The prefix 'max'
        /// is denoted here because the actual withdrawal amount can be less than
        /// the wanted amount.
        #[clap(long = "max-amount", display_order = 2, allow_hyphen_values(true))]
        max_amount: u64,
    },

    /// Update settings of an existing Deposit.
    #[clap(arg_required_else_help = true, display_order = 4)]
    UpdateSettings {
        /// The address of operator of the target pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// Flag to indicate whether the received reward in epoch transaction should be automatically staked to the pool. Default is false.
        #[clap(long = "auto-stake-rewards", display_order = 2)]
        auto_stake_rewards: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum StakeTx {
    /// Increase stakes in an existing Pool.
    #[clap(arg_required_else_help = true, display_order = 8)]
    Stake {
        /// The address of operator of the target pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// The amount of stakes (in Grays) that the stake owner wants to stake to the target pool.
        /// The prefix 'max' is denoted here because the actual amount to be staked
        /// can be less than the wanted amount.
        #[clap(long = "max-amount", display_order = 2, allow_hyphen_values(true))]
        max_amount: u64,
    },

    /// Remove stakes from an existing Pool.
    #[clap(arg_required_else_help = true, display_order = 9)]
    Unstake {
        /// The address of operator of the target pool.
        #[clap(long = "operator", display_order = 1, allow_hyphen_values(true))]
        operator: Base64Address,

        /// The amount of stakes (in Grays) that the stake owner wants to remove from the target pool.
        /// The prefix 'max' is denoted here because the actual amount to be removed
        /// can be less than the wanted amount.
        #[clap(long = "max-amount", display_order = 2, allow_hyphen_values(true))]
        max_amount: u64,
    },
}
