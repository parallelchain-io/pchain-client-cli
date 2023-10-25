/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Definition of methods related to setting up configuration of `pchain_client` and its networking.

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml::{map::Map, Value};

use crate::display_msg::DisplayMsg;

/// [Config] defines providers,
/// standard_api_url - the ParallelChain Standard API for fetching information related to blocks and transactions.
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub url: String,
}

impl Config {
    // `load` read config file in under $PCHAIN_CLI_HOME
    //
    // If config file does not exist, set up file and path and return empty config
    // If config file exists, read and return it
    //
    //  # Arguments
    //  *
    pub fn load() -> Config {
        let default_config_path = get_config_path();
        let config: Config = {
            let read_config = |default_config_path: PathBuf| -> String {
                // config file does not exist
                if (!Path::new(&default_config_path).is_file())
                    || (Path::new(&default_config_path).is_file()
                        && fs::read_to_string(&default_config_path).unwrap().is_empty())
                {
                    // create .toml config file on default_config_path
                    if !Path::new(&default_config_path.parent().unwrap()).exists() {
                        if let Err(e) =
                            std::fs::create_dir_all(default_config_path.parent().unwrap())
                        {
                            println!(
                                "{}",
                                DisplayMsg::FailToCreateDir(
                                    String::from("config"),
                                    default_config_path.to_path_buf(),
                                    e.to_string()
                                )
                            );
                            std::process::exit(1);
                        };
                    };

                    if let Err(e) = std::fs::File::create(&default_config_path) {
                        println!(
                            "{}",
                            DisplayMsg::FailToCreateFile(
                                String::from("config"),
                                default_config_path,
                                e.to_string()
                            )
                        );
                        std::process::exit(1);
                    };

                    let empty_config = Config::default();
                    let toml_string = toml::to_string(&empty_config).unwrap_or_else(|_| {
                        panic!(
                            "{}",
                            DisplayMsg::FailToTOMLEncode(
                                String::from("config"),
                                get_config_path(),
                                String::new(),
                            )
                            .to_string()
                        )
                    });

                    if let Err(e) = std::fs::write(&default_config_path, toml_string) {
                        println!(
                            "{}",
                            DisplayMsg::FailToWriteFile(
                                String::from("config"),
                                default_config_path,
                                e.to_string()
                            )
                        );
                        std::process::exit(1);
                    };
                };

                std::fs::read_to_string(default_config_path.clone()).unwrap_or_else(|_| {
                    panic!(
                        "{}",
                        DisplayMsg::FailToOpenOrReadFile(
                            String::from("config"),
                            default_config_path,
                            String::new(),
                        )
                        .to_string()
                    )
                })
            };

            toml::from_str(&read_config(default_config_path.clone())).unwrap_or_else(|_| {
                panic!(
                    "{}",
                    DisplayMsg::InvalidTOMLFormat(
                        String::from("config"),
                        default_config_path,
                        String::new(),
                    )
                    .to_string()
                )
            })
        };

        config
    }

    // `get_field_value` returns a field value corresponding to the field name from Config
    //  # Arguments
    //  * `Config` - RPC providers config url
    pub fn get_url(&self) -> &str {
        if self.url.is_empty() {
            println!("{}", DisplayMsg::NotYetSetRPCProvider);
            std::process::exit(1);
        }

        &self.url
    }

    // `update` updates Full RPC url in config.toml
    //  # Arguments
    //  * `Config` - RPC providers config url
    //  * `url` - new RPC providers config url
    pub fn update(&mut self, url: &str) {
        self.url = url.trim().trim_end_matches('/').to_string();
        self.save();
    }

    // save current config setting to file in toml
    //  # Arguments
    //  * `Config` - RPC providers config url
    pub fn save(&self) {
        let mut config_map = Map::new();
        let contents = serde_json::to_string(&self).unwrap();
        if contents.trim() != "" {
            config_map = match serde_json::from_str(&contents) {
                Ok(data) => data,
                Err(_) => {
                    // This leg mostlikely should be unreachable
                    println!(
                        "{}",
                        DisplayMsg::InvalidTOMLFormat(
                            String::from("config toml"),
                            get_config_path(),
                            String::new()
                        )
                    );
                    std::process::exit(1);
                }
            };
        };
        config_map.insert("url".to_string(), Value::from(self.url.clone()));
        let toml_string = toml::to_string(&Value::Table(config_map)).unwrap_or_else(|_| {
            panic!(
                "{}",
                DisplayMsg::FailToTOMLEncode(
                    String::from("config toml"),
                    get_config_path(),
                    String::new(),
                )
                .to_string()
            )
        });

        match std::fs::write(get_config_path(), toml_string) {
            Ok(_) => {
                println!("{}", DisplayMsg::ListRPCProvider(self.url.to_string()));
            }
            Err(e) => {
                println!(
                    "{}",
                    DisplayMsg::FailToWriteFile(
                        String::from("config toml"),
                        get_config_path(),
                        e.to_string()
                    )
                );
                std::process::exit(1);
            }
        };
    }
}

// `get_home_dir` returns path to pchain_client home directory set in enviroment variable.
//  # Arguments
//  *
pub fn get_home_dir() -> PathBuf {
    match std::env::var(PCHAIN_CLI_HOME_ENV_KEY) {
        Ok(home_path) => PathBuf::from(home_path),
        Err(_) => {
            println!(
                "{}",
                DisplayMsg::PChainCliHomeNotSet(String::from(PCHAIN_CLI_HOME_ENV_KEY))
            );
            std::process::exit(1);
        }
    }
}

// `get_config_path` returns path to config.toml
//  # Arguments
//  *
pub fn get_config_path() -> PathBuf {
    let mut file_path = get_home_dir();
    file_path.push(CONFIGURATION_FILENAME);
    file_path
}

// `get_keypair_path` returns path to keypair file
//  # Arguments
//  *
pub fn get_keypair_path() -> PathBuf {
    let mut default_keypair_path = get_home_dir();
    default_keypair_path.push(PCHAIN_CLI_KEYPAIR_FILENAME);

    default_keypair_path
}

// `get_hash_path` returns path to passphase hash
//  # Arguments
//  *
pub fn get_hash_path() -> PathBuf {
    let mut default_keypair_path = get_home_dir();
    default_keypair_path.push(PCHAIN_CLI_PASSPHASE_FILENAME);

    default_keypair_path
}

/// Env variable key for pchain_client home path
const PCHAIN_CLI_HOME_ENV_KEY: &str = "PCHAIN_CLI_HOME";

/// Default pchain_cli keypair filename
const PCHAIN_CLI_KEYPAIR_FILENAME: &str = "keypair";

/// Default pchain_cli passphase hash filename
const PCHAIN_CLI_PASSPHASE_FILENAME: &str = "hash";

/// Default path to config file
const CONFIGURATION_FILENAME: &str = "config.toml";
