---
title: The sans-IO core
description: Why authfil-core never does I/O, and how inputs and effects keep the rules the same in every language.
---

`authfil-core` never does I/O itself. It doesn't open sockets or files, read the clock or generate randomness. Every security decision lives here, and nowhere else.

## Inputs

The adapter passes in everything the core needs:

- the incoming request;
- the current time;
- random bytes;
- any stored records the flow needs.

## Effects

The core sends back **effects**, such as "set this cookie", "store this session hash", "redirect here" or "deny". The adapter carries them out using the host language's own database and HTTP stack.

## Why it's built this way

- **The rules are the same in every language.** An adapter can't change a cookie flag or a timeout, because it never decides them.
- **Every flow is testable as a pure state transition.** Session lifecycles, OAuth flows and multi-factor authentication (MFA) state can be property-tested and fuzzed without mocks.
- **Adding a language is a translation job.** A new adapter maps types and effects. It doesn't reimplement any authentication logic.

The core doesn't allow `unsafe` code. The full reasoning, and the alternatives we rejected, are in [ADR 0001](../../decisions/0001-sans-io-core/).
