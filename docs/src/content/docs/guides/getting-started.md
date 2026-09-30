---
title: Getting started
description: Set up a development environment and find your way around the repository.
---

Authloom isn't published yet, so for now "getting started" means setting up a development environment.

## Set up

Everything runs through `make`. Run `make help` to list the targets. You can work on your own machine or in Docker.

### On your machine

You need Rust 1.85 or newer. Adapter work also needs the toolchain for that adapter's language: Node.js (LTS) and Python 3.10+ with [uv](https://docs.astral.sh/uv/) for the existing adapters, or Go and a JDK for the Go and Java adapters.

```sh
make tools   # one-off: installs cargo-deny and cargo-fuzz
make build
make check   # formatting, clippy and tests: what every PR must pass
```

### In Docker

The dev image has the Rust, Node and Python toolchains, so you only need Docker.

```sh
make docker-shell   # a shell with the repo mounted at /workspace
make docker-check   # run the PR checks in the container
```

## Repository layout

| Path | What's there |
|---|---|
| `crates/authloom-core` | The sans-IO core: state machines, policy, effects |
| `crates/authloom-crypto` | Hashing, token generation, constant-time operations, secrets |
| `adapters/node` | Node.js / TypeScript bindings (napi-rs) |
| `adapters/python` | Python bindings (PyO3) |
| `adapters/go` | Go bindings (UniFFI) |
| `adapters/java` | Java bindings (JNI) |
| `adapters/rust` | Idiomatic Rust SDK over `authloom-core` |
| `conformance/` | Black-box test suite every adapter must pass |
| `docs/` | This site |

## Run the docs locally

```sh
make docs-serve        # http://localhost:4321, reloads as you edit
make docker-docs       # the same, from the dev container
```

The dev server regenerates the [API reference](../../reference/) whenever the Rust, TypeScript or Python sources change.

## Next steps

- Read the [architecture](../architecture/) overview.
- Check the [roadmap](../../project/roadmap/) to see which phase we're in.
- Read [how to contribute](../contributing/).
