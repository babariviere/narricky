use error::*;
use mailparse::*;

/// Structure representing a mail
#[derive(Debug)]
pub struct Mail {
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub subject: String,
    pub content: String,
}

impl Mail {
    /// Parse mail from fetch result
    pub fn parse_fetched(fetched: Vec<String>) -> Result<Mail> {
        let fetched = fetched.into_iter().map(|s| s).collect::<String>();
        let parsed = parse_mail(fetched.as_bytes())?;
        let from = parsed.headers.get_all_values("From")?;
        let to = parsed.headers.get_all_values("To")?;
        let cc = parsed.headers.get_all_values("Cc")?;
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
