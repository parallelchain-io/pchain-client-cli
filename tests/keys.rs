use std::{convert::TryInto, process::Command};

use common::{expect_output, TestEnv};
use ed25519_dalek::Signature;
use pchain_types::cryptography::Keypair;
use rand_chacha::rand_core::OsRng;
use serde_json::Value;
use serial_test::serial;

mod common;

/// - Case:     User enters keys page
/// - Expect:   Display usage
/// - Command:  ./pchain_client keys
#[test]
#[serial]
fn test_keys() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin).arg("keys").output().unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(&["pchain_client-keys", "USAGE:"], &output).unwrap();
}

/// - Case:     User lists the keys
/// - Expect:   Display the list of keys
/// - Command:  ./pchain_client keys list
#[test]
#[serial]
fn test_keys_list() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("list")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[r"Keypair Name \(First 50 char\)"], &output).unwrap();
}

/// - Case:     User creates a keypair
/// - Expect:   The created keypair is set to the list of keys
/// - Command:  ./pchain_client keys create
#[test]
#[serial]
fn test_keys_create() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("create")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&["Successfully create"], &output).unwrap();

    let outputs: Vec<&str> = output.split(' ').collect();
    let keyname = outputs[2];
    let address = outputs[6]
        .replace('<', "")
        .replace(">", "")
        .trim()
        .to_string();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("list")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[keyname, &address], &output).unwrap();
}

/// - Case:     User import a keypair, and then export the keypair
/// - Expect:   Keypair can be imported. The same keypair can be exported to a file.
/// - Command:  
///     - ./pchain_client keys import --public <PUBLIC> --private <PRIVATE> --keypair_name <NAME>
///     - ./pchain_client keys export --keypair_name <NAME> --destination <DESTINATION>
#[test]
#[serial]
fn test_keys_import_export() {
    let env = TestEnv::new();
    let env_export_key_path = env.cli_home.path().join("testkey.json");
    let (public, private) = {
        let mut osrng = OsRng {};
        let keypair = Keypair::generate(&mut osrng);

        let verifying = keypair.verifying_key().clone();
        let public = verifying.as_bytes();
        let secret = keypair.as_bytes();
        (base64url::encode(public), base64url::encode(secret))
    };

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("import")
        .arg("--public")
        .arg(&public)
        .arg("--private")
        .arg(&private)
        .arg("--keypair_name")
        .arg("testkey")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&["Successfully add keypair with name testkey."], &output).unwrap();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("export")
        .arg("--keypair_name")
        .arg("testkey")
        .arg("--destination")
        .arg(format!("{}", env_export_key_path.to_str().unwrap()))
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(
        &[&format!(
            "Keypair is saved at {}",
            env_export_key_path.to_str().unwrap()
        )],
        &output,
    )
    .unwrap();

    let exported_file = std::fs::read(env_export_key_path).unwrap();
    let exported_keypair: Value =
        serde_json::from_str(&String::from_utf8_lossy(&exported_file)).unwrap();

    assert_eq!(exported_keypair["keypair_name"].as_str().unwrap(), "testkey");
    assert_eq!(exported_keypair["public_key"].as_str().unwrap(), &public);
    assert_eq!(exported_keypair["private_key"].as_str().unwrap(), &private);
}

/// - Case:     User import a keypair, and then export the keypair
/// - Expect:   
/// - Command:  ./pchain_client keys sign
#[test]
#[serial]
fn test_keys_sign() {
    let env = TestEnv::new();

    let mut osrng = OsRng {};
    let keypair = Keypair::generate(&mut osrng);
    let private = base64url::encode(keypair.as_bytes());
    let verifying = keypair.verifying_key().clone();
    let public = base64url::encode(verifying.as_bytes());

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("import")
        .arg("--public")
        .arg(&public)
        .arg("--private")
        .arg(&private)
        .arg("--keypair_name")
        .arg("testkey")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&["Successfully add keypair with name testkey."], &output).unwrap();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("sign")
        .arg("--message")
        .arg(base64url::encode(&[1u8, 2, 3, 4]))
        .arg("--keypair_name")
        .arg("testkey")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&["Message: AQIDBA", "Ciphertext: "], &output).unwrap();

    let ciphertext = output.split(' ').last().unwrap().trim();
    let signature =
        Signature::from_bytes(&base64url::decode(ciphertext).unwrap().try_into().unwrap());
    assert!(keypair.verify(&[1u8, 2, 3, 4], &signature).is_ok());
}
