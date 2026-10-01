---
title: The crypto crate
description: The cryptographic building blocks in authfil-crypto, and the defaults they use.
---

`authfil-crypto` wires vetted crates together with safe defaults. It contains no novel cryptography.

| Building block | What it does |
|---|---|
| Password hashing | Argon2id ([RFC 9106](https://www.rfc-editor.org/rfc/rfc9106)) |
| Tokens | Generated with a cryptographically secure random number generator (CSPRNG), and stored only as hashes |
| Comparison | Constant-time, so timing doesn't leak how much of a secret matched |
| Secrets | Types that zeroise their memory on drop and never print their contents |

The crate doesn't allow `unsafe` code.

These are Phase 1 work. The [roadmap](../../project/roadmap/#phase-1-cryptographic-and-core-foundations) lists the security requirements for each one.
