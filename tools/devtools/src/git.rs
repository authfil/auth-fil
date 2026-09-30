//! Thin wrappers over the git CLI.

use std::io::Write;
use std::process::{Command, Stdio};

pub struct StagedFile {
    pub path: String,
    /// Lines added plus lines removed; zero for binary files.
    pub lines: usize,
}

fn run(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("could not run git: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn current_branch() -> Option<String> {
    run(&["symbolic-ref", "--quiet", "--short", "HEAD"])
        .ok()
        .map(|s| s.trim().to_owned())
}

pub fn staged() -> Result<Vec<StagedFile>, String> {
    let out = run(&["diff", "--cached", "--numstat", "--no-renames"])?;
    Ok(out
        .lines()
        .filter_map(|line| {
            let mut cols = line.splitn(3, '\t');
            let added = cols.next()?.parse::<usize>().unwrap_or(0);
            let removed = cols.next()?.parse::<usize>().unwrap_or(0);
            Some(StagedFile {
                path: cols.next()?.to_owned(),
                lines: added + removed,
            })
        })
        .collect())
}

pub fn switch_create(branch: &str) -> Result<(), String> {
    run(&["switch", "--create", branch]).map(drop)
}

/// Commit with the given message. Hooks still run, so the commit-msg hook
/// checks the message a second time.
pub fn commit(message: &str) -> Result<(), String> {
    let mut child = Command::new("git")
        .args(["commit", "--file", "-", "--cleanup", "strip"])
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("could not run git: {e}"))?;
    child
        .stdin
        .take()
        .ok_or("could not write to git")?
        .write_all(message.as_bytes())
        .map_err(|e| format!("could not write to git: {e}"))?;
    let status = child
        .wait()
        .map_err(|e| format!("git commit failed: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err("git commit failed".into())
    }
}

/// Non-merge commits in a range such as `origin/main..HEAD`, oldest first.
pub fn commits_in(range: &str) -> Result<Vec<String>, String> {
    let out = run(&["rev-list", "--no-merges", "--reverse", range])?;
    Ok(out.lines().map(str::to_owned).collect())
}

pub fn message_of(sha: &str) -> Result<String, String> {
    run(&["log", "-1", "--format=%B", sha])
}

pub fn set_config(key: &str, value: &str) -> Result<(), String> {
    run(&["config", key, value]).map(drop)
}
