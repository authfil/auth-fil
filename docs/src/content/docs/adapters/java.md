---
title: Java
description: Java bindings for Authloom, built with JNI.
---

:::caution[Not ready yet]
This adapter is scaffolded in `adapters/java/` but doesn't expose an API yet. Check the [roadmap](../../project/roadmap/) for when it lands.
:::

The Java adapter binds the core to Java through the Java Native Interface (JNI).

## API reference

The Java API reference will be generated with Javadoc from the bindings' comments.

## Security

The adapter validates every value that crosses the FFI boundary and carries out the effects the core returns. It never decides a cookie flag, a timeout or whether a token is valid. See [how adapters work](../).

## Contributing

The adapter's source is in `adapters/java/`. Read [how to contribute](../../project/contributing/) first.
