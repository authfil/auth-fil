---
title: "0001: Sans-IO core"
description: All security decisions live in a Rust core that performs no I/O.
---

**Status:** Accepted

## Context

Authfil targets several languages. If each adapter implemented parts of the auth logic, such as cookie flags, timeouts or token checks, those implementations would drift apart. Each would be a separate place for vulnerabilities to appear.

## Decision

`authfil-core` makes every security decision and performs no I/O. Adapters pass in requests, the current time, random bytes and stored records. The core returns effects that the adapter carries out.

## Alternatives

- **A Rust library that does its own I/O** (a database driver and an HTTP client). Rejected because it would force one storage and HTTP stack on every host language and make flows hard to test without mocks.
- **A separate auth server process.** Rejected for now because it adds deployment complexity for users. It stays possible later as another adapter over the same core.

## Consequences

- Security behaviour is identical across languages, and a shared conformance suite can check it.
- Flows can be property-tested and fuzzed as pure state transitions.
- Adapters are more work to write at first, because they must carry out effects rather than just call a function.
