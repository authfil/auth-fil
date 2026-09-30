//! Contributor tooling for Authloom.
//!
//! One place for the repository's branch and commit conventions, used by
//! the interactive prompts (`make branch`, `make commit`), the commit-msg
//! git hook, and CI.

// A CLI: printing is its job.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod conventional;
mod git;
mod naming;
mod tui;

use std::process::ExitCode;

use conventional::Mode;

const USAGE: &str = "\
Usage: devtools <command>

Interactive:
  branch               Create a branch named ISSUE-XXXX/TYPE/short-descriptive-title
  commit               Write a Conventional Commit for the staged changes

Checks:
  lint-commit <file>   Check a commit message file (used by the commit-msg hook)
  lint-range <range>   Check every non-merge commit in a range, e.g. origin/main..HEAD
  lint-name <name>     Check a branch name or pull request title

Setup:
  install-hooks        Use the repository's git hooks in .githooks/";

const COMMIT_HELP: &str = "\
Format:  type(scope): description    e.g. feat(core): add session rotation
Run `make commit` for a guided prompt. The rules are in CONTRIBUTING.md under \"Commits\".";

const NAME_HELP: &str = "\
Format:  ISSUE-XXXX/TYPE/short-descriptive-title    e.g. ISSUE-0001/FEATURE/add-session-rotation
Run `make branch` for a guided prompt. The rules are in CONTRIBUTING.md under \"Branches and pull requests\".";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    let result = match args[..] {
        ["branch"] => tui::branch(),
        ["commit"] => tui::commit(),
        ["lint-commit", file] => lint_commit_file(file),
        ["lint-range", range] => lint_range(range),
        ["lint-name", name] => lint_name(name),
        ["install-hooks"] => install_hooks(),
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn lint_commit_file(path: &str) -> Result<(), String> {
    let message =
        std::fs::read_to_string(path).map_err(|e| format!("could not read {path}: {e}"))?;
    conventional::lint(&message, Mode::Local).map_err(|errors| {
        let header = conventional::clean(&message)
            .lines()
            .next()
            .unwrap_or_default()
            .to_owned();
        report(&format!("commit message `{header}`"), &errors, COMMIT_HELP)
    })
}

fn lint_range(range: &str) -> Result<(), String> {
    let commits = git::commits_in(range)?;
    let mut failed = 0;
    for sha in &commits {
        let message = git::message_of(sha)?;
        if let Err(errors) = conventional::lint(&message, Mode::Strict) {
            let header = message.lines().next().unwrap_or_default();
            eprintln!("✗ {} {header}", &sha[..sha.len().min(10)]);
            for error in errors {
                eprintln!("    - {error}");
            }
            failed += 1;
        }
    }
    if failed > 0 {
        eprintln!("\n{COMMIT_HELP}\nReword commits with `git rebase -i`.");
        return Err(format!(
            "{failed} of {} commit(s) don't follow the conventions",
            commits.len()
        ));
    }
    println!("✓ {} commit(s) follow the conventions", commits.len());
    Ok(())
}

fn lint_name(name: &str) -> Result<(), String> {
    naming::parse(name)
        .map(drop)
        .map_err(|errors| report(&format!("name `{name}`"), &errors, NAME_HELP))
}

fn install_hooks() -> Result<(), String> {
    git::set_config("core.hooksPath", ".githooks")?;
    println!("✓ git now uses the hooks in .githooks/");
    Ok(())
}

fn report(subject: &str, errors: &[String], help: &str) -> String {
    eprintln!("✗ {subject}");
    for error in errors {
        eprintln!("    - {error}");
    }
    eprintln!("\n{help}");
    "check failed".into()
}
