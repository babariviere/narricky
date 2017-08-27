use error::*;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub domain: String,
    pub port: u16,
    pub secure: bool,
}

impl Account {
    pub fn from_toml(toml: &Value) -> Result<Account> {
        toml.clone().try_into().map_err(|e| e.into())
    }
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub description: Option<String>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
    pub exceptions: Vec<String>,
}

#[derive(Deserialize)]
struct RuleData {
    pub description: Option<String>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
    pub exceptions: Vec<String>,
}

impl Rule {
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

pub struct Config {
    pub account: Account,
    pub rules: Vec<Rule>,
}

impl Config {
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
}
