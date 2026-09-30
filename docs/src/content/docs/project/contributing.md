---
title: How to contribute
description: The contribution workflow, and how to write code docs and site pages.
---

The full guide is `CONTRIBUTING.md` at the root of the repository. This page summarises it and covers how to write documentation.

## The feature workflow

Every security-relevant feature follows the same loop:

1. **Read the spec**: the RFC, NIST or W3C document, plus the matching OWASP cheat sheet.
2. **Write the [threat model](../../security/threat-models/)**: what an attacker controls, and what must never happen.
3. **Write the attacks as tests**: each threat becomes a test that fails if the defence is missing.
4. **Implement it.**
5. **Map it to OWASP ASVS** in the pull request description.

Security issues must not be reported publicly. See `SECURITY.md` in the repository.

## Branches, pull requests and commits

- Every change starts with an issue. Branches are named `ISSUE-XXXX/TYPE/short-descriptive-title`, for example `ISSUE-0001/FEATURE/adding-python-adaptor-X-endpoint`. Pull request titles are Conventional Commits, for example `feat(python): add the X endpoint`.
- Every commit follows [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), such as `feat(core): rotate the session ID on privilege change`, and makes one logical change.
- New code goes where the [directory conventions](../directory-conventions/) say.
- `make branch` and `make commit` walk you through both. `make hooks` runs fmt and clippy and checks commit messages as you commit, adds cargo-deny before each push, and CI checks them on every pull request.

## Writing API docs

The API reference is generated from doc comments, so writing doc comments is how you document the API. Every public item needs one.

Most of the API is defined in Rust, and its `///` comments flow into all three references: rustdoc renders them, napi-rs copies them into the TypeScript declarations, and PyO3 turns them into Python docstrings. Code written directly in TypeScript uses TSDoc, and code written directly in Python uses Google-style docstrings.

## Documentation standards

The full rules for writing documentation, including style, rules for examples, site pages and doc comments, are in `docs/README.md` in the repository. Read them before changing any documentation.
