use error::*;
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
