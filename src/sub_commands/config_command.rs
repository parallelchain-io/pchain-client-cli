/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Methods related to subcommand `setup` in `pchain-client`.

use pchain_client::{ClientV1, NetworkProvider};
use config::Config;

use crate::display_msg::DisplayMsg;
use crate::command::ConfigCommand;
use crate::config;

// `match_setup_subcommand` matches a CLI argument to its corresponding `Setup` subcommand and processes 
//  the request.
//  # Arguments
//  * `setup_subcommand` - setup subcommand from CLI
//  
pub async fn match_setup_subcommand(setup_subcommand: ConfigCommand) {
    match setup_subcommand {
        ConfigCommand::Setup { url } => {
            let url = url.trim().trim_end_matches('/').to_string();
            if !ClientV1::new(&url).is_provider_up().await {
                println!("{}", DisplayMsg::InavtiveRPCProvider(url));
                std::process::exit(1);
            }

            Config::load().update(&url);
        },
        ConfigCommand::List => {
            let config = Config::load();
            let url = config.get_url();
            
            println!("{}", DisplayMsg::ListRPCProvider(url.to_string()));
            if !ClientV1::new(url).is_provider_up().await {
                println!("{}", DisplayMsg::InavtiveRPCProvider(String::from(url)));
            }
            else{
                println!("{}", DisplayMsg::ActiveRPCProvider(String::from(url)))
            }
        },
    };
}