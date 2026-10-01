---
title: Directory conventions
description: Where each kind of code, test and document lives in the repository, and how to name new files and directories.
---

The repository has one Cargo workspace. Every directory has one job, and the directory a file is in decides which commit scope it belongs to.

## Layout

```
auth-fil/
├── crates/
│   ├── authfil-core/       # Sans-IO core: state machines, policy, effects
│   └── authfil-crypto/     # Hashing, tokens, constant-time comparison, secrets
├── adapters/
│   ├── node/               # Node.js / TypeScript bindings (napi-rs)
│   ├── python/             # Python bindings (PyO3)
│   ├── go/                 # Go bindings (UniFFI)
│   ├── java/               # Java bindings (JNI)
│   └── rust/               # Idiomatic Rust SDK over authfil-core
├── conformance/            # Black-box suite every adapter must pass
├── tools/devtools/         # make branch, make commit and the convention linters
├── docs/                   # This site (Starlight)
│   └── src/content/docs/   # Hand-written pages
├── .githooks/              # pre-commit, commit-msg and pre-push hooks
├── .github/                # Workflows, issue and pull request templates
├── Cargo.toml              # Workspace members, shared versions and lints
├── deny.toml               # cargo-deny: advisories, licences, banned crates
├── Makefile                # Every task, locally and in CI
├── Dockerfile, compose.yaml
└── CONTRIBUTING.md, SECURITY.md, README.md, LICENSE
```

## What goes where

| Change | Directory | Commit scope |
|---|---|---|
| Any security decision: validation, expiry, comparison, policy, access checks | `crates/authfil-core/` | `core` |
| Cryptographic primitives and secret handling | `crates/authfil-crypto/` | `crypto` |
| Converting between a host language and the core | `adapters/<language>/` | the language, for example `node` or `python` |
| Behaviour every adapter must share | `conformance/` | `conformance` |
| Contributor tooling and git hooks | `tools/devtools/`, `.githooks/` | `devtools` |
| Workflows and GitHub templates | `.github/` | `ci` |
| Site pages and the Markdown files at the root | `docs/`, `*.md` | `docs` |
| Container setup | `Dockerfile`, `compose.yaml`, `.dockerignore` | `docker` |
| Lockfile and dependency policy | `Cargo.lock`, `deny.toml` | `deps` |
| Anything else workspace-wide | root files | `repo` |

`make commit` suggests the scope from the files you've staged, using this same mapping.

**Adapters don't make security decisions.** If adapter code sets a cookie flag, checks a token, compares a secret or decides a timeout, it belongs in `crates/authfil-core/` instead. See [how adapters work](../../adapters/).

## Naming

| Thing | Convention | Example |
|---|---|---|
| Core crates | `crates/authfil-<name>/`, and the crate has the same name | `crates/authfil-crypto/` |
| Adapters | `adapters/<language>/`, lowercase, and the crate is `authfil-<language>` | `adapters/go/` is `authfil-go` |
| Site pages | `docs/src/content/docs/<section>/<page>.md`, lowercase kebab-case | `concepts/sans-io-core.md` |
| Threat models | `security/threat-models/<feature>.md` | `security/threat-models/sessions.md` |
| Decision records | `decisions/NNNN-short-title.md`, numbered in order | `decisions/0001-sans-io-core.md` |

## Tests

- **Unit tests** sit next to the code they test, in a `#[cfg(test)] mod tests` block at the bottom of the file.
- **Integration tests** for a crate go in that crate's `tests/` directory.
- **Cross-adapter behaviour** is tested in `conformance/`, never in a single adapter.
- **Fuzz targets** will go in a top-level `fuzz/` directory, run with `make fuzz`.

## Site sections

| Section | For |
|---|---|
| `get-started/` | The first pages a new reader needs |
| `concepts/` | How Authfil works and why |
| `adapters/` | One page per language adapter |
| `security/` | Threat models and security design |
| `reference/` | Overview of the API reference; the rest is generated |
| `project/` | Roadmap, contributing, conventions, research, credits, sponsors |
| `decisions/` | Architecture decision records |

## Generated directories

These are rebuilt on every run and aren't committed. Never edit them by hand:

- `target/`
- `docs/dist/` and `docs/.astro/`
- `docs/public/api/`
- `docs/src/content/docs/reference/typescript/` and `docs/src/content/docs/reference/python/`

## Adding a directory

When you add a crate, an adapter or another top-level directory:

1. Add Rust crates to `members` in the root `Cargo.toml`.
2. Add its scope to `SCOPES` and its path to `scope_for_path` in `tools/devtools/src/conventional.rs`, so `make commit` and CI recognise it.
3. Add the scope to the list under "Commits" in `CONTRIBUTING.md`.
4. Add it to the layout above and to the layout in `README.md`.
