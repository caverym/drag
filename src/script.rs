use std::ffi::OsString;
use std::fs::File;
use crate::error::Result;
use std::io::Read;

#[derive(Debug, serde_derive::Deserialize)]
pub struct Script {
    config: Option<Vec<String>>,
    build: Option<Vec<String>>,
    install: Option<Vec<String>>,
    clean: Option<Vec<String>>,
}

impl Script {
    pub fn new(file: Option<OsString>) -> Result<Self> {
        let mut sc: String;

        if let Some(f) = file {
            sc = f.to_string_lossy().to_string();
        } else {
            sc = "./Dragrun".to_string();
        }

        let mut file: File = File::open(&sc)?;

        let mut buff: String =  String::new();
        file.read_to_string(&mut buff)?;

        let script: Self = toml::from_str(&buff)?;

        Ok(script)
    }
}
