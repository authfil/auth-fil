---
title: The conformance suite
description: The black-box test suite that every language adapter must pass, and why it's the security contract.
---

The `conformance/` suite sends real HTTP requests to a reference app built with each adapter. It checks security behaviour from the outside:

- cookie flags;
- cross-site request forgery (CSRF) checks;
- session rotation;
- timeouts;
- error messages;
- response timing.

Each case is written once and every adapter must pass all of them. That makes the suite the security contract between the core and the [adapters](../../adapters/): no adapter can quietly weaken a default, and an adapter that fails even one security case isn't released.

The suite is planned from Phase 2 onwards. See the [roadmap](../../project/roadmap/#phase-2-sessions-and-credentials).
