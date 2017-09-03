mod action;
mod condition;

pub use self::action::*;
pub use self::condition::*;

use error::*;
use toml::Value;

/// Data in toml for one rule
#[derive(Deserialize)]
struct RuleData {
    pub description: Option<String>,
    pub any: Option<bool>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
    pub exceptions: Vec<String>,
}

/// Rule for mail account
#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub description: Option<String>,
    pub any: bool,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub exceptions: Vec<Condition>,
}

impl Rule {
    /// Convert toml table into Rule
    pub fn from_toml(name: String, toml: &Value) -> Result<Rule> {
        let data: RuleData = toml.clone().try_into::<RuleData>()?;
        let mut conditions = Vec::new();
        for condition in data.conditions {
            conditions.push(Condition::new(condition)?);
        }
        let mut actions = Vec::new();
        for action in data.actions {
            actions.push(Action::new(action)?);
        }
        let mut exceptions = Vec::new();
        for exception in data.exceptions {
            exceptions.push(Condition::new(exception)?);
        }
        Ok(Rule {
            name: name,
            description: data.description,
            any: data.any.unwrap_or(false),
            conditions: conditions,
            actions: actions,
            exceptions: exceptions,
        })
    }
}
