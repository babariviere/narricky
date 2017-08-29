use error::*;

/// Describe action type
#[derive(Debug, PartialEq)]
enum ActionType {
    NoMoreRules,
    CopyTo(String),
    MoveTo(String),
    Delete,
    PermanentDelete,
    ForwardItTo(String),
    ReplyWith(String),
    Flag(String),
    ClearFlag,
    MarkAsImportant,
    MarkAsRead,
}

impl ActionType {
    /// Parse action
    fn parse<S: AsRef<str>>(action: S) -> Result<ActionType> {
        let action = action.as_ref();
        if action == "no more rules" {
            return Ok(ActionType::NoMoreRules);
        } else if action.starts_with("copy to ") && action.len() > 9 {
            return Ok(ActionType::CopyTo(action[8..].to_string()));
        } else if action.starts_with("move to ") && action.len() > 9 {
            return Ok(ActionType::MoveTo(action[8..].to_string()));
        }
        Ok(ActionType::NoMoreRules)
    }
}

/// Action structure to apply
pub struct Action(ActionType);

impl Action {
    /// Parse an action and return it
    pub fn new<S: AsRef<str>>(action: S) -> Result<Action> {
        let act = ActionType::parse(action)?;
        Ok(Action(act))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::ActionType;

    #[test]
    fn action_no_rules() {
        assert_eq!(
            ActionType::parse("no more rules").unwrap(),
            ActionType::NoMoreRules,
            "fail with action no more ruels"
        );
    }

    fn action_copy_helper(dest: &str) {
        let test_val = ActionType::CopyTo(dest.to_string());
        assert!(
            test_val == ActionType::parse(&format!("copy to {}", dest)).unwrap(),
            "failed on copy {}",
            dest
        )
    }

    #[test]
    fn action_copy() {
        action_copy_helper("INBOX");
        action_copy_helper("TRASH");
        action_copy_helper("Unknown folder");
    }
}
