# Documentation rules and standards

This is the rulebook for everything written about Authfil: pages on the docsite, doc comments in the code, and the Markdown files at the repository root. Pull requests that change documentation are reviewed against it.

For an auth library, documentation is part of the security surface. A misleading example gets copied into production. So these rules are strict about accuracy and secure defaults, and relaxed about everything else.

## Contents

- [Where documentation lives](#where-documentation-lives)
- [Running the site](#running-the-site)
- [Writing style](#writing-style)
- [Security rules for examples](#security-rules-for-examples)
- [Site pages](#site-pages)
- [API doc comments](#api-doc-comments)
- [Threat models and ADRs](#threat-models-and-adrs)
- [Generated content](#generated-content)
- [Review checklist](#review-checklist)

## Where documentation lives

| What | Where | Audience |
|---|---|---|
| Site pages, threat models, decisions | `docs/src/content/docs/` | Users and contributors |
| API reference | Doc comments in the code, generated into the site | Users |
| Project overview | `README.md` | First-time visitors |
| Contribution process | `CONTRIBUTING.md` | Contributors |
| Vulnerability reporting | `SECURITY.md` | Security researchers |
| These rules | `docs/README.md` | Anyone writing docs |

Each fact should live in one place. Link to it instead of copying it; copies drift apart.

## Running the site

```sh
make docs-serve    # http://localhost:4321, reloads as you edit
make docs-build    # the static site in docs/dist, exactly as deployed
```

The dev server also regenerates the API reference whenever you change the Rust, TypeScript or Python sources.

## Writing style

**Language**
- Use British English (organisation, behaviour, licence as a noun).
- Write for a developer who knows their web framework but isn't an auth expert.
- Use plain words and short sentences. Prefer "use" to "utilise" and "so" to "in order to".
- Address the reader as "you". Refer to the project as "Authfil" or "we".
- Use the active voice: "the core rotates the session ID", not "the session ID is rotated".

**Structure**
- Headings use sentence case: "Session management", not "Session Management".
- Put the most important information first. A reader should be able to stop after any paragraph.
- Use a numbered list for steps that happen in order, and a bulleted list for anything else.
- Use a table when you're comparing things across the same attributes.

**Terms**
- Define a term the first time you use it on a page, or link to where it's defined.
- Use the spec's terms: "relying party", "authorization code", "authenticator".
- Spell out an acronym on first use per page, unless it's universal (HTTP, URL, API).
- Keep "authentication" (who you are) and "authorization" (what you can do) distinct. Never shorten either to "auth" where the difference matters.

**Precision**
- Cite the spec for a security claim, down to the section: "[RFC 9700 §4.14](https://www.rfc-editor.org/rfc/rfc9700#section-4.14)".
- Don't use "secure" as a bare adjective. Say what it protects against: "resistant to session fixation".
- Don't use marketing language ("blazing fast", "military-grade", "bulletproof").
- Don't promise what the code doesn't do. If a feature is planned, say so and link to the roadmap.

## Security rules for examples

Readers copy examples. Every example must be safe to copy.

1. **Examples use secure defaults.** If the default is secure, the example doesn't mention the option.
2. **Insecure options are fenced off.** An example that turns on a weakening option must sit under a `:::danger` aside that explains the risk and when (if ever) the option is appropriate.
3. **No real secrets.** Use values that are obviously fake: `example-client-secret`, `al_test_0000000000000000`. Never paste a real key, token or password hash, even an expired one.
4. **No real people or domains.** Use `example.com`, `example.org` and `alice@example.com` ([RFC 2606](https://www.rfc-editor.org/rfc/rfc2606)).
5. **Examples must run.** Code examples are complete enough to work, or clearly marked as a fragment with `// ...`.
6. **Show error handling.** Auth failures are normal control flow. An example that ignores the error case teaches readers to ignore it too.

## Site pages

### Files and frontmatter

- Pages go in `docs/src/content/docs/<section>/<page>.md`. Filenames are lowercase kebab-case.
- One topic per page. If a page needs more than one H2 section about unrelated things, split it.
- Every page has a `title` and a `description`. The description is one sentence of up to 160 characters, used in search results and link previews.

```md
---
title: Session management
description: How Authfil creates, rotates and revokes server-side sessions.
---
```

- Don't repeat the title as an H1; Starlight renders it. Start sections at `##`.
- Add each new page to the `sidebar` in `docs/astro.config.mjs`. The exception is `decisions/`, whose sidebar is generated automatically.

### Sections

| Section | For |
|---|---|
| `get-started/` | The first pages a new reader needs |
| `concepts/` | How Authfil works and why, one idea per page |
| `adapters/` | An overview, then one page per language adapter |
| `security/` | Threat models and security design |
| `reference/` | Overview of the API reference; the rest is generated |
| `project/` | Roadmap, contributing, conventions, research, credits, sponsors |
| `decisions/` | Architecture decision records |

The full layout of the repository is on the [directory conventions](src/content/docs/project/directory-conventions.md) page.

### Links

- Link to other pages with relative links and a trailing slash: `[roadmap](../../project/roadmap/)`.
- Link text says where the link goes. Never use "click here" or a bare URL.
- Link to specs at their canonical source: rfc-editor.org for RFCs, w3.org for W3C, openid.net for OpenID.

### Code blocks

- Always set the language: ` ```ts `, ` ```python `, ` ```rust `, ` ```sh `.
- Give a code block a file name with `title="app.ts"` when the reader needs to know where it goes.
- Shell examples don't include a `$` prompt, so they can be copied.

### Asides

Use asides sparingly. A page with an aside in every section has none that stand out.

| Aside | Use for |
|---|---|
| `:::note` | Helpful context the reader can skip |
| `:::tip` | A better way to do something |
| `:::caution` | Something that can cause bugs or confusion |
| `:::danger` | Something that weakens security or loses data |

### Examples in several languages

When a page shows the same thing in more than one language, use tabs in an `.mdx` page with `syncKey="lang"`, so the reader's choice carries across the site. Order the tabs TypeScript, Python, Rust.

```mdx
import { Tabs, TabItem } from '@astrojs/starlight/components';

<Tabs syncKey="lang">
  <TabItem label="TypeScript">...</TabItem>
  <TabItem label="Python">...</TabItem>
  <TabItem label="Rust">...</TabItem>
</Tabs>
```

## API doc comments

The API reference is generated from doc comments, so writing doc comments is how the API gets documented. There are no reference pages to edit by hand.

### Every public item is documented

Every public function, type, method, field and module needs a doc comment. A missing doc comment is a review blocker.

A doc comment contains:
1. **A one-line summary** in the third person, ending with a full stop: "Verifies a password against a stored hash."
2. **Details**, if needed: behaviour, edge cases and security properties.
3. **Sections**, as they apply: errors, panics, security notes and examples.

### Rust: write it once

Most of the public API is defined in Rust. `///` comments flow into all three references: rustdoc renders them, napi-rs copies them into the TypeScript declarations, and PyO3 turns them into Python docstrings. Write them for all three audiences, so avoid Rust-only jargon in anything the adapters expose.

Use the standard rustdoc sections:

```rust
/// Verifies a password against a stored PHC hash in constant time.
///
/// Returns [`Verdict::Rehash`] when the password matches but the hash uses
/// outdated parameters, so the caller can store an upgraded hash.
///
/// # Errors
///
/// Returns [`Error::InvalidHash`] if `stored` is not a valid PHC string.
///
/// # Security
///
/// Runs in constant time with respect to the password, and takes the same
/// time for a missing user when called with [`DUMMY_HASH`].
pub fn verify_password(password: &Secret, stored: &str) -> Result<Verdict, Error> {
```

- Add a `# Security` section to anything with a security property the caller relies on or could undermine.
- Link to other items with intra-doc links (`` [`Verdict`] ``). `make docs-api` fails on broken ones.

### TypeScript

Code written directly in TypeScript uses [TSDoc](https://tsdoc.org/): `/** ... */` with `@param`, `@returns`, `@throws` and `@example`.

### Python

Code written directly in Python uses [Google-style docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings) with `Args:`, `Returns:`, `Raises:` and `Examples:` sections. The generator turns these into parameter tables. Type hints are required, because they're the signatures in the reference.

## Threat models and ADRs

- **Threat models** go in `security/threat-models/<feature>.md`, using the template on that section's index page. Write the threat model before the feature, and keep it updated as the feature changes.
- **ADRs** go in `decisions/NNNN-short-title.md`, numbered in order. Once accepted, an ADR isn't edited. A later ADR replaces it, and the old one's status changes to "Superseded by NNNN".

## Generated content

Never edit these by hand. They're rebuilt on every run and aren't committed:

- `docs/src/content/docs/reference/typescript/`
- `docs/src/content/docs/reference/python/`
- `docs/public/api/`

To change what they say, change the doc comments in the source.

## Review checklist

Before opening a pull request that changes documentation:

- [ ] `make docs-build` succeeds.
- [ ] New pages have a `title` and a `description`, and appear in the sidebar.
- [ ] Links work, including the anchors.
- [ ] Examples use secure defaults and fake values, and would run.
- [ ] Security claims cite a spec.
- [ ] New public API items have doc comments.
- [ ] Commits use the `docs` type, for example `docs(core): explain session rotation`.
