---
title: Architecture
description: How the sans-IO core, the crypto crate and the language adapters fit together in a hexagonal (ports-and-adapters) architecture.
---

Authfil is one Rust engine with a thin adapter for each language. The engine makes every authentication and authorization decision. The adapters only translate.

```mermaid
flowchart TB
    subgraph adapters["Language adapters"]
        direction LR
        node["<b>Node / TypeScript</b><br/>napi-rs"]
        python["<b>Python</b><br/>PyO3"]
        go["<b>Go</b><br/>UniFFI"]
        java["<b>Java</b><br/>JNI"]
        rust["<b>Rust</b><br/>native SDK"]
        more["<b>Future languages</b>"]
    end

    boundary(["Validated FFI boundary<br/>inputs in · effects out"])

    subgraph engine["Rust engine"]
        direction TB
        core["<b>authfil-core</b><br/>sans-IO state machines<br/>policy · access checks"]
        crypto["<b>authfil-crypto</b><br/>Argon2id · tokens<br/>constant-time · zeroize"]
        core --> crypto
    end

    node & python & go & java & rust & more --> boundary --> core

    classDef adapter fill:#e0f2fe,stroke:#0284c7,color:#0c4a6e
    classDef port fill:#fef3c7,stroke:#d97706,color:#78350f
    classDef engine fill:#ede9fe,stroke:#7c3aed,color:#3b0764,stroke-width:2px
    class node,python,go,java,rust,more adapter
    class boundary port
    class core,crypto engine
    style adapters fill:transparent,stroke:#0284c7,stroke-dasharray:5 4
    style engine fill:transparent,stroke:#7c3aed,stroke-dasharray:5 4
```

## Hexagonal architecture

Authfil follows a ports-and-adapters (hexagonal) architecture. `authfil-core` is the hexagon: it owns every auth decision and exposes **ports** — the inputs it needs (a request, the current time, random bytes, stored records) and the effects it produces (set this cookie, store this session hash, deny). It never does I/O and never calls into a specific database or HTTP stack itself.

```mermaid
flowchart TB
    subgraph driving["Driving adapters · call the core"]
        direction LR
        express["Express"]
        fastapi["FastAPI"]
        other["Other frameworks"]
    end

    inbound(["<b>Inbound port</b><br/>request · current time<br/>random bytes<br/>stored records"])
    core{{"<b>authfil-core</b><br/>state machines · policy<br/>access checks"}}
    outbound(["<b>Outbound port</b><br/>effects: set cookie<br/>store hash · redirect · deny"])

    subgraph driven["Driven adapters · carry out effects"]
        direction LR
        db[("Database")]
        response["HTTP response"]
    end

    express & fastapi & other --> inbound
    inbound --> core --> outbound
    outbound --> db & response

    classDef adapter fill:#e0f2fe,stroke:#0284c7,color:#0c4a6e
    classDef port fill:#fef3c7,stroke:#d97706,color:#78350f
    classDef hexagon fill:#ede9fe,stroke:#7c3aed,color:#3b0764,stroke-width:2px
    classDef infra fill:#dcfce7,stroke:#16a34a,color:#14532d
    class express,fastapi,other adapter
    class inbound,outbound port
    class core hexagon
    class db,response infra
    style driving fill:transparent,stroke:#0284c7,stroke-dasharray:5 4
    style driven fill:transparent,stroke:#16a34a,stroke-dasharray:5 4
```

The colours are the same in both diagrams: blue for adapters, amber for ports, purple for the core and green for the infrastructure that adapters talk to. Click any diagram to enlarge it.

Everything outside the hexagon is an **adapter**: the Node, Python, Go, Java, Rust and future-language bindings plug into those ports, and framework integrations such as Express or FastAPI sit a layer further out again. Adapters translate between the host language's ecosystem and the core's ports — they never make auth decisions of their own.

The benefit of drawing the boundary this way is that the core has exactly one implementation of the rules, and adapters are interchangeable, testable in isolation, and can't drift from each other on security-relevant behaviour.

## Where to go next

- [The sans-IO core](../sans-io-core/): why the core never does I/O, and what that buys you.
- [The crypto crate](../crypto/): the primitives every feature is built on.
- [Adapters](../../adapters/): how each language plugs into the core's ports.
- [The conformance suite](../conformance/): the tests that hold every adapter to the same behaviour.
- [ADR 0001](../../decisions/0001-sans-io-core/): the decision record behind this design.
