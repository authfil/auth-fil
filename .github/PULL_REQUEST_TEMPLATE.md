<!--
Title must be a Conventional Commit, e.g. feat(core): add session rotation.
Branches are named ISSUE-XXXX/TYPE/short-descriptive-title (run `make branch`).
CI checks the title and every commit against CONTRIBUTING.md.
-->

## Summary

<!-- What does this change do, and why? Keep it to one concern. -->

Closes #

## Security relevance

<!-- Delete the section that doesn't apply. -->

**Not security-relevant** — adapter plumbing, tooling, docs, or other change with no validation, expiry, comparison, policy or access-check logic.

**Security-relevant:**

- Spec(s) followed: <!-- RFC / NIST / W3C link -->
- OWASP Cheat Sheet: <!-- link -->
- Threat model: `docs/src/content/docs/security/threat-models/<feature>.md`
- ASVS requirements covered: <!-- list -->

## Testing

<!-- What did you test, and how? Unit tests, property tests, fuzz targets, conformance cases. -->

## Checklist

- [ ] `make check` passes (fmt, clippy, tests)
- [ ] `make deny` passes (dependency advisories and licences)
- [ ] Commits are atomic and follow Conventional Commits (`make lint-commits`)
- [ ] Public API items have doc comments
- [ ] New/changed behaviour has a conformance test if it's visible through an adapter
- [ ] Threat model and attack-based tests added, if security-relevant
- [ ] ADR added under `docs/src/content/docs/decisions/`, if this is a significant design decision
