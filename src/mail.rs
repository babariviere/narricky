use error::*;
use mailparse::*;

/// Structure representing a mail address
#[derive(Clone, Debug, PartialEq)]
pub struct MailAddress {
    pub name: String,
    pub address: String,
}

impl MailAddress {
    /// Create a new mail address
    pub fn new(name: String, address: String) -> MailAddress {
        MailAddress {
            name: name,
            address: address,
        }
    }
    /// Parse address from a value obtained by imap
    pub fn parse_imap<S: AsRef<str>>(value: S) -> MailAddress {
        let mut value = value.as_ref().to_string();
        let idx = value.find('<').unwrap_or(value.len());
        let name = value.drain(..idx).collect::<String>();
        if value.len() > 0 {
            let idx = value.find('>').unwrap_or(value.len());
            let address = value.drain(1..idx).collect();
            MailAddress::new(name.trim().to_string(), address)
        } else {
            MailAddress::new(String::new(), name.trim().to_string())
        }
    }
}

/// Structure representing a mail
#[derive(Debug)]
pub struct Mail {
    pub from: Vec<MailAddress>,
    pub to: Vec<MailAddress>,
    pub cc: Vec<MailAddress>,
    pub subject: String,
    pub content: String,
}

impl Mail {
    /// Parse mail from fetch result
    pub fn parse_fetched(fetched: Vec<String>) -> Result<Mail> {
        let fetched = fetched.into_iter().map(|s| s).collect::<String>();
        let parsed = parse_mail(fetched.as_bytes())?;
        let from = parsed
            .headers
            .get_all_values("From")?
            .join(", ")
            .split(", ")
            .map(|s| MailAddress::parse_imap(s))
            .collect();
        let to = parsed
            .headers
            .get_all_values("To")?
            .join(", ")
            .split(", ")
            .map(|s| MailAddress::parse_imap(s))
            .collect();
        let cc = parsed
            .headers
            .get_all_values("Cc")?
            .join(", ")
            .split(", ")
            .map(|s| MailAddress::parse_imap(s))
            .collect();
        let subject = parsed
            .headers
            .get_first_value("Subject")?
            .unwrap_or_default();
        let content = parsed.get_body()?;
        Ok(Mail {
            from: from,
            to: to,
            cc: cc,
            subject: subject,
            content: content,
        })
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn mail_address_parse_imap() {
        let test_val = MailAddress::new("Test".to_owned(), "test@test.com".to_owned());
        assert_eq!(
            MailAddress::parse_imap("Test <test@test.com>"),
            test_val,
            "error with parsing"
        );
    }
}
