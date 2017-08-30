use error::*;

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
    pub fn parse_fetched(headers: Vec<String>, text: Vec<String>) -> Result<Mail> {
        let mut from = Vec::new();
        let mut to = Vec::new();
        let mut cc = Vec::new();
        let mut subject = String::new();
        for elem in headers.iter() {
            if elem.starts_with("From: ") && elem.len() >= 6 {
                let mut list = elem[6..]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                from.append(&mut list);
            } else if elem.starts_with("To: ") && elem.len() >= 4 {
                let mut list = elem[4..]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                to.append(&mut list);
            } else if elem.starts_with("Cc: ") && elem.len() >= 4 {
                let mut list = elem[4..]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();
                cc.append(&mut list);
            } else if elem.starts_with("Subject: ") && elem.len() >= 9 {
                subject = elem[9..].trim().to_string();
            }
        }
        let content = text.into_iter().map(|s| s.to_string()).collect();
        Ok(Mail {
            from: from,
            to: to,
            cc: cc,
            subject: subject,
            content: content,
        })
    }
}
