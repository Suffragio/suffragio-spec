# Suffragio gRPC protocol

Canonical Protobuf/gRPC definitions for every Suffragio service.

**Normative behaviour** (crypto, state machine, Freenet rules, Lua tallies, auth):  
→ [Protocol v1](https://suffragio.github.io/suffragio-spec/protocol-v1/) (`src/content/docs/protocol-v1.md`)

Human-readable RPC tables:  
→ [gRPC API Reference](https://suffragio.github.io/suffragio-spec/api-reference/)

## Layout

```
proto/
  buf.yaml
  buf.gen.yaml
  suffragio/v1/
    common.proto
    election_registry.proto
    registration_eligibility.proto
    blind_signature.proto
    vote_queue.proto
    tally.proto
    discovery.proto
    formula_catalog.proto
```

## Using these definitions

```sh
buf lint
buf breaking --against '.git#branch=main'
buf generate
```

## Conventions (v1)

- Mutating RPCs: clients send gRPC metadata `idempotency-key: <uuid>`.
- `WatchEvents` supports `after_cursor`; each service also exposes a snapshot RPC.
- Public elections: `RequestBlindSignature` and `SubmitVote` MUST use Freenet (see Protocol v1).
- No `receipt_hash` on `SubmitVote`.
- Tallies are Lua scripts (`FormulaCatalog` or inline), not built-in formula enums.
