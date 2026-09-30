# Contributing to Authloom

Thanks for helping out. Authloom is an auth library, so a subtle bug here becomes a vulnerability in every app that uses it. The workflow below is stricter than most projects for that reason. It isn't meant to put anyone off, and small, careful contributions are very welcome.

## Before you start

- **Security issues:** don't open a public issue. Follow [SECURITY.md](SECURITY.md).
- **Every change starts with an issue.** Pull requests are named after their issue (see [Branches and pull requests](#branches-and-pull-requests)), so open one first, even for a small fix.
- **New features:** use the issue to check the feature fits the [roadmap](docs/src/content/docs/project/roadmap.md) and to agree on the design before you write code.

## Development setup

You'll need Rust 1.85 or newer (edition 2024). For adapter work you'll also need Node.js (LTS), Python 3.10+, Go and/or a JDK, depending on which adapter you're touching. If you don't want to install those, `make docker-shell` gives you a container that has all of them.

```sh
make tools   # one-off: installs cargo-deny and cargo-fuzz
make hooks   # one-off: runs fmt and clippy before each commit and checks your commit messages
make check   # formatting, clippy and tests
make deny    # dependency advisories and licences
```

A pull request must pass `make check` and `make deny` before review. `make help` lists every target.

## Where code goes

| Change | Location |
|---|---|
| Any security decision (validation, expiry, comparison, policy, access checks) | `crates/authloom-core` |
| Cryptographic primitives and secret handling | `crates/authloom-crypto` |
| Converting between a host language and the core | `adapters/<language>` |
| Behaviour every adapter must share | `conformance/` |

**The main rule: adapters don't make security decisions.** If an adapter PR sets a cookie flag, checks a token, compares a secret or decides a timeout, that logic belongs in the core. Adapters validate input at the FFI boundary, call the core, and carry out the effects it returns.

## The feature workflow

Every security-relevant feature follows the same loop.

1. **Read the spec.** Start from the normative source (the RFC, NIST or W3C document) and the matching [OWASP Cheat Sheet](https://cheatsheetseries.owasp.org/). Link them in your PR.
2. **Write the threat model.** Add `docs/src/content/docs/security/threat-models/<feature>.md` using the [template](docs/src/content/docs/security/threat-models/index.md). It should say what an attacker controls and what must never happen.
3. **Write the attacks as tests.** Each threat in the model becomes a test that would fail if the defence were missing.
4. **Implement it.**
5. **Map it to ASVS.** List the [OWASP ASVS](https://owasp.org/www-project-application-security-verification-standard/) requirements the feature covers in the PR description.

A PR for a security-relevant feature without a threat model and matching tests won't be merged.

## Security rules for code

- **Don't write your own crypto.** Use vetted crates (e.g. `argon2`, `subtle`, `zeroize`, `webauthn-rs`). Adding a new crypto dependency needs its own issue and discussion.
- **Only use a CSPRNG** for anything secret, with at least 128 bits of entropy. Store tokens as hashes, never in plaintext.
- **Compare secrets in constant time.** Never use `==` on tokens, hashes or MACs.
- **Never log secrets.** Wrap secret values in redacting newtypes that zeroise on drop and don't derive `Debug` or `Display` in a way that exposes them.
- **Secure by default.** Every option that weakens security must be off by default and have a name that says what it does (e.g. `allow_insecure_http_cookies`).
- **Fail closed.** If a check can't complete, the answer is deny.
- **Don't reveal whether an account exists.** Use generic error messages and keep timing consistent for lookups by user.
- **Keep the core free of I/O.** `authloom-core` must not open sockets or files, read the clock, or generate its own randomness. Those come in as inputs.
- **No `unsafe`** in `authloom-core` or `authloom-crypto`. Both crates `#![forbid(unsafe_code)]`.

## Testing expectations

- **Unit tests** for every function that makes a decision.
- **Property tests** ([proptest](https://docs.rs/proptest)) for state machines such as session lifecycles, OAuth flows and MFA state.
- **Fuzz targets** ([cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html)) for every parser of untrusted input: tokens, JWTs, cookies, OAuth callbacks, SAML.
- **Conformance tests** for any behaviour visible through an adapter. Add the case to `conformance/` once and every adapter runs it.

## Documentation

**All documentation follows [docs/README.md](docs/README.md).** It covers where docs live, writing style, rules for examples, site pages and API doc comments, and has a review checklist. Read it before changing any docs or adding a public API item.

The short version:

- Every public API item needs a doc comment. The API reference is generated from them.
- Examples must use secure defaults and obviously fake secrets, because readers copy them.
- Security claims cite the spec they come from.

## Adding a new language adapter

1. Open an issue proposing the language and binding approach (e.g. napi-rs, PyO3, UniFFI, a C ABI).
2. Create `adapters/<language>/` with a thin binding over `authloom-core`.
3. Validate every value that crosses the FFI boundary. Treat anything from the host language as untrusted.
4. Make it pass the whole conformance suite. An adapter that fails even one security case isn't released.
5. Add framework integrations (e.g. Express, FastAPI) as separate, thin layers on top.

## Dependencies

New dependencies are reviewed for maintenance status, `unsafe` usage, licence compatibility and known advisories (via RustSec / `cargo-deny`). Prefer fewer, well-audited crates. Explain why you need a new dependency in the PR.

## Branches and pull requests

Branches use this format:

```
ISSUE-XXXX/TYPE/short-descriptive-title
```

For example, `ISSUE-0001/FEATURE/adding-python-adaptor-X-endpoint`.

- **`ISSUE-XXXX`**: the issue number, zero-padded to four digits.
- **`TYPE`**: one of the change types below. It matches the commit type you'll mostly use on the branch.
- **`short-descriptive-title`**: a few words separated by single hyphens.

| `TYPE` | Commit type | For |
|---|---|---|
| `FEATURE` | `feat` | A new feature |
| `FIX` | `fix` | A bug fix |
| `DOCS` | `docs` | Documentation only |
| `STYLE` | `style` | Formatting, no change in behaviour |
| `REFACTOR` | `refactor` | Restructuring, no change in behaviour |
| `PERF` | `perf` | A performance improvement |
| `TEST` | `test` | Adding or correcting tests |
| `BUILD` | `build` | Build system or dependencies |
| `CI` | `ci` | CI configuration |
| `CHORE` | `chore` | Maintenance that fits nowhere else |
| `REVERT` | `revert` | Reverting a previous change |

Run **`make branch`** to create a correctly named branch.

Pull request titles are different: they use the [Conventional Commits](#commits) format, `type(scope): description`, for example `feat(core): add session rotation`. Pull requests are squash-merged, so the title becomes the commit on `main` that release-please reads to version and changelog the release. CI checks the title.

Security fixes are the exception. They're developed privately in a security advisory, following [SECURITY.md](SECURITY.md), so that the branch name doesn't reveal the vulnerability.

In the pull request:

- Keep it to one concern. Small pull requests get reviewed sooner and more thoroughly.
- Link the issue, the specs you followed, and the threat model if there is one.
- Say what you tested and how.
- Security-relevant changes need approval from at least one maintainer who didn't write them.

Pull requests are merged with **rebase merging**, never squashed, so every atomic commit survives into `main`.

## Commits

Every commit must follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) and be atomic. The commit-msg hook (`make hooks`) checks each message as you commit, and CI checks every commit in a pull request. A pull request with a non-conforming commit can't be merged.

### Format

```
type(scope): description

Optional body explaining why the change was needed.

Optional footers, e.g. BREAKING CHANGE: ... or Refs: #12
```

- **type**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore` or `revert` (see the table above).
- **scope** (optional): the area changed, one of `core`, `crypto`, `node`, `python`, `go`, `java`, `rust`, `conformance`, `docs`, `devtools`, `docker`, `ci`, `deps` or `repo`. Leave it out for cross-cutting changes.
- **description**: what the commit does, in the imperative mood and lowercase, with no full stop: `add session rotation`, not `Added session rotation.`
- The header is at most 72 characters.
- Mark a breaking change with `!` after the type or scope, and explain it in a `BREAKING CHANGE:` footer.

Examples:

```
feat(core): rotate the session ID on privilege change
fix(crypto): compare API keys in constant time
docs: add the passkeys threat model
feat(python)!: rename verify to verify_password
```

### Atomic commits

An atomic commit makes **one logical change**, completely:

- **One intent.** If the description needs an "and", it's probably two commits. A refactor and the feature it enables are separate commits.
- **Complete.** Each commit builds and passes its tests, so `git bisect` and reverts work.
- **Self-explanatory.** The header says what the commit does, and the body says why.

Stage selectively with `git add -p` to split mixed changes. To fix an earlier commit on your branch, use `git commit --fixup <sha>`, then `git rebase -i --autosquash main` before review. `fixup!` commits are allowed locally but rejected by CI.

### The commit TUI

Run **`make commit`** instead of `git commit` for a guided prompt. It:

- lists the staged files grouped by scope, and warns when a commit spans unrelated areas or is very large;
- suggests the type from your branch name and the scope from the staged files;
- checks the header as you type;
- adds `Refs: #<issue>` from your branch name.

Run `make lint-commits` to check your branch's commits the same way CI does.

## Architecture decisions

Significant design choices (a new crate, a change to the effect model, a new crypto dependency, a public API shape) are recorded as ADRs in [docs/src/content/docs/decisions/](docs/src/content/docs/decisions/). Propose one in your PR if your change makes such a decision.

## Code of conduct

Be respectful and assume good faith. Criticise the code, not the person. Harassment of any kind isn't tolerated.

## Licence

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
