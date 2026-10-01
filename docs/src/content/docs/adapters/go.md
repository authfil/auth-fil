---
title: Go
description: Go bindings for Authfil, built with UniFFI.
---

:::caution[Not ready yet]
This adapter is scaffolded in `adapters/go/` but doesn't expose an API yet. Check the [roadmap](../../project/roadmap/) for when it lands.
:::

The Go adapter binds the core to Go with [UniFFI](https://mozilla.github.io/uniffi-rs/).

## API reference

The Go API reference will be generated with godoc from the UniFFI-generated bindings.

## Security

The adapter validates every value that crosses the FFI boundary and carries out the effects the core returns. It never decides a cookie flag, a timeout or whether a token is valid. See [how adapters work](../).

## Contributing

The adapter's source is in `adapters/go/`. Read [how to contribute](../../project/contributing/) first.
