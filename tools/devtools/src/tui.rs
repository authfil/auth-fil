//! Interactive prompts for creating branches and commits that follow the
//! repository's conventions.

use std::collections::BTreeMap;
use std::fmt;

use inquire::error::InquireError;
use inquire::validator::Validation;
use inquire::{Confirm, CustomType, Editor, Select, Text};

use crate::conventional::{self, CommitType, MAX_HEADER_LEN, Mode, SCOPES, Scope, TYPES};
use crate::{git, naming};

/// Commits touching more lines than this prompt the author to consider splitting.
const LARGE_COMMIT_LINES: usize = 400;

/// Issue numbers are appended to this to link the `Refs:` footer.
const ISSUES_URL: &str = "https://github.com/authloom/auth-loom/issues";

pub fn branch() -> Result<(), String> {
    let issue = CustomType::<u32>::new("Issue number:")
        .with_help_message("Every branch starts from an issue. Open one first if needed.")
        .with_validator(|n: &u32| {
            Ok(if *n > 0 {
                Validation::Valid
            } else {
                Validation::Invalid("issue numbers start at 1".into())
            })
        })
        .prompt()
        .map_err(cancelled)?;

    let kind = Select::new("Type of change:", TYPES.iter().map(BranchType).collect())
        .with_page_size(TYPES.len())
        .prompt()
        .map_err(cancelled)?;

    let title = Text::new("Short descriptive title:")
        .with_help_message(
            "A few words; spaces become hyphens. E.g. `add python adaptor X endpoint`",
        )
        .with_validator(|input: &str| {
            Ok(if naming::slugify(input).is_empty() {
                Validation::Invalid("enter at least one word".into())
            } else {
                Validation::Valid
            })
        })
        .prompt()
        .map_err(cancelled)?;

    let name = naming::format(issue, kind.0.branch, &naming::slugify(&title));
    naming::parse(&name).map_err(|errors| errors.join("; "))?;

    if confirm(&format!("Create and switch to {name}?"), true)? {
        git::switch_create(&name)?;
        println!("Switched to {name}. Open your pull request with this same name as its title.");
    }
    Ok(())
}

pub fn commit() -> Result<(), String> {
    let staged = git::staged()?;
    if staged.is_empty() {
        return Err(
            "nothing is staged. Stage one logical change first, e.g. with `git add -p`.".into(),
        );
    }

    let mut by_scope: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for file in &staged {
        by_scope
            .entry(conventional::scope_for_path(&file.path))
            .or_default()
            .push(&file.path);
    }
    let total_lines: usize = staged.iter().map(|f| f.lines).sum();

    println!(
        "Staged: {} file(s), {total_lines} line(s) changed",
        staged.len()
    );
    for (scope, paths) in &by_scope {
        println!("  {scope}");
        for path in paths {
            println!("    {path}");
        }
    }
    println!();

    if !check_atomic(by_scope.len(), total_lines)? {
        return Ok(());
    }

    let branch = git::current_branch();
    let branch_name = branch.as_deref().and_then(|b| naming::parse(b).ok());

    let type_cursor = branch_name
        .as_ref()
        .and_then(|n| TYPES.iter().position(|t| t.branch == n.kind))
        .unwrap_or(0);
    let kind: &CommitType = Select::new("Type:", TYPES.iter().collect())
        .with_starting_cursor(type_cursor)
        .with_page_size(TYPES.len())
        .prompt()
        .map_err(cancelled)?;

    // Scopes this commit touches come first, so the usual answer is Enter.
    let mut scope_options: Vec<ScopeOption> = by_scope
        .keys()
        .filter_map(|name| SCOPES.iter().find(|s| s.name == *name))
        .map(ScopeOption::Some)
        .collect();
    scope_options.push(ScopeOption::None);
    scope_options.extend(
        SCOPES
            .iter()
            .filter(|s| !by_scope.contains_key(s.name))
            .map(ScopeOption::Some),
    );
    let scope = Select::new("Scope:", scope_options)
        .with_help_message("Areas this commit touches are listed first")
        .with_page_size(SCOPES.len() + 1)
        .prompt()
        .map_err(cancelled)?;

    let breaking = confirm("Is this a breaking change for users of the library?", false)?;

    let scope_part = match scope {
        ScopeOption::Some(s) => format!("({})", s.name),
        ScopeOption::None => String::new(),
    };
    let prefix = format!(
        "{}{scope_part}{}: ",
        kind.name,
        if breaking { "!" } else { "" }
    );
    let budget = MAX_HEADER_LEN.saturating_sub(prefix.chars().count());

    let question = format!("Description: {prefix}");
    let help = format!(
        "What the commit does, in the imperative: `add session rotation`. Up to {budget} characters."
    );
    let description = Text::new(&question)
        .with_help_message(&help)
        .with_validator(move |input: &str| {
            let mut errors = conventional::lint_description(input);
            let len = input.chars().count();
            if len > budget {
                errors.push(format!("{len} characters; the limit here is {budget}"));
            }
            Ok(match errors.into_iter().next() {
                Some(error) => Validation::Invalid(error.into()),
                None => Validation::Valid,
            })
        })
        .prompt()
        .map_err(cancelled)?;

    let breaking_note = if breaking {
        Some(
            Text::new("What breaks, and how should users migrate?")
                .with_validator(|input: &str| {
                    Ok(if input.trim().is_empty() {
                        Validation::Invalid("describe the breaking change".into())
                    } else {
                        Validation::Valid
                    })
                })
                .prompt()
                .map_err(cancelled)?,
        )
    } else {
        None
    };

    let body = if confirm("Add a body explaining why this change is needed?", false)? {
        Some(
            Editor::new("Body:")
                .with_help_message("Explain why, not what: the diff already shows what changed.")
                .prompt()
                .map_err(cancelled)?,
        )
    } else {
        None
    };

    let issue = match branch_name.as_ref().map(|n| n.issue) {
        Some(n) if confirm(&format!("Reference issue #{n}?"), true)? => Some(n),
        Some(_) => None,
        None => ask_issue()?,
    };

    let message = build_message(
        &prefix,
        &description,
        body.as_deref(),
        breaking_note.as_deref(),
        issue,
    );
    conventional::lint(&message, Mode::Local).map_err(|errors| errors.join("; "))?;

    println!("\n{}\n", indent(&message));
    if confirm("Commit with this message?", true)? {
        git::commit(&message)?;
    }
    Ok(())
}

/// Warn about commits that look like more than one logical change. Returns
/// whether to continue.
fn check_atomic(areas: usize, lines: usize) -> Result<bool, String> {
    if areas > 1 {
        println!(
            "These changes span {areas} areas. An atomic commit makes one logical change, so\n\
             unrelated changes belong in separate commits. Unstage files with\n\
             `git restore --staged <path>`, or stage hunks selectively with `git add -p`.\n"
        );
        if !confirm("Is this one logical change?", false)? {
            return Ok(false);
        }
    }
    if lines > LARGE_COMMIT_LINES {
        println!(
            "This commit changes {lines} lines. Large commits are hard to review; consider splitting it.\n"
        );
        if !confirm("Commit it as one change?", false)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn ask_issue() -> Result<Option<u32>, String> {
    let answer = Text::new("Issue number (leave blank for none):")
        .with_validator(|input: &str| {
            let input = input.trim().trim_start_matches('#');
            Ok(match input.parse::<u32>() {
                _ if input.is_empty() => Validation::Valid,
                Ok(n) if n > 0 => Validation::Valid,
                _ => Validation::Invalid("enter a number such as 42".into()),
            })
        })
        .prompt()
        .map_err(cancelled)?;
    Ok(answer.trim().trim_start_matches('#').parse().ok())
}

fn build_message(
    prefix: &str,
    description: &str,
    body: Option<&str>,
    breaking_note: Option<&str>,
    issue: Option<u32>,
) -> String {
    let mut message = format!("{prefix}{description}");
    if let Some(body) = body.map(str::trim).filter(|b| !b.is_empty()) {
        message.push_str("\n\n");
        message.push_str(body);
    }
    let mut footers = Vec::new();
    if let Some(note) = breaking_note {
        footers.push(format!("BREAKING CHANGE: {}", note.trim()));
    }
    if let Some(issue) = issue {
        footers.push(format!("Refs: {ISSUES_URL}/{issue}"));
    }
    if !footers.is_empty() {
        message.push_str("\n\n");
        message.push_str(&footers.join("\n"));
    }
    message
}

fn indent(text: &str) -> String {
    text.lines()
        .map(|line| format!("    {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn confirm(question: &str, default: bool) -> Result<bool, String> {
    Confirm::new(question)
        .with_default(default)
        .prompt()
        .map_err(cancelled)
}

fn cancelled(error: InquireError) -> String {
    match error {
        InquireError::OperationCanceled | InquireError::OperationInterrupted => "cancelled".into(),
        other => other.to_string(),
    }
}

struct BranchType(&'static CommitType);

impl fmt::Display for BranchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<9} {}", self.0.branch, self.0.description)
    }
}

enum ScopeOption {
    Some(&'static Scope),
    None,
}

impl fmt::Display for ScopeOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScopeOption::Some(scope) => scope.fmt(f),
            ScopeOption::None => {
                write!(f, "{:<12} No scope: the change is cross-cutting", "(none)")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_messages_that_pass_the_linter() {
        let message = build_message(
            "feat(core)!: ",
            "rename verify to verify_password",
            Some("The old name was ambiguous.\n"),
            Some("`verify` is now `verify_password`."),
            Some(12),
        );
        assert_eq!(
            message,
            "feat(core)!: rename verify to verify_password\n\n\
             The old name was ambiguous.\n\n\
             BREAKING CHANGE: `verify` is now `verify_password`.\n\
             Refs: https://github.com/authloom/auth-loom/issues/12"
        );
        assert_eq!(conventional::lint(&message, Mode::Strict), Ok(()));
    }

    #[test]
    fn builds_header_only_messages() {
        assert_eq!(
            build_message("docs: ", "fix typo", Some("  "), None, None),
            "docs: fix typo"
        );
    }
}
