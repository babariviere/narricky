use error::*;

/// Structure representing a mail
#[derive(Debug)]
pub struct Mail {
    from: Vec<String>,
    to: Vec<String>,
    cc: Vec<String>,
    subject: String,
    content: String,
}

impl Mail {
    /// Parse mail from fetch result
    pub fn parse_fetched(fetched: Vec<String>) -> Result<Mail> {
        let mut from = Vec::new();
        let mut to = Vec::new();
        let mut cc = Vec::new();
        let mut subject = String::new();
        let mut content = String::new();
        let mut count = 0;
        for elem in fetched.iter() {
            if elem.starts_with("From: ") {
                let mut list = elem[6..]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                from.append(&mut list);
                count += 1;
            } else if elem.starts_with("To: ") {
                let mut list = elem[4..]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                to.append(&mut list);
                count += 1;
            } else if elem.starts_with("Cc: ") {
                let mut list = elem[4..]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                cc.append(&mut list);
                count += 1;
            } else if elem.starts_with("Subject: ") {
                subject = elem[9..].trim().to_string();
                count += 1;
            }
        }
        content = fetched[count..]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        Ok(Mail {
            from: from,
            to: to,
            cc: cc,
            subject: subject,
            content: content,
        })
    }
}
