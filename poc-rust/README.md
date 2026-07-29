# Suffragio Rust POC

Proof-of-concept backend for the Suffragio electoral protocol v1, implemented in Rust with `tonic`/`prost`, `tokio`, `rsa`, `mlua`, and `cbor4ii`.

## What is implemented

- **Election lifecycle** (`ElectionRegistry`): create, ballot templates, formula scripts, BSA keys, scheduling, state transitions, publish.
- **Voter registration & eligibility** (`RegistrationEligibility`): voter roll, identity verification stub, token issuance, atomic token consumption.
- **Blind signatures** (`BlindSignatureAuthority`): RSA-FDH 3072-bit blind signing with the `rsa` crate.
- **Vote broadcast queue** (`VoteBroadcastQueue`): vote submission with signature/DSL validation, public append-only vote log with SHA-256 hash chain.
- **Tally engine** (`TallyEngine`): close voting, run Lua formula scripts pinned by content hash, publish results package.
- **Formula catalog** (`FormulaCatalog`): publish/list Lua tally scripts by content hash.
- **Discovery** (`Discovery`): node announcements and election discovery.

## Build

```sh
cargo build
```

## Run the end-to-end demo

```sh
cargo run
```

The demo starts an in-memory gRPC server on `[::1]:50051` and a client that exercises the full voting flow.

## Project layout

```
src/
  app.rs          # AppState and in-memory stores
  crypto.rs       # RSA-FDH blind signature primitives
  ballot.rs       # Deterministic CBOR ballot encoding + DSL validation
  tally.rs        # Lua formula runner
  services/       # gRPC service implementations
  main.rs         # end-to-end demo
```

## Notes

- This is a **proof of concept**. Cryptography, transport, and audit are simplified or stubbed.
- The Freenet transport layer is not yet integrated; all RPCs run over local gRPC.
- `AddBsaPublicKey` currently generates the private key inside the server for the POC.
- `VerifyIdentity` uses a stub adapter: `proof.assertion` is interpreted as the `voter_id`.
