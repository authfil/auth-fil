---
title: Introduction
description: What Authfil is, the problem it solves, and the principles behind it.
---

Authfil is an open-source authentication and authorization library. Its security logic is written once, in Rust, and you use it from your own language through a thin adapter.

:::caution[Early development]
Authfil is in Phase 1 (foundations). Nothing here is ready for production use yet. The [roadmap](../../project/roadmap/) shows what's coming and when.
:::

## Why Authfil

Authentication and authorization are where one small mistake costs the most. Right now every language ecosystem rebuilds them from scratch, and each rebuild brings its own session fixation bugs, timing leaks, weak token handling and OAuth mistakes.

Authfil takes a different approach:

- **The security logic is written once.** A single Rust core holds every security decision: password hashing, session rules, token validation, OAuth state, multi-factor authentication (MFA) enforcement and access checks.
- **Every language can use it.** Thin [adapters](../../adapters/) for Node.js, Python, Go, Java and Rust expose that core through each language's own idioms and web frameworks.
- **Adapters can't weaken it.** The [core](../../concepts/sans-io-core/) makes the decisions itself, and adapters only move bytes. A shared [conformance suite](../../concepts/conformance/) checks that every adapter behaves the same way on security.

## Principles

- **Secure by default.** The safe choice needs no configuration. Any setting that weakens security has to be turned on explicitly and says so in its name.
- **Checked against standards.** Acceptance criteria come from the [OWASP ASVS](https://owasp.org/www-project-application-security-verification-standard/), [NIST SP 800-63B](https://pages.nist.gov/800-63-4/sp800-63b.html) and the relevant RFCs, not from our own opinions.
- **Threat-modelled first.** Every security-relevant feature gets a [threat model](../../security/threat-models/) before it's built, and each threat becomes a test.

## About the name

*Fil* is French for thread: the core's security logic runs through every adapter.

## Next steps

- [Set up a development environment](../development-setup/).
- Read how the [architecture](../../concepts/architecture/) fits together.
- See [how to contribute](../../project/contributing/).
