use std::process::Command;

use common::{TestEnv, expect_output};
use ed25519_dalek::Signature;
use pchain_types::cryptography::Keypair;
use rand_chacha::rand_core::OsRng;
use serde_json::Value;

mod common;

/// - Case:     User enters keys page
/// - Expect:   Display usage
/// - Command:  ./pchain_client keys
#[test]
fn test_keys() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("keys")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(&[
        "pchain_client-keys",
        "USAGE:"
    ], &output)
    .unwrap();
}

/// - Case:     User lists the keys
/// - Expect:   Display the list of keys
/// - Command:  ./pchain_client keys list
#[test]
fn test_keys_list() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("list")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[
        r"Keypair Name \(First 50 char\)",
    ], &output)
    .unwrap();
}

/// - Case:     User creates a keypair
/// - Expect:   The created keypair is set to the list of keys
/// - Command:  ./pchain_client keys create
#[test]
fn test_keys_create() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("create")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[
        "Successfully create"
    ], &output)
    .unwrap();

    let outputs: Vec<&str> = output.split(' ').collect();
    let keyname = outputs[2];
    let address = outputs[6].replace('<', "").replace(">", "").trim().to_string();
    
    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("list")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[
        keyname,
        &address,
    ], &output)
    .unwrap();
}

/// - Case:     User import a keypair, and then export the keypair
/// - Expect:   Keypair can be imported. The same keypair can be exported to a file.
/// - Command:  
///     - ./pchain_client keys import --public <PUBLIC> --private <PRIVATE> --name <NAME>
///     - ./pchain_client keys export --name <NAME> --destination <DESTINATION>
#[test]
fn test_keys_import_export() {
    let env = TestEnv::new();
    let env_export_key_path = env.cli_home.path().join("testkey.json");
    let (public, private) =
    {
        let mut osrng = OsRng{};
        let keypair = Keypair::generate(&mut osrng);
        ( 
            base64url::encode(keypair.public.as_bytes()),
            base64url::encode(keypair.secret.as_bytes())
        )
    };

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("import")
        .arg("--public")
        .arg(&public)
        .arg("--private")
        .arg(&private)
        .arg("--name")
        .arg("testkey")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[
        "Successfully add keypair with name testkey."
    ], &output)
    .unwrap();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("export")
        .arg("--name")
        .arg("testkey")
        .arg("--destination")
        .arg(format!("{}", env_export_key_path.to_str().unwrap()))
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[
        &format!("Keypair is saved at {}", env_export_key_path.to_str().unwrap()),
    ], &output)
    .unwrap();

    let exported_file = std::fs::read(env_export_key_path).unwrap();
    let exported_keypair: Value = serde_json::from_str(&String::from_utf8_lossy(&exported_file)).unwrap();

    assert_eq!(exported_keypair["name"].as_str().unwrap(), "testkey");
    assert_eq!(exported_keypair["public_key"].as_str().unwrap(), &public);
    assert_eq!(exported_keypair["private_key"].as_str().unwrap(), &private);
}

/// - Case:     User import a keypair, and then export the keypair
/// - Expect:   
/// - Command:  ./pchain_client keys sign
#[test]
fn test_keys_sign() {
    let env = TestEnv::new();

    let mut osrng = OsRng{};
    let keypair = Keypair::generate(&mut osrng);
    let public = base64url::encode(keypair.public.as_bytes());
    let private = base64url::encode(keypair.secret.as_bytes());

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("import")
        .arg("--public")
        .arg(&public)
        .arg("--private")
        .arg(&private)
        .arg("--name")
        .arg("testkey")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    expect_output(&[
        "Successfully add keypair with name testkey."
    ], &output)
    .unwrap();

    let output = Command::new(&env.bin)
        .arg("keys")
        .arg("sign")
        .arg("--message")
        .arg(base64url::encode(&[1u8, 2, 3, 4]))
        .arg("--name")
        .arg("testkey")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();
    
    expect_output(&[
        "Message: AQIDBA",
        "Ciphertext: ",
    ], &output)
    .unwrap();

    let ciphertext = output.split(' ').last().unwrap().trim();
    let signature = Signature::from_bytes(&base64url::decode(ciphertext).unwrap()).unwrap();
    assert!(keypair.verify(&[1u8, 2, 3, 4], &signature).is_ok());
}