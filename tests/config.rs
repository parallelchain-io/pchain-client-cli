use serial_test::serial;
use std::process::Command;

use crate::common::{expect_output, TestEnv};

mod common;

/// - Case:     User runs the program. (Not the first time)
/// - Expect:   Display CLI version and usage
/// - Command:  ./pchain_client
#[test]
#[serial]
fn test_pchain_client() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin).output().unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(
        &[
            &format!("ParallelChain Client CLI {}", env!("CARGO_PKG_VERSION")),
            "USAGE:",
        ],
        &output,
    )
    .unwrap();
}

/// - Case:     User enters config command page
/// - Expect:   display usage of config command
/// - Command:  ./pchain_client config
#[test]
#[serial]
fn test_config() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin).arg("config").output().unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(&["pchain_client-config", "USAGE:"], &output).unwrap();
}

/// - Case:     User lists RPC url configuration config
/// - Expect:   display RPC url configuration (empty)
/// - Command:  ./pchain_client config list
#[test]
#[serial]
fn test_config_list() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("config")
        .arg("list")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(output.is_empty());
}

/// - Case:     User setups RPC url configuration config
/// - Expect:   RPC url is set to configuration
/// - Command:  
///   - ./pchain_client config setup
///   - ./pchain_client config setup --url <URL>
#[test]
#[serial]
fn test_config_setup() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("config")
        .arg("setup")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(&["pchain_client-config-setup", "USAGE:"], &output).unwrap();

    let output = Command::new(&env.bin)
        .arg("config")
        .arg("setup")
        .arg("--url")
        .arg("https://pchain-test-rpc02.parallelchain.io")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(
        &["Fullnode RPC Provider is <https://pchain-test-rpc02.parallelchain.io>"],
        &output,
    )
    .unwrap();

    let output = Command::new(&env.bin)
        .arg("config")
        .arg("list")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(
        &["Fullnode RPC Provider is <https://pchain-test-rpc02.parallelchain.io>"],
        &output,
    )
    .unwrap();
}
