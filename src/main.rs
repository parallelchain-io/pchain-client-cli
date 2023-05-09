/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

/// `command` defines the set of client commands of the program.
pub mod command;

/// `sub_commands` defines a new set of methods which process the requests from CLI commands
pub mod sub_commands;

/// display_msg` defines list of displayed messages, which give user insight to the result of their command.
/// For example, successful execution, reason of failing to execute command, cannot find relevent data.
pub mod display_msg;

/// `display_types` defines attributes defined in ParallelChain protocol to human readable format.
/// For example, data which are originally in bytes will be displayed in base64 encoded string.
pub mod display_types;

/// `result` defines the methods to process and generate result of command execution from the terminal.
pub mod result;

/// `setup` defines configuration and file I/O of the program.
pub mod config;

/// `keypair` defines all utilities required to generate/append to a JSON file, to hold public and private 
/// keys to your accounts on ParallelChain.
pub mod keypair;

/// `utils` defines methods to read file and generate random string for keypair name.
pub mod utils;

extern crate argon2;
use clap::Parser;
use config::{Config, get_hash_path};
use command::PChainCLI;

use crate::sub_commands::{
    match_submit_subcommand, 
    match_query_subcommand,
    match_crypto_subcommand, match_parse_subcommand,
    match_setup_subcommand
};

#[tokio::main]
async fn main() {
    let config = Config::load();

    let default_hash_file = get_hash_path();
    if !default_hash_file.exists(){
        match utils::setup_password(){
            Ok(()) => keypair::setup_keypair_file(),
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
    }

    let args = PChainCLI::parse();

    match args {
        PChainCLI::Config { config_subcommand } => {
            match_setup_subcommand(config_subcommand).await
        },
        PChainCLI::Transaction { tx_subcommand } => {
            match_submit_subcommand(tx_subcommand, config).await
        },
        PChainCLI::Query { query_subcommand } => { 
            match_query_subcommand(query_subcommand, config).await
        },
        PChainCLI::Keys { crypto_subcommand } => {
            match_crypto_subcommand(crypto_subcommand)
        },
        PChainCLI::Parse { parse_subcommand } => {
            match_parse_subcommand(parse_subcommand)
        },
    };
}



