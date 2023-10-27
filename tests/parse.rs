use std::process::Command;

use common::{TestEnv, expect_output};

mod common;

/// - Case:     User enters parse page
/// - Expect:   Display usage
/// - Command:  ./pchain_client parse
#[test]
fn test_parse() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("parse")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(&[
        "pchain_client-parse",
        "USAGE:"
    ], &output)
    .unwrap();
}

/// - Case:     User enters parse base64 encoding page
/// - Expect:   Display usage
/// - Command:  ./pchain_client parse base64-encoding
#[test]
fn test_parse_base64_encoding() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("parse")
        .arg("base64-encoding")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(&[
        "pchain_client-parse-base64-encoding",
        "USAGE:"
    ], &output)
    .unwrap();
}

/// - Case:     User parses input into encoded base64url string
/// - Expect:   Show the result of base64url string
/// - Command:  ./pchain_client parse base64-encoding --encode --value <VALUE>
#[test]
fn test_parse_base64_encoding_encode() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("parse")
        .arg("base64-encoding")
        .arg("--encode")
        .arg("--value")
        .arg("[0, 1, 2, 3]")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    assert_eq!(&output, "AAECAw\n")
}

/// - Case:     User parses base64url string input into decoded bytes
/// - Expect:   Show the result of decoded bytes
/// - Command:  ./pchain_client parse base64-encoding --decode --value <VALUE>
#[test]
fn test_parse_base64_encoding_decode() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("parse")
        .arg("base64-encoding")
        .arg("--decode")
        .arg("--value")
        .arg("AAECAw")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    assert_eq!(&output, "[0, 1, 2, 3]\n")
}

/// - Case:     User enters call result page
/// - Expect:   Display Usage
/// - Command:  ./pchain_client parse call-result
#[test]
fn test_parse_call_result() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("parse")
        .arg("call-result")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stderr).to_string();

    expect_output(&[
        "pchain_client-parse-call-result",
        "USAGE:"
    ], &output)
    .unwrap();
}

/// - Case:     User specifies a data type, parses base64url string into a decoded data
/// - Expect:   Show the value of the decoded data
/// - Command:  ./pchain_client parse call-result --value <VALUE> --data-type <DATA_TYPE>
#[test]
fn test_parse_call_result_from_data_type() {
    let env = TestEnv::new();

    let output = Command::new(&env.bin)
        .arg("parse")
        .arg("call-result")
        .arg("--value")
        .arg("AAECAw") // [0, 1, 2, 3]
        .arg("--data-type")
        .arg("u32")
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    assert_eq!(&output, "50462976\n");
}

/// - Case:     User specifies the schema file, and parses base64url string into a decoded data
/// - Expect:   Show the value of the decoded data
/// - Command:  ./pchain_client parse call-result --value <VALUE> --data-type <DATA_TYPE>
#[test]
fn test_parse_call_result_from_schema() {
    let env = TestEnv::new();
    let test_file = env.add_file("test.json",
    serde_json::json!([
        {"argument_type": "u8"},
        {"argument_type": "bool"},
        {"argument_type": "u16"},
    ]).to_string().as_bytes());
    

    let output = Command::new(&env.bin)
        .arg("parse")
        .arg("call-result")
        .arg("--value")
        .arg("AAECAw") // [0, 1, 2, 3]
        .arg("--schema-file")
        .arg(test_file.as_os_str().to_str().unwrap())
        .output()
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    assert_eq!(&output, "[0]: 0\n[1]: true\n[2]: 770\n");
}
