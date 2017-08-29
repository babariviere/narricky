use error::*;

#[derive(Debug, PartialEq)]
enum ConditionChecker {
    Contains,
    Is,
}

impl ConditionChecker {
    /// Parse condition checker
    fn parse<S: AsRef<str>>(checker: S) -> Result<ConditionChecker> {
        let checker = checker.as_ref();
        if checker == "is" {
            Ok(ConditionChecker::Is)
        } else if checker == "contains" {
            Ok(ConditionChecker::Contains)
        } else {
            bail!(ErrorKind::InvalidConditionChecker(checker.to_string()));
        }
    }
}

#[derive(Debug, PartialEq)]
enum ConditionType {
    Sender(ConditionChecker, String),
    Cc(ConditionChecker, String),
    Recipient(ConditionChecker, String),
}

impl ConditionType {
    /// Parse string and return condition
    fn parse<S: AsRef<str>>(condition: S) -> Result<ConditionType> {
        let condition = condition.as_ref();
        let splitted: Vec<&str> = condition.split_whitespace().collect();
        let checker = ConditionChecker::parse(splitted[1])?;
        let len = splitted[0].len() + splitted[1].len();
        if splitted[0] == "sender" {
            Ok(ConditionType::Sender(
                checker,
                condition[len + 2..].to_string(),
            ))
        } else if splitted[0] == "cc" {
            Ok(ConditionType::Cc(checker, condition[len + 2..].to_string()))
        } else if splitted[0] == "recipient" {
            Ok(ConditionType::Recipient(
                checker,
                condition[len + 2..].to_string(),
            ))
        } else {
            bail!(ErrorKind::InvalidCondition(condition.to_string()));
        }
    }
}

pub struct Condition(ConditionType);

impl Condition {
    /// Create new condition
    pub fn new<S: AsRef<str>>(condition: S) -> Result<Condition> {
        let cond = ConditionType::parse(condition)?;
        Ok(Condition(cond))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn condition_sender() {
        assert_eq!(
            ConditionType::parse("sender is hello@world").unwrap(),
            ConditionType::Sender(ConditionChecker::Is, "hello@world".to_string()),
            "fail with sender is"
        );
        assert_eq!(
            ConditionType::parse("sender contains hello").unwrap(),
            ConditionType::Sender(ConditionChecker::Contains, "hello".to_string()),
            "fail with sender contains"
        );
    }

    #[test]
    fn condition_cc() {
        assert_eq!(
            ConditionType::parse("cc is hello@world").unwrap(),
            ConditionType::Cc(ConditionChecker::Is, "hello@world".to_string()),
            "fail with cc is"
        );
        assert_eq!(
            ConditionType::parse("cc contains hello").unwrap(),
            ConditionType::Cc(ConditionChecker::Contains, "hello".to_string()),
            "fail with cc contains"
        );
    }

    #[test]
    fn condition_recipient() {
        assert_eq!(
            ConditionType::parse("recipient is hello@world").unwrap(),
            ConditionType::Recipient(ConditionChecker::Is, "hello@world".to_string()),
            "fail with recipient is"
        );
        assert_eq!(
            ConditionType::parse("recipient contains hello").unwrap(),
            ConditionType::Recipient(ConditionChecker::Contains, "hello".to_string()),
            "fail with recipient contains"
        );
    }
}
