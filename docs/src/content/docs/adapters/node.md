---
title: Node.js / TypeScript
description: Node.js and TypeScript bindings for Authfil, built with napi-rs.
---

:::caution[Not ready yet]
This adapter is scaffolded in `adapters/node/` but doesn't expose an API yet. Check the [roadmap](../../project/roadmap/) for when it lands.
:::

The Node.js adapter binds the core to JavaScript with [napi-rs](https://napi.rs/). napi-rs also generates the TypeScript declarations, including the doc comments written in Rust.

## Framework integrations

The first framework integrations planned are Express and Hono, in Phase 2 of the [roadmap](../../project/roadmap/#phase-2-sessions-and-credentials).

## API reference

The TypeScript API reference is generated from `adapters/node/index.d.ts` and appears under [API reference](../../reference/) once the adapter exposes an API.

## Security

The adapter validates every value that crosses the FFI boundary and carries out the effects the core returns. It never decides a cookie flag, a timeout or whether a token is valid. See [how adapters work](../).

## Contributing

The adapter's source is in `adapters/node/`. Read [how to contribute](../../project/contributing/) first.
