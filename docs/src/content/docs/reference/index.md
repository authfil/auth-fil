---
title: API reference
description: How the generated API reference for each language works.
---

The API reference is generated from the source code, including signatures and doc comments. It's rebuilt on every docs build, and the dev server rebuilds it whenever a source changes. A language's section appears once its adapter exists.

| Language | Generated from | Tool |
|---|---|---|
| Rust | The workspace crates | rustdoc |
| TypeScript | `adapters/node/index.d.ts`, which napi-rs generates | TypeDoc |
| Python | The `authloom` package and its `.pyi` stubs | griffe |
| Go | The UniFFI-generated Go bindings | godoc |
| Java | The JNI bindings' Javadoc comments | Javadoc |

Most public items are written in Rust. Their `///` doc comments flow into all three references, so each item is documented once. See [writing API docs](../guides/contributing/#writing-api-docs).

## Status

The adapters are planned for Phase 1 and Phase 2 of the [roadmap](../project/roadmap/). Until then, only the Rust reference is available.
