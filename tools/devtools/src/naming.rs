//! Branch names: `ISSUE-0001/FEATURE/short-descriptive-title`.

use crate::conventional::TYPES;

pub struct Name<'a> {
    pub issue: u32,
    pub kind: &'a str,
}

/// Check a branch or pull request name, returning every problem found.
pub fn parse(name: &str) -> Result<Name<'_>, Vec<String>> {
    let parts: Vec<&str> = name.split('/').collect();
    let [issue, kind, title] = parts[..] else {
        return Err(vec![format!(
            "`{name}` must have three parts: ISSUE-XXXX/TYPE/short-descriptive-title"
        )]);
    };

    let mut errors = Vec::new();

    let issue = match issue.strip_prefix("ISSUE-") {
        Some(digits) if digits.len() >= 4 && digits.bytes().all(|b| b.is_ascii_digit()) => {
            match digits.parse::<u32>() {
                Ok(n) if n > 0 => Some(n),
                _ => None,
            }
        }
        _ => None,
    };
    if issue.is_none() {
        errors.push("start with the issue number, zero-padded to 4 digits: `ISSUE-0042`".into());
    }

    if !TYPES.iter().any(|t| t.branch == kind) {
        let kinds: Vec<_> = TYPES.iter().map(|t| t.branch).collect();
        errors.push(format!(
            "unknown change type `{kind}`; use one of: {}",
            kinds.join(", ")
        ));
    }

    let valid_title = !title.is_empty()
        && title.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !title.starts_with('-')
        && !title.ends_with('-')
        && !title.contains("--");
    if !valid_title {
        errors.push(
            "the title must be words separated by single hyphens: `add-session-rotation`".into(),
        );
    }

    match issue {
        Some(issue) if errors.is_empty() => Ok(Name { issue, kind }),
        _ => Err(errors),
    }
}

pub fn format(issue: u32, kind: &str, title: &str) -> String {
    format!("ISSUE-{issue:04}/{kind}/{title}")
}

/// Turn free text into a title: `Add Python adaptor X endpoint` becomes
/// `add-python-adaptor-X-endpoint`. Capitalised words are lowercased, while
/// identifiers such as `X`, `OAuth` or `PKCE` keep their case.
pub fn slugify(text: &str) -> String {
    let words: Vec<String> = text
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| {
            let capitalised = w.len() > 1 && w.chars().skip(1).all(|c| !c.is_ascii_uppercase());
            if capitalised {
                w.to_lowercase()
            } else {
                w.to_owned()
            }
        })
        .collect();
    words.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_the_documented_example() {
        let name = parse("ISSUE-0001/FEATURE/adding-python-adaptor-X-endpoint")
            .ok()
            .unwrap();
        assert_eq!(name.issue, 1);
        assert_eq!(name.kind, "FEATURE");
        assert!(parse("ISSUE-12345/FIX/reject-expired-sessions").is_ok());
    }

    #[test]
    fn rejects_bad_names() {
        for name in [
            "",
            "main",
            "ISSUE-0001/FEATURE",
            "ISSUE-1/FEATURE/add-sessions",
            "ISSUE-0000/FEATURE/add-sessions",
            "issue-0001/FEATURE/add-sessions",
            "ISSUE-0001/feature/add-sessions",
            "ISSUE-0001/FEAT/add-sessions",
            "ISSUE-0001/FEATURE/add_sessions",
            "ISSUE-0001/FEATURE/add--sessions",
            "ISSUE-0001/FEATURE/-add-sessions",
            "ISSUE-0001/FEATURE/add sessions",
            "ISSUE-0001/FEATURE/add/sessions",
        ] {
            assert!(parse(name).is_err(), "{name}");
        }
    }

    #[test]
    fn formats_and_slugifies() {
        assert_eq!(
            format(7, "DOCS", "add-onboarding"),
            "ISSUE-0007/DOCS/add-onboarding"
        );
        assert_eq!(
            slugify("Adding Python adaptor X endpoint!"),
            "adding-python-adaptor-X-endpoint"
        );
        assert_eq!(slugify("  fix  OAuth   state "), "fix-OAuth-state");
    }
}
