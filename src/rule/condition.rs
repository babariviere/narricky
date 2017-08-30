use error::*;
use mail::Mail;

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

    /// Check if condition is true
    fn check<S: AsRef<str>, T: AsRef<str>>(&self, checker: S, to_check: T) -> bool {
        let (checker, to_check) = (checker.as_ref(), to_check.as_ref());
        match *self {
            ConditionChecker::Is => checker == to_check,
            ConditionChecker::Contains => to_check.contains(checker),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ConditionType {
    Sender(ConditionChecker, String),
    Cc(ConditionChecker, String),
    Recipient(ConditionChecker, String),
    Subject(ConditionChecker, String),
    Content(ConditionChecker, String),
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
        } else if splitted[0] == "subject" {
            Ok(ConditionType::Subject(
                checker,
                condition[len + 2..].to_string(),
            ))
        } else if splitted[0] == "content" {
            Ok(ConditionType::Content(
                checker,
                condition[len + 2..].to_string(),
            ))
        } else {
            bail!(ErrorKind::InvalidCondition(condition.to_string()));
        }
    }

    /// Check if condition is true
    fn check(&self, mail: &Mail) -> bool {
        match self {
            &ConditionType::Sender(ref c, ref checker) => {
                c.check(
                    &checker,
                    mail.from.iter().map(|s| s.to_string()).collect::<String>(),
                )
            }
            &ConditionType::Cc(ref c, ref checker) => {
                c.check(
                    &checker,
                    mail.cc.iter().map(|s| s.to_string()).collect::<String>(),
                )
            }
            &ConditionType::Recipient(ref c, ref checker) => {
                c.check(
                    &checker,
                    mail.to.iter().map(|s| s.to_string()).collect::<String>(),
                )
            }
            &ConditionType::Subject(ref c, ref checker) => c.check(&checker, mail.subject.trim()),
            &ConditionType::Content(ref c, ref checker) => c.check(&checker, mail.content.trim()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Condition(ConditionType);

impl Condition {
    /// Create new condition
    pub fn new<S: AsRef<str>>(condition: S) -> Result<Condition> {
        let cond = ConditionType::parse(condition)?;
        Ok(Condition(cond))
    }

    /// Check if mail respects condition
    pub fn check(&self, mail: &Mail) -> bool {
        self.0.check(mail)
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

    #[test]
    fn condition_subject() {
        assert_eq!(
            ConditionType::parse("subject is hello world").unwrap(),
            ConditionType::Subject(ConditionChecker::Is, "hello world".to_string()),
            "fail with subject is"
        );
        assert_eq!(
            ConditionType::parse("subject contains hello").unwrap(),
            ConditionType::Subject(ConditionChecker::Contains, "hello".to_string()),
            "fail with subject contains"
        );
    }

    #[test]
    fn condition_content() {
        assert_eq!(
            ConditionType::parse("content is hello world").unwrap(),
            ConditionType::Content(ConditionChecker::Is, "hello world".to_string()),
            "fail with content is"
        );
        assert_eq!(
            ConditionType::parse("content contains hello").unwrap(),
            ConditionType::Content(ConditionChecker::Contains, "hello".to_string()),
            "fail with content contains"
        );
    }

    #[test]
    fn condition_check_subject() {
        let mail = Mail::parse_fetched(
            vec![
                "Subject: Hehe\r\n",
                "From: Inconnito <inconnito@superrito.com>\r\n",
                "To: hineen1975@superrito.com\r\n",
            ].iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["Hello world\r\n", "This is some text\r\n", "Wow"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ).unwrap();
        let c = Condition::new("subject is Hehe").unwrap();
        assert!(c.check(&mail));
        let c = Condition::new("subject contains Hehe").unwrap();
        assert!(c.check(&mail));
    }

    #[test]
    fn condition_check_sender() {
        let mail = Mail::parse_fetched(
            vec![
                "Subject: Hehe\r\n",
                "From: Inconnito <inconnito@superrito.com>\r\n",
                "To: hineen1975@superrito.com\r\n",
            ].iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["Hello world\r\n", "This is some text\r\n", "Wow"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ).unwrap();
        let c = Condition::new("sender contains Inconnito").unwrap();
        assert!(c.check(&mail));
    }

    #[test]
    fn condition_check_recipient() {
        let mail = Mail::parse_fetched(
            vec![
                "Subject: Hehe\r\n",
                "From: Inconnito <inconnito@superrito.com>\r\n",
                "To: hineen1975@superrito.com\r\n",
            ].iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["Hello world\r\n", "This is some text\r\n", "Wow"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ).unwrap();
        let c = Condition::new("recipient contains hineen").unwrap();
        assert!(c.check(&mail));
        let c = Condition::new("recipient is hineen1975@superrito.com").unwrap();
        assert!(c.check(&mail));
    }

    #[test]
    fn condition_check_cc() {
        let mail = Mail::parse_fetched(
            vec![
                "Subject: Hehe\r\n",
                "From: Inconnito <inconnito@superrito.com>\r\n",
                "To: hineen1975@superrito.com\r\n",
                "Cc: hineen1975@superrito.com\r\n",
            ].iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["Hello world\r\n", "This is some text\r\n", "Wow"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ).unwrap();
        let c = Condition::new("cc contains hineen").unwrap();
        assert!(c.check(&mail));
        let c = Condition::new("cc is hineen1975@superrito.com").unwrap();
        assert!(c.check(&mail));
    }

    #[test]
    fn condition_check_content() {
        let mail = Mail::parse_fetched(
            vec![
                "Subject: Hehe\r\n",
                "From: Inconnito <inconnito@superrito.com>\r\n",
                "To: hineen1975@superrito.com\r\n",
            ].iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["Hello world\r\n", "This is some text\r\n", "Wow"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ).unwrap();
        let c = Condition::new("content is Hello world\r\nThis is some text\r\nWow").unwrap();
        assert!(c.check(&mail));
        let c = Condition::new("content contains This").unwrap();
        assert!(c.check(&mail));
        let c = Condition::new("content contains is").unwrap();
        assert!(c.check(&mail));
    }
}
