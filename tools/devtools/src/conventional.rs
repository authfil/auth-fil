//! Conventional Commits rules for this repository.
//!
//! See <https://www.conventionalcommits.org/en/v1.0.0/>. On top of the spec,
//! types and scopes are restricted to the lists below so history stays
//! consistent and filterable.

use std::fmt;

pub const MAX_HEADER_LEN: usize = 72;

pub struct CommitType {
    pub name: &'static str,
    /// The matching change type in branch and pull request names.
    pub branch: &'static str,
    pub description: &'static str,
}

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<9} {}", self.name, self.description)
    }
}

pub const TYPES: &[CommitType] = &[
    CommitType {
        name: "feat",
        branch: "FEATURE",
        description: "A new feature",
    },
    CommitType {
        name: "fix",
        branch: "FIX",
        description: "A bug fix",
    },
    CommitType {
        name: "docs",
        branch: "DOCS",
        description: "Documentation only",
    },
    CommitType {
        name: "style",
        branch: "STYLE",
        description: "Formatting, no change in behaviour",
    },
    CommitType {
        name: "refactor",
        branch: "REFACTOR",
        description: "Restructuring, no change in behaviour",
    },
    CommitType {
        name: "perf",
        branch: "PERF",
        description: "A performance improvement",
    },
    CommitType {
        name: "test",
        branch: "TEST",
        description: "Adding or correcting tests",
    },
    CommitType {
        name: "build",
        branch: "BUILD",
        description: "Build system or dependencies",
    },
    CommitType {
        name: "ci",
        branch: "CI",
        description: "CI configuration",
    },
    CommitType {
        name: "chore",
        branch: "CHORE",
        description: "Maintenance that fits nowhere else",
    },
    CommitType {
        name: "revert",
        branch: "REVERT",
        description: "Reverts a previous commit",
    },
];

pub struct Scope {
    pub name: &'static str,
    pub description: &'static str,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<12} {}", self.name, self.description)
    }
}

pub const SCOPES: &[Scope] = &[
    Scope {
        name: "core",
        description: "crates/authloom-core",
    },
    Scope {
        name: "crypto",
        description: "crates/authloom-crypto",
    },
    Scope {
        name: "node",
        description: "adapters/node",
    },
    Scope {
        name: "python",
        description: "adapters/python",
    },
    Scope {
        name: "go",
        description: "adapters/go",
    },
    Scope {
        name: "java",
        description: "adapters/java",
    },
    Scope {
        name: "rust",
        description: "adapters/rust",
    },
    Scope {
        name: "conformance",
        description: "conformance/",
    },
    Scope {
        name: "docs",
        description: "docs/ and the repo's Markdown files",
    },
    Scope {
        name: "devtools",
        description: "tools/devtools and git hooks",
    },
    Scope {
        name: "docker",
        description: "Dockerfile and compose",
    },
    Scope {
        name: "ci",
        description: ".github/",
    },
    Scope {
        name: "deps",
        description: "Cargo.lock and dependency policy",
    },
    Scope {
        name: "repo",
        description: "Workspace-wide files",
    },
];

/// The scope a changed file belongs to, used to suggest a scope and to spot
/// commits that mix unrelated areas.
pub fn scope_for_path(path: &str) -> &'static str {
    let prefixes = [
        ("crates/authloom-core/", "core"),
        ("crates/authloom-crypto/", "crypto"),
        ("adapters/node/", "node"),
        ("adapters/python/", "python"),
        ("adapters/go/", "go"),
        ("adapters/java/", "java"),
        ("adapters/rust/", "rust"),
        ("conformance/", "conformance"),
        ("docs/", "docs"),
        ("tools/devtools/", "devtools"),
        (".githooks/", "devtools"),
        (".github/", "ci"),
    ];
    if let Some((_, scope)) = prefixes.iter().find(|(prefix, _)| path.starts_with(prefix)) {
        return scope;
    }
    match path {
        "Dockerfile" | "compose.yaml" | ".dockerignore" => "docker",
        "Cargo.lock" | "deny.toml" => "deps",
        p if !p.contains('/') && p.ends_with(".md") => "docs",
        _ => "repo",
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Local commits: `fixup!` and `squash!` commits are allowed.
    Local,
    /// Commits about to be merged: every commit must be final.
    Strict,
}

/// Remove what git strips before committing: comment lines and everything
/// below the scissors line of `git commit --verbose`.
pub fn clean(raw: &str) -> String {
    let lines: Vec<&str> = raw
        .lines()
        .take_while(|line| !line.starts_with("# ------------------------ >8"))
        .filter(|line| !line.starts_with('#'))
        .map(str::trim_end)
        .collect();
    lines.join("\n").trim().to_owned()
}

/// Check a commit message, returning every problem found.
pub fn lint(message: &str, mode: Mode) -> Result<(), Vec<String>> {
    let message = clean(message);
    let mut lines = message.lines();
    let Some(header) = lines.next() else {
        return Err(vec!["the commit message is empty".into()]);
    };

    if header.starts_with("Merge ") {
        return Ok(());
    }
    if ["fixup! ", "squash! ", "amend! "]
        .iter()
        .any(|p| header.starts_with(p))
    {
        return match mode {
            Mode::Local => Ok(()),
            Mode::Strict => Err(vec![
                "fixup commits must be squashed before merging (git rebase -i --autosquash)".into(),
            ]),
        };
    }

    let mut errors = lint_header(header);
    if lines.next().is_some_and(|line| !line.is_empty()) {
        errors.push("leave a blank line between the header and the body".into());
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Check a pull request title. It becomes the squash commit header on
/// `main`, so it follows the same header rules as a commit.
pub fn lint_title(title: &str) -> Result<(), Vec<String>> {
    let errors = lint_header(title);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn lint_header(header: &str) -> Vec<String> {
    let mut errors = Vec::new();

    let Some((prefix, description)) = header.split_once(": ") else {
        return vec!["the header must look like `type(scope): description`".into()];
    };

    let (prefix, _breaking) = match prefix.strip_suffix('!') {
        Some(p) => (p, true),
        None => (prefix, false),
    };
    let (kind, scope) = match prefix.split_once('(') {
        Some((kind, rest)) => match rest.strip_suffix(')') {
            Some(scope) => (kind, Some(scope)),
            None => return vec!["the scope must be closed with `)`".into()],
        },
        None => (prefix, None),
    };

    if !TYPES.iter().any(|t| t.name == kind) {
        errors.push(format!(
            "unknown type `{kind}`; use one of: {}",
            names(TYPES.iter().map(|t| t.name))
        ));
    }
    if let Some(scope) = scope
        && !SCOPES.iter().any(|s| s.name == scope)
    {
        errors.push(format!(
            "unknown scope `{scope}`; use one of: {}, or leave it out",
            names(SCOPES.iter().map(|s| s.name))
        ));
    }

    errors.extend(lint_description(description));

    let len = header.chars().count();
    if len > MAX_HEADER_LEN {
        errors.push(format!(
            "the header is {len} characters; keep it to {MAX_HEADER_LEN} or fewer"
        ));
    }
    errors
}

/// Rules for the text after `type(scope): `.
pub fn lint_description(description: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let mut chars = description.chars();
    match (chars.next(), chars.next()) {
        (None, _) => errors.push("the description is empty".into()),
        (Some(c), _) if c.is_whitespace() => {
            errors.push("the description must not start with a space".into())
        }
        // Allow acronyms such as `OAuth` or `PKCE`, but not `Add`.
        (Some(first), Some(second)) if first.is_uppercase() && second.is_lowercase() => {
            errors.push("start the description with a lowercase letter (`add`, not `Add`)".into())
        }
        _ => {}
    }
    if description.ends_with('.') {
        errors.push("don't end the description with a full stop".into());
    }
    errors
}

fn names<'a>(items: impl Iterator<Item = &'a str>) -> String {
    items.collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(message: &str) {
        assert_eq!(lint(message, Mode::Strict), Ok(()), "{message}");
    }

    fn err(message: &str, mode: Mode) {
        assert!(lint(message, mode).is_err(), "{message}");
    }

    #[test]
    fn accepts_valid_headers() {
        ok("feat: add session rotation");
        ok("fix(core): reject expired sessions");
        ok("feat(python)!: rename verify to verify_password");
        ok("docs: explain PKCE in the OAuth guide");
        ok("feat(core): add OAuth state parameter");
    }

    #[test]
    fn pull_request_titles_follow_the_header_rules() {
        assert_eq!(lint_title("feat(core): add session rotation"), Ok(()));
        assert!(lint_title("ISSUE-0001/FEATURE/add-session-rotation").is_err());
        assert!(lint_title("feat(core): Add session rotation").is_err());
    }

    #[test]
    fn accepts_body_and_footers() {
        ok(
            "fix(crypto): compare tokens in constant time\n\nTiming leaked the prefix.\n\nRefs: #12",
        );
    }

    #[test]
    fn rejects_bad_headers() {
        for message in [
            "",
            "add session rotation",
            "feature: add session rotation",
            "feat(sessions): add session rotation",
            "feat(core: add session rotation",
            "feat(core): Add session rotation",
            "feat(core): add session rotation.",
            "feat(core):  add session rotation",
            "feat(core):",
        ] {
            err(message, Mode::Local);
        }
        err(&format!("feat: {}", "a".repeat(70)), Mode::Local);
    }

    #[test]
    fn requires_blank_line_before_body() {
        err("feat: add sessions\nbody straight after", Mode::Local);
    }

    #[test]
    fn fixups_allowed_locally_only() {
        assert_eq!(lint("fixup! feat: add sessions", Mode::Local), Ok(()));
        err("fixup! feat: add sessions", Mode::Strict);
    }

    #[test]
    fn allows_merge_commits() {
        ok("Merge branch 'main' into ISSUE-0001/FEATURE/sessions");
    }

    #[test]
    fn ignores_comments_and_verbose_diff() {
        ok("feat: add sessions\n# Please enter the commit message\n");
        ok(
            "feat: add sessions\n# ------------------------ >8 ------------------------\ndiff --git a b",
        );
    }

    #[test]
    fn maps_paths_to_scopes() {
        assert_eq!(scope_for_path("crates/authloom-core/src/lib.rs"), "core");
        assert_eq!(scope_for_path("adapters/python/src/lib.rs"), "python");
        assert_eq!(scope_for_path("README.md"), "docs");
        assert_eq!(scope_for_path("Cargo.lock"), "deps");
        assert_eq!(scope_for_path("Makefile"), "repo");
        assert_eq!(scope_for_path(".github/workflows/docs.yml"), "ci");
    }

    #[test]
    fn every_scope_from_paths_is_known() {
        for path in [
            "x/y",
            "Dockerfile",
            "Cargo.lock",
            "a.md",
            ".githooks/commit-msg",
        ] {
            let scope = scope_for_path(path);
            assert!(SCOPES.iter().any(|s| s.name == scope), "{scope}");
        }
    }
}
