use error::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value;

/// Mail account
#[derive(Debug, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub domain: String,
    pub port: u16,
    pub secure: bool,
}

impl Account {
    /// Convert toml table into Account
    pub fn from_toml(toml: &Value) -> Result<Account> {
        toml.clone().try_into().map_err(|e| e.into())
    }
}

/// Rule for mail account
#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub description: Option<String>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
    pub exceptions: Vec<String>,
}

/// Data in toml for one rule
#[derive(Deserialize)]
struct RuleData {
    pub description: Option<String>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
    pub exceptions: Vec<String>,
}

impl Rule {
    /// Convert toml table into Rule
    pub fn from_toml(name: String, toml: &Value) -> Result<Rule> {
        let data: RuleData = toml.clone().try_into::<RuleData>()?;
        Ok(Rule {
            name: name,
            description: data.description,
            conditions: data.conditions,
            actions: data.actions,
            exceptions: data.exceptions,
        })
    }
}

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
