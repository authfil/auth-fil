---
title: Research resources
description: The standards, books, labs and projects Authloom is built from.
---

Every feature starts from a spec and an attack, not from someone's opinion. This page collects the sources. The [roadmap](../roadmap/) lists which ones apply to each phase.

## Core reading

Keep these open throughout.

- [OWASP ASVS](https://owasp.org/www-project-application-security-verification-standard/): the checklist the library should eventually be verifiable against. We use it as our acceptance criteria.
- [OWASP Cheat Sheet Series](https://cheatsheetseries.owasp.org/): short, practical guidance on each topic.
- [NIST SP 800-63B](https://pages.nist.gov/800-63-4/sp800-63b.html): the authoritative source on passwords, authenticators and session rules.
- [The Copenhagen Book](https://thecopenhagenbook.com/): a free, implementation-focused guide to auth.
- [PortSwigger Web Security Academy](https://portswigger.net/web-security): free labs on authentication, OAuth, JWT and access control attacks. Do the labs for an area before building it.

## Books

- *API Security in Action*, Neil Madden
- *OAuth 2 in Action*, Justin Richer and Antonio Sanso
- *Real-World Cryptography*, David Wong
- *Threat Modeling: Designing for Security*, Adam Shostack

## Specifications by area

| Area | Specs |
|---|---|
| Password hashing | [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106) (Argon2) |
| Cookies | [RFC 6265bis](https://datatracker.ietf.org/doc/draft-ietf-httpbis-rfc6265bis/) (draft) |
| OAuth 2.0 | [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749), [RFC 9700](https://www.rfc-editor.org/rfc/rfc9700) (security BCP), [RFC 7636](https://www.rfc-editor.org/rfc/rfc7636) (PKCE), [RFC 8252](https://www.rfc-editor.org/rfc/rfc8252) (native apps), [RFC 8628](https://www.rfc-editor.org/rfc/rfc8628) (device flow) |
| OpenID Connect | [OIDC Core 1.0](https://openid.net/specs/openid-connect-core-1_0.html), [OIDC Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html) |
| One-time passwords | [RFC 6238](https://www.rfc-editor.org/rfc/rfc6238) (TOTP), [RFC 4226](https://www.rfc-editor.org/rfc/rfc4226) (HOTP) |
| Passkeys | [WebAuthn Level 3](https://www.w3.org/TR/webauthn-3/), [passkeys.dev](https://passkeys.dev/) |
| Tokens | [RFC 7519](https://www.rfc-editor.org/rfc/rfc7519) (JWT), [RFC 8725](https://www.rfc-editor.org/rfc/rfc8725) (JWT BCP), [RFC 9068](https://www.rfc-editor.org/rfc/rfc9068) (JWT access tokens), [RFC 9449](https://www.rfc-editor.org/rfc/rfc9449) (DPoP) |
| Provider mode | [RFC 7662](https://www.rfc-editor.org/rfc/rfc7662) (introspection), [RFC 7009](https://www.rfc-editor.org/rfc/rfc7009) (revocation) |
| Enterprise | [SAML 2.0](https://docs.oasis-open.org/security/saml/v2.0/), [RFC 7643](https://www.rfc-editor.org/rfc/rfc7643) and [RFC 7644](https://www.rfc-editor.org/rfc/rfc7644) (SCIM) |
| Disclosure | [RFC 9116](https://www.rfc-editor.org/rfc/rfc9116) (security.txt) |

Drafts such as OAuth 2.1 and RFC 6265bis change over time. Check for newer revisions before implementing.

## Papers

- Sudhodanan and Paverd, "Pre-hijacked accounts: An Empirical Study of Security Failures in User Account Creation on the Web", USENIX Security 2022.
- Pang et al., "Zanzibar: Google's Consistent, Global Authorization System", USENIX ATC 2019.

## Code to read

- [Better Auth](https://www.better-auth.com/): a TypeScript auth library. Study its plugin model and access control.
- [Kanidm](https://kanidm.com/): a Rust identity server with a strong security posture.
- [webauthn-rs](https://github.com/kanidm/webauthn-rs): a WebAuthn implementation in Rust.
- [Cedar](https://www.cedarpolicy.com/): a formally verified policy language written in Rust.
- [SpiceDB](https://authzed.com/spicedb) and [OpenFGA](https://openfga.dev/): open-source implementations of Zanzibar.
