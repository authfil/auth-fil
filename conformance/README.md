# Adapter conformance suite

A black-box test suite that every language adapter must pass. It sends real HTTP requests to a small reference app built with each adapter, and checks security behaviour from the outside: cookie flags, CSRF checks, session rotation, timeouts, error messages and response timing.

The suite is the adapter security contract. Because every adapter runs the same cases, no adapter can quietly weaken a default. A case is written once here and applies to all of them.

Planned from Phase 2 onwards (see the [roadmap](../docs/src/content/docs/project/roadmap.md)).
