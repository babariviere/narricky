use connection::Connection;
use error::*;

/// Describe action type
#[derive(Debug, PartialEq)]
enum ActionType {
    NoMoreRules,
    CopyTo(String),
    MoveTo(String),
    Delete,
    PermanentDelete,
    ForwardTo(String),
    ReplyWith(String),
    SetFlag(String),
    RemoveFlag(String),
    ClearFlags,
    MarkAsImportant,
    MarkAsRead,
}

impl ActionType {
    /// Parse action
    fn parse<S: AsRef<str>>(action: S) -> Result<ActionType> {
        let action = action.as_ref().trim();
        if action == "no more rules" {
            Ok(ActionType::NoMoreRules)
        } else if action.starts_with("copy to ") {
            Ok(ActionType::CopyTo(action[8..].to_string()))
        } else if action.starts_with("move to ") {
            Ok(ActionType::MoveTo(action[8..].to_string()))
        } else if action == "delete" {
            Ok(ActionType::Delete)
        } else if action == "permanent delete" {
            Ok(ActionType::PermanentDelete)
        } else if action.starts_with("forward to ") {
            Ok(ActionType::ForwardTo(action[11..].to_string()))
        } else if action.starts_with("reply with ") {
            Ok(ActionType::ReplyWith(action[11..].to_string()))
        } else if action.starts_with("set flag ") {
            Ok(ActionType::SetFlag(action[9..].to_string()))
        } else if action.starts_with("remove flag ") {
            Ok(ActionType::RemoveFlag(action[12..].to_string()))
        } else if action == "clear flags" {
            Ok(ActionType::ClearFlags)
        } else if action == "mark as important" {
            Ok(ActionType::MarkAsImportant)
        } else if action == "mark as read" {
            Ok(ActionType::MarkAsRead)
        } else {
            bail!(ErrorKind::InvalidAction(action.to_string()));
        }
    }

    /// Apply action to mail
    fn apply(&self, connection: &mut Connection, idx: usize) -> Result<()> {
        match self {
            &ActionType::CopyTo(ref folder) => {
                connection.create(folder)?;
                connection.copy(&idx.to_string(), folder)?;
                Ok(())
            }
            &ActionType::MoveTo(ref folder) => {
                connection.create(folder)?;
                connection.copy(&idx.to_string(), folder)?;
                connection.store(&idx.to_string(), "+flags (\\deleted)")?;
                connection.expunge()?;
                Ok(())
            }
            &ActionType::Delete => {
                connection.store(&idx.to_string(), "+flags (\\deleted)")?;
                Ok(())
            }
            &ActionType::PermanentDelete => {
                connection.store(&idx.to_string(), "+flags (\\deleted)")?;
                connection.expunge()?;
                Ok(())
            }
            &ActionType::SetFlag(ref flag) => {
                connection.store(
                    &idx.to_string(),
                    &format!("+flags ({})", flag),
                )?;
                Ok(())
            }
            &ActionType::RemoveFlag(ref flag) => {
                connection.store(
                    &idx.to_string(),
                    &format!("-flags ({})", flag),
                )?;
                Ok(())
            }
            &ActionType::ClearFlags => {
                // TODO connection.store(idx.to_string(), &format!("+flags ({})", flag));
                Ok(())
            }
            &ActionType::MarkAsImportant => connection.copy(&idx.to_string(), "Important"),
            &ActionType::MarkAsRead => {
                connection.store(&idx.to_string(), "+flags (\\seen)")?;
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}

/// Action structure to apply
#[derive(Debug, PartialEq)]
pub struct Action(ActionType);

impl Action {
    /// Parse an action and return it
    pub fn new<S: AsRef<str>>(action: S) -> Result<Action> {
        let act = ActionType::parse(action)?;
        Ok(Action(act))
    }

    /// Apply action to mail
    pub fn apply(&self, connection: &mut Connection, idx: usize) -> Result<()> {
        self.0.apply(connection, idx)
    }

    /// Check if actions remove mail
    pub fn remove_mail(&self) -> bool {
        match self.0 {
            ActionType::MoveTo(_) |
            ActionType::PermanentDelete => true,
            _ => false,
        }
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
        assert_eq!(
            test_val,
            ActionType::parse(&format!("copy to {}", dest)).unwrap(),
            "failed on copy {}",
            dest
        )
    }

    #[test]
    fn action_copy() {
        action_copy_helper("INBOX");
        action_copy_helper("TRASH");
    }

    fn action_move_helper(dest: &str) {
        let test_val = ActionType::MoveTo(dest.to_string());
        assert_eq!(
            test_val,
            ActionType::parse(&format!("move to {}", dest)).unwrap(),
            "failed on move {}",
            dest
        )
    }

    #[test]
    fn action_move() {
        action_move_helper("INBOX");
        action_move_helper("TRASH");
    }

    #[test]
    fn action_delete() {
        assert_eq!(
            ActionType::parse("delete").unwrap(),
            ActionType::Delete,
            "fail with action delete"
        );
        assert_eq!(
            ActionType::parse("  delete   ").unwrap(),
            ActionType::Delete,
            "fail with action delete"
        );
    }

    #[test]
    fn action_permanent_delete() {
        assert_eq!(
            ActionType::parse("permanent delete").unwrap(),
            ActionType::PermanentDelete,
            "fail with action permanent delete"
        );
    }

    fn action_forward_helper(dest: &str) {
        let test_val = ActionType::ForwardTo(dest.to_string());
        assert_eq!(
            test_val,
            ActionType::parse(&format!("forward to {}", dest)).unwrap(),
            "failed on forward {}",
            dest
        )
    }

    #[test]
    fn action_forward() {
        action_forward_helper("test@test.com");
    }

    fn action_reply_helper(dest: &str) {
        let test_val = ActionType::ReplyWith(dest.to_string());
        assert_eq!(
            test_val,
            ActionType::parse(&format!("reply with {}", dest)).unwrap(),
            "failed on reply {}",
            dest
        )
    }

    #[test]
    fn action_reply() {
        action_reply_helper("Hello world");
        action_reply_helper("Testing reply with");
    }

    fn action_set_flag_helper(dest: &str) {
        let test_val = ActionType::SetFlag(dest.to_string());
        assert_eq!(
            test_val,
            ActionType::parse(&format!("set flag {}", dest)).unwrap(),
            "failed on flag {}",
            dest
        )
    }

    #[test]
    fn action_set_flag() {
        action_set_flag_helper("\\Seen");
        action_set_flag_helper("\\Important");
    }

    fn action_remove_flag_helper(dest: &str) {
        let test_val = ActionType::RemoveFlag(dest.to_string());
        assert_eq!(
            test_val,
            ActionType::parse(&format!("remove flag {}", dest)).unwrap(),
            "failed on flag {}",
            dest
        )
    }

    #[test]
    fn action_remove_flag() {
        action_remove_flag_helper("\\Seen");
        action_remove_flag_helper("\\Important");
    }

    #[test]
    fn action_clear_flag() {
        assert_eq!(
            ActionType::parse("clear flags").unwrap(),
            ActionType::ClearFlags,
            "fail with clear flags"
        );
    }

    #[test]
    fn action_mark_as_important() {
        assert_eq!(
            ActionType::parse("mark as important").unwrap(),
            ActionType::MarkAsImportant,
            "fail with mark as important"
        );
    }

    #[test]
    fn action_mark_as_read() {
        assert_eq!(
            ActionType::parse("mark as read").unwrap(),
            ActionType::MarkAsRead,
            "fail with mark as read"
        );
    }
}
