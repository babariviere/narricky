use error::*;
use toml::Value;

/// Data in toml for one rule
#[derive(Deserialize)]
struct RuleData {
    pub description: Option<String>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
    pub exceptions: Vec<String>,
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
