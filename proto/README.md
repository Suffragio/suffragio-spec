# Suffragio gRPC protocol

Canonical Protobuf/gRPC definitions for every Suffragio service described in
the [System Architecture](https://suffragio.github.io/suffragio-spec/architecture/)
docs. This is the single source of truth for the wire protocol — the
Starlight docs render human-readable tables generated from these same
service/method names, but **these `.proto` files are what should be
imported when implementing a service or client**.

## Layout

```
proto/
  buf.yaml            # buf module + lint/breaking-change config
  buf.gen.yaml         # code-generation plugin config (enable what you need)
  suffragio/v1/
    common.proto                  # shared IDs, enums, and value types
    election_registry.proto       # ElectionRegistry service
    registration_eligibility.proto# RegistrationEligibility service
    blind_signature.proto         # BlindSignatureAuthority service
    vote_queue.proto              # VoteBroadcastQueue service
    tally.proto                  # TallyEngine service
    discovery.proto               # Discovery / Election Catalog service
```

Every service file only imports `suffragio/v1/common.proto` — there are no
cross-service imports, so each service can be implemented, versioned, and
deployed independently. Every service also exposes a `WatchEvents`
server-streaming RPC returning a `oneof`-wrapped event type, so consumers
can subscribe to just the events they care about from a single stream.

## Using these definitions

This directory is a self-contained [buf](https://buf.build) module.

```sh
# Lint the protos
buf lint

# Check for breaking changes against the previous version
buf breaking --against '.git#branch=main'

# Generate code for your stack: uncomment the relevant plugins in
# buf.gen.yaml first (Go, TypeScript/Connect, Python, ... — anything with
# a buf.build remote plugin, or a locally installed protoc plugin works).
buf generate
```

Without buf, plain `protoc` works too, e.g. for Go:

```sh
protoc \
  -I proto \
  --go_out=gen/go --go_opt=paths=source_relative \
  --go-grpc_out=gen/go --go-grpc_opt=paths=source_relative \
  proto/suffragio/v1/*.proto
```

## Adding a new service or field

- Never renumber or reuse existing field numbers or enum values.
- New optional fields/RPCs are backward compatible; removing or repurposing
  a field number is not — run `buf breaking` before merging.
- Update the corresponding tables on the Starlight
  [gRPC API Reference](https://suffragio.github.io/suffragio-spec/api-reference/)
  page (`src/content/docs/api-reference.md` and `src/content/docs/pl/api-reference.md`)
  alongside any `.proto` change.
