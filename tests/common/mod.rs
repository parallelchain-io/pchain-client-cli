#![allow(dead_code)]

use std::{path::PathBuf, env};

use regex::Regex;
use temp_dir::TempDir;

pub(crate) struct TestEnv {
    pub cli_home: TempDir,
    pub bin: PathBuf,
}

impl TestEnv {
    pub fn new() -> Self {
        let cli_home = TempDir::new().unwrap();
        env::set_var("PCHAIN_CLI_HOME", cli_home.path().as_os_str().to_str().unwrap());
    
        let template_cli_home = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("cli_home");
        template_cli_home.read_dir().unwrap().filter_map(|f| f.ok()).for_each(|f| {
            let file_name = f.file_name();
            let file_path = f.path();
            std::fs::copy(file_path, cli_home.path().join(file_name)).unwrap();
        });
    
        let bin = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target").join("debug").join("pchain_client");
        Self { cli_home, bin }
    }

    pub fn add_file(&self, file_name: &str, content: &[u8]) -> PathBuf {
        let file_path = self.cli_home.path().join(file_name);
        std::fs::write(&file_path, content).unwrap();
        file_path
    }
}

pub fn expect_output(patterns: &[&str], output: &str) -> Result<(), String> {
    for p in patterns {
        Regex::new(p)
            .unwrap()
            .find(&output)
            .ok_or(format!("expected pattern {p} not found"))?;
    };

    Ok(())
}