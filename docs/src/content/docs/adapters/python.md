---
title: Python
description: Python bindings for Authfil, built with PyO3.
---

:::caution[Not ready yet]
This adapter is scaffolded in `adapters/python/` but doesn't expose an API yet. Check the [roadmap](../../project/roadmap/) for when it lands.
:::

The Python adapter binds the core to Python with [PyO3](https://pyo3.rs/), packaged with maturin. PyO3 turns the doc comments written in Rust into Python docstrings.

## Framework integrations

The first framework integration planned is FastAPI, in Phase 2 of the [roadmap](../../project/roadmap/#phase-2-sessions-and-credentials).

## API reference

The Python API reference is generated from the `authfil` package and its `.pyi` stubs, and appears under [API reference](../../reference/) once the adapter exposes an API.

## Security

The adapter validates every value that crosses the FFI boundary and carries out the effects the core returns. It never decides a cookie flag, a timeout or whether a token is valid. See [how adapters work](../).

## Contributing

The adapter's source is in `adapters/python/`. Read [how to contribute](../../project/contributing/) first.
