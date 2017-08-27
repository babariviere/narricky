use account::Account;
use error::*;
use rule::Rule;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value;

/// Configuration for a mail account
pub struct Config {
    pub account: Account,
    pub rules: Vec<Rule>,
}

impl Config {
    /// Convert toml table into Config
    pub fn from_toml(toml: Value) -> Result<Config> {
        let mut rules = Vec::new();
        let account = match toml.get("account") {
            Some(val) => Account::from_toml(val)?,
            None => {
                bail!(ErrorKind::MissingAccount);
            }
        };
        if let Some(rule_val) = toml.get("rule") {
            if let Some(rule_val) = rule_val.as_table() {
                for (name, table) in rule_val.iter() {
                    match Rule::from_toml(name.to_string(), table) {
                        Ok(rule) => rules.push(rule),
                        Err(e) => println!("error for rule {}: {}", name, e),
                    }
                }
            }
        }
        Ok(Config {
            account: account,
            rules: rules,
        })
    }

    /// Read file and try to return a Config if there is no error
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut file = File::open(path.as_ref())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let toml = ::toml::from_str::<Value>(&buf)?;
        Config::from_toml(toml)
    }
}
