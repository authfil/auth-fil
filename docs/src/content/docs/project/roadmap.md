---
title: Roadmap
description: The seven-phase plan for building Authfil, from crypto foundations to enterprise SSO.
---

The project is built in seven phases. Each one can ship on its own and depends only on the phases before it.

1. **Foundations:** crypto primitives and the sans-IO core everything else sits on.
2. **Sessions and credentials:** the minimum viable auth library.
3. **Federated identity:** OAuth 2.0 and OIDC as a client.
4. **Strong authentication:** MFA and passkeys.
5. **Authorization:** RBAC, organisations, then relationship-based models.
6. **Tokens and API access:** JWTs, API keys, and acting as a provider.
7. **Enterprise:** SAML, SCIM, audit, admin tooling.

## The loop for each feature

1. Read the normative spec (RFC, NIST, W3C) and the matching OWASP cheat sheet.
2. Do the PortSwigger Web Security Academy labs for that area, so you've broken it before you build it.
3. Write the threat model: what an attacker controls, and what must never happen.
4. Implement it, then write tests that turn each threat into a failing case.
5. Map it to the relevant OWASP ASVS requirements and tick them off.

The specs, books and labs behind each phase are collected in [Research resources](../research/).

---

## Phase 1: Cryptographic and core foundations

Get the primitives and the core boundary right first, because every later feature inherits their mistakes. We don't invent any crypto here. The work is choosing vetted crates and wiring them together correctly.

| Feature | Security requirements | Resources |
|---|---|---|
| Password hashing | Argon2id parameters (memory, iterations, parallelism), a salt per hash, self-describing PHC strings, rehash on login when parameters change, optional pepper | RFC 9106, OWASP Password Storage, `argon2` |
| Secure randomness and tokens | CSPRNG only, at least 128 bits of entropy for any secret token, store only a hash of tokens at rest | Copenhagen Book, OWASP Cryptographic Storage |
| Constant-time comparison | Timing side channels in token and hash comparison, and in user lookups (user enumeration by timing) | `subtle`, Real-World Cryptography ch. 3 |
| Secrets in memory | Zeroise secrets, never log or `Debug`-print them, use redacted newtypes | `zeroize`, `secrecy` |
| Key management and rotation | Key IDs, versioned signing keys, rotating keys without invalidating every session | OWASP Key Management |
| Sans-IO core and adapter boundary | Keep security decisions inside Rust so adapters can't weaken them, and validate everything that crosses the FFI boundary | sans-IO pattern, Firezone: sans-IO in Rust, napi-rs, PyO3 |

**Milestone:** a Rust core that hashes and verifies passwords, mints and verifies opaque tokens, and returns effects to a Node test harness. Nothing touches a database yet.

## Phase 2: Sessions and credentials

This phase is the MVP, and it's where most real-world auth bugs happen. Default to server-side opaque sessions: they're easier to revoke and harder to misuse than JWTs.

| Feature | Security requirements | Resources |
|---|---|---|
| Email and password sign-up/sign-in | Password length rules (not composition rules), screening against breached passwords, generic error messages to prevent account enumeration | NIST SP 800-63B, OWASP Authentication, HIBP k-anonymity API |
| Session management | New session ID on login and on privilege change (prevents session fixation), idle and absolute timeouts, server-side revocation, listing and revoking other devices | OWASP Session Management, Copenhagen Book |
| Cookies | `HttpOnly`, `Secure`, `SameSite`, `__Host-` prefixes, cookie scope across subdomains | MDN cookies, RFC 6265bis |
| CSRF protection | Why `SameSite` alone isn't enough, `Origin` checks, synchroniser tokens for forms | OWASP CSRF Prevention, PortSwigger CSRF labs |
| Rate limiting and lockout | Throttling per account and per IP, defences against credential stuffing, stopping lockout from becoming a denial-of-service attack | OWASP Credential Stuffing Prevention |
| Email verification | Single-use, short-lived, hashed tokens tied to the address they were sent to | Copenhagen Book |
| Password reset | Token entropy and expiry, ending all sessions on reset, not revealing whether an account exists, Host header poisoning | OWASP Forgot Password, PortSwigger auth labs |
| Magic links and email OTP | The same rules as reset tokens, plus email scanners prefetching links and flows that switch devices | Copenhagen Book |

**Milestone:** a Node adapter (Express or Hono) and a Python adapter (FastAPI) using one core and passing the same black-box conformance suite.

## Phase 3: Federated identity

Build OAuth 2.0 and OIDC as a client (relying party) first. Acting as a provider comes in Phase 6.

| Feature | Security requirements | Resources |
|---|---|---|
| OAuth 2.0 authorization code flow | `state` for CSRF, exact redirect URI matching, no implicit or password grants | RFC 6749, RFC 9700, OAuth 2.1 draft |
| PKCE | Code interception, S256 only, PKCE for confidential clients too | RFC 7636 |
| OpenID Connect | ID token validation (iss, aud, exp, nonce, signature, azp), caching discovery and JWKS, handling key rotation | OIDC Core 1.0, OIDC Discovery |
| Social providers | Provider quirks, when `email_verified` can and can't be trusted, Apple private relay emails | Provider OIDC docs, oauth.net |
| Account linking | Pre-account-takeover, never linking automatically on an unverified email, linking only from an authenticated session | PortSwigger OAuth labs, "Pre-hijacked accounts" (USENIX Security 2022) |
| Native and mobile clients | Loopback and custom-scheme redirects, why embedded webviews aren't allowed | RFC 8252 |

**Milestone:** sign-in with two providers plus generic OIDC config, with every flow modelled as a pure state transition and covered by property tests.

## Phase 4: Strong authentication

Passkeys are the feature most likely to set the project apart. Do TOTP first as a warm-up.

| Feature | Security requirements | Resources |
|---|---|---|
| TOTP | Generating secrets and storing them encrypted, clock-skew windows, replay prevention | RFC 6238, RFC 4226, OWASP MFA |
| Recovery codes | Stored as hashes, single use, generating a new set invalidates the old one | OWASP MFA |
| MFA enforcement flow | A user who has only passed the password step must not reach anything before the second factor | PortSwigger 2FA bypass labs |
| Passkeys / WebAuthn | Challenge binding, origin and RP ID checks, attestation, signature counters, UV flags, discoverable credentials, conditional UI | WebAuthn Level 3, passkeys.dev, `webauthn-rs` |
| Step-up authentication | Re-authentication for sensitive actions, `auth_time`, `acr`/`amr` | OIDC Core, NIST SP 800-63B |
| SMS OTP (optional) | Why NIST restricts it, SIM-swap risk, SMS pumping fraud | NIST SP 800-63B |

**Milestone:** passwordless sign-up with a passkey, TOTP as a second factor, and MFA state enforced in the core rather than by adapters.

## Phase 5: Authorization

Start with RBAC scoped to organisations, but design the core API as `check(subject, action, resource)` from the start. That way it can grow into ReBAC without breaking adapters.

| Feature | Security requirements | Resources |
|---|---|---|
| Core RBAC | Deny by default, least privilege, roles as bundles of permissions, role hierarchies | NIST RBAC, OWASP Authorization |
| Organisations and teams | Tenant isolation on every query, IDOR prevention, roles scoped to an organisation, expiring invitations, protecting the last owner | PortSwigger access control labs, Oso Academy |
| Privilege change safety | Stopping users granting roles above their own, re-checking permissions on session refresh, revoking sessions on demotion | OWASP ASVS |
| Typed permission schema | Generate typed checks for each language, so a typo fails the build instead of silently denying or allowing | Better Auth access control, Cedar schemas |
| ABAC and policy engines | Attribute conditions, the order policies are evaluated in, keeping policies auditable | Cedar, OPA |
| ReBAC | Relationship graphs, limits on graph traversal, the "new enemy" problem | Zanzibar paper, SpiceDB, OpenFGA |

**Milestone:** org and team RBAC behind a single `check` entry point, typed helpers generated for TypeScript and Python, and a suite of cross-tenant access attempts that must all fail.

## Phase 6: Tokens and API access

Acting as an OAuth/OIDC provider is the hardest feature on the list. Leave it until the client side and sessions are solid.

| Feature | Security requirements | Resources |
|---|---|---|
| JWT access tokens | Pinning the algorithm (no `alg: none`, no RS/HS confusion), `kid` handling, short lifetimes | RFC 7519, RFC 8725, RFC 9068, PortSwigger JWT labs |
| Refresh tokens | Rotation with reuse detection, binding to the client, absolute expiry | RFC 9700 |
| API keys | Prefixed, checksummed keys that secret scanners can detect, stored as hashes, scopes, last-used tracking | GitHub token format, OWASP REST Security |
| Sender-constrained tokens | Proof of possession | RFC 9449 (DPoP) |
| Device authorization flow | Sign-in on CLIs and TVs, phishing of the user code | RFC 8628 |
| OIDC provider mode | Client registration, consent, introspection, revocation, publishing discovery and JWKS | RFC 7662, RFC 7009, OpenID conformance suite |

**Milestone:** Authfil acts as the OIDC provider for a second app and passes the OpenID Foundation conformance tests.

## Phase 7: Enterprise features

These are the features companies pay for, so they're the most likely way to fund the project long term.

| Feature | Security requirements | Resources |
|---|---|---|
| SAML 2.0 SSO | XML signature wrapping, canonicalisation bugs, replay and audience checks. Use a hardened library and never write your own | OASIS SAML 2.0, OWASP SAML Security |
| SCIM provisioning | Authenticating the IdP, deprovisioning that really revokes sessions, scoping every request to a tenant | RFC 7643, RFC 7644 |
| Domain verification and SSO enforcement | DNS TXT verification, forcing SSO for verified domains, blocking fallback to passwords | IdP admin docs |
| Audit logging | What to log, what never to log, making tampering detectable | OWASP Logging |
| Admin tools and impersonation | Impersonation that's clearly marked, time-limited, audited and can't be used to gain more access | OWASP ASVS |
| Account deletion and data export | GDPR erasure across sessions, keys and linked accounts | ICO right to erasure |

**Milestone:** SAML and SCIM working against Okta and Microsoft Entra ID test tenants, with every admin action in the audit log.

---

## Practices for every phase

Start these in Phase 1, not at the end.

| Practice | What it involves | Resources |
|---|---|---|
| Threat modelling | STRIDE for each feature, and the trust boundaries (especially FFI and each adapter) | OWASP Threat Modeling, Shostack |
| Adapter security contract | A conformance suite every adapter must pass | `conformance/` in the repo |
| Fuzzing and property testing | Fuzz every parser, property-test every state machine | `cargo-fuzz`, `proptest` |
| Dependencies and supply chain | Scanning for advisories, licence policy, signed releases, provenance for crates, npm and PyPI | RustSec, `cargo-deny`, SLSA, OpenSSF Scorecard |
| Secure defaults | The safe choice needs no configuration, and every option that weakens security is explicit and named as such | OWASP ASVS |
| Vulnerability disclosure | SECURITY.md, private reporting, security.txt, CVE process | RFC 9116, GitHub private vulnerability reporting |
| External review | Apply for a free audit once Phase 4 ships | OSTIF, OpenSSF |

> OAuth 2.1 and RFC 6265bis were still drafts when this was written. Check for newer revisions before implementing.
