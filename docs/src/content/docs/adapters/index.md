---
title: Adapters
description: How language adapters bring the Rust core to Node.js, Python, Go, Java and Rust apps.
---

An adapter is a thin binding over the [core's](../concepts/sans-io-core/) ports. It has three jobs:

1. Validate every value that crosses the foreign function interface (FFI) boundary, treating it as untrusted.
2. Call the core through its ports.
3. Carry out the effects the core returns.

Adapters never make authentication or authorization decisions of their own. Framework integrations such as Express or FastAPI sit on top of the adapter as separate, thin layers.

## Available adapters

| Language | Binding | Source | Status |
|---|---|---|---|
| [Node.js / TypeScript](node/) | napi-rs | `adapters/node` | Scaffolded |
| [Python](python/) | PyO3 | `adapters/python` | Scaffolded |
| [Go](go/) | UniFFI | `adapters/go` | Scaffolded |
| [Java](java/) | JNI | `adapters/java` | Scaffolded |
| [Rust](rust/) | Native SDK | `adapters/rust` | Scaffolded |

"Scaffolded" means the crate exists but doesn't expose an API yet. The [roadmap](../project/roadmap/) says when each one lands.

Every adapter must pass the [conformance suite](../concepts/conformance/) before it's released.

## Adding an adapter

The steps are in `CONTRIBUTING.md` under "Adding a new language adapter". Put the new adapter in `adapters/<language>/`, following the [directory conventions](../project/directory-conventions/).
