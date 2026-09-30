---
title: Threat models
description: How to write the threat model every security-relevant feature needs.
---

One file per feature, named `<feature>.md`, written before the feature is built. Copy the template below.

```markdown
# Threat model: <feature>

## Specs and references
- Normative spec(s):
- OWASP cheat sheet:
- ASVS requirements covered:

## Assets
What we're protecting (e.g. session tokens, password hashes, account ownership).

## Trust boundaries
Where untrusted data enters: HTTP request, FFI boundary, email link, IdP response, etc.

## Attacker capabilities
What an attacker controls (e.g. request headers, timing measurements, a victim's email inbox scanner).

## Threats (STRIDE)
| # | Threat | Category | Mitigation | Test |
|---|--------|----------|------------|------|
| T1 |       |          |            |      |

## Invariants
Things that must never happen, written as statements the tests check.
```
