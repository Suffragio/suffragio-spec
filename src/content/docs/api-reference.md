---
title: gRPC API Reference
description: Full gRPC protocol specification for every Suffragio service — commands, queries, and event streams, generated from the canonical .proto definitions.
---

This page documents the gRPC protocol for every service described in [System Architecture](/suffragio-spec/architecture/). It mirrors the canonical, implementation-ready `.proto` files in [`proto/suffragio/v1/`](https://github.com/Suffragio/suffragio-spec/tree/main/proto/suffragio/v1) — treat the `.proto` files as the source of truth for field numbers and message shapes, and this page as the human-readable reference. **Normative behaviour** (cryptography, state machine, Freenet, Lua tallies, auth) is defined in [Protocol v1](/suffragio-spec/protocol-v1/). See [`proto/README.md`](https://github.com/Suffragio/suffragio-spec/blob/main/proto/README.md) for how to lint and generate code with [buf](https://buf.build).

Every service follows the same shape:

- **Commands** — unary RPCs that mutate state.
- **Queries** — unary or server-streaming RPCs that read state without mutating it.
- **Snapshots** — unary RPCs that return rebuildable state plus an `EventCursor` for catch-up.
- **Events** — a single `WatchEvents` server-streaming RPC returning a `oneof`-wrapped event message (each event carries a `cursor`), so any node or auditor can subscribe and independently reconstruct state.

### Cross-cutting conventions

| Topic | Rule |
| --- | --- |
| Package | All messages live in `suffragio.v1`. Shared types are in `common.proto`. |
| Idempotency | Mutating RPCs MUST accept gRPC metadata `idempotency-key: <uuid>`. Replays return the original result. |
| AuthZ | Organizer/officer RPCs go through a pluggable AuthZ port (default OIDC JWT → action strings). See [Protocol v1](/suffragio-spec/protocol-v1/). |
| Transport | For public elections, `RequestBlindSignature` and `SubmitVote` **MUST** use Freenet. Other RPCs MAY use plain gRPC. |
| Watch resume | `WatchEventsRequest` = optional `election_id` + `after_cursor`. Empty `election_id` means all elections (where applicable). |

## Shared types (`common.proto`)

| Type | Fields | Notes |
| --- | --- | --- |
| `ElectionId` | `value: string` | Globally unique election id |
| `ConstituencyId` | `value: string` | Constituency within an election |
| `EligibilityToken` | `value: string` | Opaque random token; semantics in RegSvc state |
| `CryptoSuiteId` | `value: string` | e.g. `BLIND_SIG_RSA_FDH_3072_SHA256` |
| `Constituency` | `id`, `name` | |
| `VotingWindow` | `starts_at`, `ends_at` | timestamps |
| `BsaPublicKey` | `key_id`, `suite_id`, `public_key`, `not_before`, `not_after` | Published BSA verification keys |
| `BallotTemplate` | `constituency_id`, `dsl_version`, `document_json` | Suffragio ballot DSL (JSON), not raw JSON Schema |
| `FormulaScriptRef` | `content_hash`, optional `inline_script`, optional `catalog_script_id` | Lua tally; hash is authoritative |
| `ElectionState` | enum | `DRAFT`, `READY`, `PUBLISHED`, `VOTING`, `CLOSED`, `TALLIED`, `RESULTS_PUBLISHED` |
| `EventCursor` | `value: string` | Resume token for event streams |
| `WatchEventsRequest` | `election_id`, `after_cursor` | Shared watch filter |

## ElectionRegistry

Source of truth for election configuration. `proto/suffragio/v1/election_registry.proto`.

```proto
service ElectionRegistry {
  rpc CreateElection(CreateElectionRequest) returns (CreateElectionResponse);
  rpc DefineBallotTemplate(DefineBallotTemplateRequest) returns (DefineBallotTemplateResponse);
  rpc SetFormulaScript(SetFormulaScriptRequest) returns (SetFormulaScriptResponse);
  rpc AddBsaPublicKey(AddBsaPublicKeyRequest) returns (AddBsaPublicKeyResponse);
  rpc ScheduleElection(ScheduleElectionRequest) returns (ScheduleElectionResponse);
  rpc SetPublicTimestamps(SetPublicTimestampsRequest) returns (SetPublicTimestampsResponse);
  rpc TransitionElectionState(TransitionElectionStateRequest) returns (TransitionElectionStateResponse);
  rpc PublishElection(PublishElectionRequest) returns (PublishElectionResponse);
  rpc GetElection(GetElectionRequest) returns (GetElectionResponse);
  rpc ListElections(ListElectionsRequest) returns (ListElectionsResponse);
  rpc GetElectionSnapshot(GetElectionSnapshotRequest) returns (GetElectionSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream ElectionRegistryEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `CreateElection` | `title: string`, `constituencies: Constituency[]` | `election_id: ElectionId` |
| `DefineBallotTemplate` | `election_id`, `template: BallotTemplate` | *(empty)* |
| `SetFormulaScript` | `election_id`, `script: FormulaScriptRef` | *(empty)* |
| `AddBsaPublicKey` | `election_id`, `key: BsaPublicKey` | *(empty)* |
| `ScheduleElection` | `election_id`, `voting_window: VotingWindow` | *(empty)* |
| `SetPublicTimestamps` | `election_id`, `publish_received_at: bool` | *(empty)* — when `true`, `StreamVotes` may include `received_at`. Default for public political elections **MUST** be `false` ([Protocol v1](/suffragio-spec/protocol-v1/)). |
| `TransitionElectionState` | `election_id`, `to_state: ElectionState` | `state: ElectionState` |
| `PublishElection` | `election_id` | *(empty)* |

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetElection` | `election_id` | `election: ElectionConfig` |
| `ListElections` | `page_size: int32`, `page_token: string` | `elections: ElectionConfig[]`, `next_page_token: string` |
| `GetElectionSnapshot` | `election_id` | `election: ElectionConfig`, `cursor: EventCursor`, `captured_at` |

`ElectionConfig` = `election_id`, `title`, `constituencies[]`, `ballot_templates[]`, `formula_script`, `bsa_public_keys[]`, `voting_window`, `state`, `published: bool`, `publish_received_at: bool`.

### Events (`WatchEvents`)

Subscribe with an empty `election_id` for all elections, or a specific one to filter. Resume with `after_cursor`.

| Event | Fields | Emitted when |
| --- | --- | --- |
| `ElectionCreated` | `election_id`, `title`, `occurred_at` | `CreateElection` succeeds |
| `BallotTemplateDefined` | `election_id`, `template`, `occurred_at` | `DefineBallotTemplate` succeeds |
| `FormulaScriptSet` | `election_id`, `content_hash`, `catalog_script_id`, `occurred_at` | `SetFormulaScript` succeeds |
| `BsaPublicKeyAdded` | `election_id`, `key`, `occurred_at` | `AddBsaPublicKey` succeeds |
| `ElectionScheduled` | `election_id`, `voting_window`, `occurred_at` | `ScheduleElection` succeeds |
| `ElectionStateTransitioned` | `election_id`, `from_state`, `to_state`, `occurred_at` | `TransitionElectionState` succeeds |
| `ElectionPublished` | `election_id`, `occurred_at` | `PublishElection` succeeds |

Each `ElectionRegistryEvent` also carries `cursor: EventCursor`.

## RegistrationEligibility

Validates voter identity/constituency, maintains rolls, issues and atomically consumes single-use `EligibilityToken`s. Never talks to an external identity system directly — see [External identity integration](/suffragio-spec/architecture/#external-identity-integration-anti-corruption-layer). `proto/suffragio/v1/registration_eligibility.proto`.

```proto
service RegistrationEligibility {
  rpc RegisterVoterRoll(RegisterVoterRollRequest) returns (RegisterVoterRollResponse);
  rpc VerifyIdentity(VerifyIdentityRequest) returns (VerifyIdentityResponse);
  rpc RevokeVotingRights(RevokeVotingRightsRequest) returns (RevokeVotingRightsResponse);
  rpc ConsumeEligibilityToken(ConsumeEligibilityTokenRequest) returns (ConsumeEligibilityTokenResponse);
  rpc GetVoterStatus(GetVoterStatusRequest) returns (GetVoterStatusResponse);
  rpc GetRegistrationSnapshot(GetRegistrationSnapshotRequest) returns (GetRegistrationSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream RegistrationEligibilityEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `RegisterVoterRoll` | `election_id`, `voters: VoterEntry[]` (`voter_id`, `constituency_id`) | `registered_count: int32` |
| `VerifyIdentity` | `election_id`, optional `proof: IdentityProof` (`adapter`, `assertion`), optional `auth_session_ref: string` | `token: EligibilityToken`, `constituency_id`, `expires_at` |
| `RevokeVotingRights` | `election_id`, `voter_id`, `reason: string` | *(empty)* |
| `ConsumeEligibilityToken` | `election_id`, `token: EligibilityToken` | `constituency_id`, `expires_at` — **never** `voter_id` |

Notes:

- `IdentityProof.adapter` identifies which external identity adapter produced the assertion, e.g. `"gov-registry"`, `"mobywatel-eidas"`, `"ldap"`, `"oidc"`. Exactly one of `proof` or `auth_session_ref` SHOULD be set (adapter-dependent). `auth_session_ref` covers server-side flows (OIDC redirect, mObywatel broker, in-person registrar session).
- `voter_id` is an opaque, **per-election stable** pseudonym — never a raw national ID / PESEL. Different across elections for external observers ([Protocol v1](/suffragio-spec/protocol-v1/)).
- `RevokeVotingRights` removes from the roll / blocks **new** tokens. It does **not** invalidate ballots already blindly signed or already in the vote log.
- `ConsumeEligibilityToken` is **BSA-only** (AuthZ). It is the sole atomic single-use ledger; BSA MUST NOT keep a parallel spent-token store. Concurrent consumes → at most one success.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetVoterStatus` | `election_id`, `voter_id` | `registered`, `eligible`, `revoked: bool`, `constituency_id` |
| `GetRegistrationSnapshot` | `election_id` | `cursor`, `captured_at`, `registered_count`, `revoked_count`, `tokens_issued_count`, `tokens_consumed_count` (aggregates only — no `voter_id` dump) |

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `VoterRegistered` | `election_id`, `constituency_id`, `count: int32`, `occurred_at` | `RegisterVoterRoll` succeeds |
| `VoterEligibilityVerified` | `election_id`, `constituency_id`, `occurred_at` | `VerifyIdentity` succeeds — **no `voter_id`**, to avoid correlating identity verification with vote casting |
| `VoterRightsRevoked` | `election_id`, `occurred_at` | `RevokeVotingRights` succeeds |
| `EligibilityTokenConsumed` | `election_id`, `constituency_id`, `occurred_at` | `ConsumeEligibilityToken` succeeds — no token value or `voter_id` |

Each `RegistrationEligibilityEvent` also carries `cursor: EventCursor`.

## BlindSignatureAuthority

Consumes an `EligibilityToken` (via RegSvc) and issues a blind signature over a **full filled ballot** it cannot read. See [Blind-signature ballot issuance](/suffragio-spec/architecture/#blind-signature-ballot-issuance) and [Protocol v1](/suffragio-spec/protocol-v1/). `proto/suffragio/v1/blind_signature.proto`.

For public elections, the voter path to this service **MUST** use Freenet.

```proto
service BlindSignatureAuthority {
  rpc RequestBlindSignature(RequestBlindSignatureRequest) returns (RequestBlindSignatureResponse);
  rpc GetBlindSignatureSnapshot(GetBlindSignatureSnapshotRequest) returns (GetBlindSignatureSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream BlindSignatureEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `RequestBlindSignature` | `election_id`, `token: EligibilityToken`, `blinded_ballot: bytes`, `key_id: string`, `suite_id: CryptoSuiteId` | `blind_signature: bytes`, `key_id: string` |

Notes:

- `blinded_ballot` is the suite-specific blinded encoding of the **deterministic CBOR** filled ballot (default suite `BLIND_SIG_RSA_FDH_3072_SHA256`).
- `key_id` selects which published `BsaPublicKey` from the election config will verify the unblinded signature.
- Before signing, BSA MUST call `RegistrationEligibility.ConsumeEligibilityToken`. On failure, do not sign.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetBlindSignatureSnapshot` | `election_id` | `cursor`, `captured_at`, `signatures_issued_count: uint64` |

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `BlindSignatureIssued` | `election_id`, `occurred_at` | `RequestBlindSignature` succeeds — no token, ballot content, signature, or voter identity |

Each `BlindSignatureEvent` also carries `cursor: EventCursor`.

## VoteBroadcastQueue

The public, append-only, multi-writer, hash-chained log of every cast vote (eventual consistency). `proto/suffragio/v1/vote_queue.proto`.

For public elections, `SubmitVote` **MUST** use Freenet. There is **no** `receipt_hash` (coercion resistance).

```proto
service VoteBroadcastQueue {
  rpc SubmitVote(SubmitVoteRequest) returns (SubmitVoteResponse);
  rpc StreamVotes(StreamVotesRequest) returns (stream SignedVote);
  rpc GetLogHead(GetLogHeadRequest) returns (GetLogHeadResponse);
  rpc ReportLogHead(ReportLogHeadRequest) returns (ReportLogHeadResponse);
  rpc GetVoteQueueSnapshot(GetVoteQueueSnapshotRequest) returns (GetVoteQueueSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream VoteQueueEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `SubmitVote` | `election_id`, `constituency_id`, `ballot: bytes`, `signature: bytes`, `key_id`, `suite_id`, optional `prev_hash` | `sequence: uint64`, `entry_hash: bytes`, `prev_hash: bytes` |
| `ReportLogHead` | `election_id`, `reporter_node_id`, `head_hash`, `sequence` | *(empty)* — auxiliary sync signal; **non-binding** |

Notes:

- `ballot` is the deterministic CBOR filled ballot — the **same bytes** covered by the unblinded BSA `signature`.
- Queue MUST verify the BSA signature (using `key_id` against election `bsa_public_keys`) **and** validate the ballot against the constituency ballot DSL. Invalid structure → **reject** (do not append), even if the signature verifies.
- Hash chain: `entry_hash = H(prev_hash || canonical_entry)` (see Protocol v1). Optional request `prev_hash` is advisory under multi-writer.
- Intentionally **no** `receipt_hash`. The voter keeps a local copy of ballot+signature and later searches the official log.
- Binding finality of the vote set is the commission’s m-of-n **official results package**, not a single peer ACK.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `StreamVotes` | `election_id`, `after_sequence: uint64`, `follow: bool` | stream of `SignedVote` |
| `GetLogHead` | `election_id` | `head_hash`, `sequence`, `observed_at` |
| `GetVoteQueueSnapshot` | `election_id` | `cursor`, `head_hash`, `sequence`, `captured_at` |

`SignedVote` = `election_id`, `constituency_id`, `ballot`, `signature`, `key_id`, `suite_id`, `sequence`, `entry_hash`, `prev_hash`, optional `received_at`.

`received_at` is present only if the election’s `publish_received_at` flag is `true`.

`StreamVotes` replays the log from `after_sequence` (0 = from the beginning) and, if `follow` is `true`, keeps streaming new votes live — this is how the Tally Engine and auditors consume the queue.

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `VoteCast` | `election_id`, `sequence`, `entry_hash`, `occurred_at` | `SubmitVote` succeeds |
| `LogHeadReported` | `election_id`, `reporter_node_id`, `head_hash`, `sequence`, `occurred_at` | `ReportLogHead` succeeds |

Each `VoteQueueEvent` also carries `cursor: EventCursor`.

## TallyEngine

Runs the election-pinned **Lua** script over the vote log after an authorized close, and publishes the official m-of-n-signed results package. Official close time is signed `CloseVotingWindow`, not peer NTP. `proto/suffragio/v1/tally.proto`.

```proto
service TallyEngine {
  rpc CloseVotingWindow(CloseVotingWindowRequest) returns (CloseVotingWindowResponse);
  rpc ComputeResults(ComputeResultsRequest) returns (ComputeResultsResponse);
  rpc PublishResults(PublishResultsRequest) returns (PublishResultsResponse);
  rpc GetResults(GetResultsRequest) returns (GetResultsResponse);
  rpc GetOfficialResultsPackage(GetOfficialResultsPackageRequest) returns (GetOfficialResultsPackageResponse);
  rpc GetTallySnapshot(GetTallySnapshotRequest) returns (GetTallySnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream TallyEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `CloseVotingWindow` | `election_id`, `authorizing_signatures: bytes[]` | *(empty)* |
| `ComputeResults` | `election_id`, `log_head_hash: bytes` | `results: ElectionResults` |
| `PublishResults` | `election_id`, `package: OfficialResultsPackage` | *(empty)* |

Notes:

- `authorizing_signatures` are detached signatures from commission keys authorizing close (m-of-n policy).
- `ComputeResults` runs the pinned Lua script (`FormulaScriptRef.content_hash`) over the vote set identified by `log_head_hash`.
- `OfficialResultsPackage` (normative contents):
  - `results: ElectionResults`
  - `log_head_hash: bytes`
  - `formula_content_hash: bytes`
  - optional `formula_catalog_script_id: string`
  - `signatures: CommissionSignature[]` (`key_id`, `signature`, `signed_at`) — **m-of-n** commission/PKW signatures over the canonical package payload

`ElectionResults` = `election_id`, `constituency_results: ConstituencyResult[]`, `total_votes_counted`, `invalid_rejected_at_submit`, `computed_at`, `formula_content_hash`, `log_head_hash`.

`ConstituencyResult` = `constituency_id`, `tally_by_choice: map<string, int64>` — keys are **stable DSL ids** only (not UI labels).

There is no built-in electoral-formula enum; formulas are Lua only in v1.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetResults` | `election_id` | `results: ElectionResults`, `published: bool` |
| `GetOfficialResultsPackage` | `election_id` | `package: OfficialResultsPackage` |
| `GetTallySnapshot` | `election_id` | `cursor`, `closed`, `published`, `log_head_hash`, `captured_at` |

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `VotingWindowClosed` | `election_id`, `occurred_at`, `log_head_hash_hint` | `CloseVotingWindow` succeeds |
| `ResultsPublished` | `election_id`, `occurred_at`, `results_payload_hash`, `log_head_hash`, `formula_content_hash` | `PublishResults` succeeds |

Each `TallyEvent` also carries `cursor: EventCursor`.

## FormulaCatalog

Stores and discovers reusable Lua tally scripts. Elections pin scripts by `content_hash` (inline bytes and/or `catalog_script_id`). Frontend presets (e.g. Polish Sejm/Senat/presidential/referendum) are catalog content, not core engines. `proto/suffragio/v1/formula_catalog.proto`.

```proto
service FormulaCatalog {
  rpc PublishScript(PublishScriptRequest) returns (PublishScriptResponse);
  rpc GetScript(GetScriptRequest) returns (GetScriptResponse);
  rpc ListScripts(ListScriptsRequest) returns (ListScriptsResponse);
  rpc GetFormulaCatalogSnapshot(GetFormulaCatalogSnapshotRequest) returns (GetFormulaCatalogSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream FormulaCatalogEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `PublishScript` | `title`, `description`, `tags: string[]`, `script: bytes` (Lua source) | `script_id: string`, `content_hash: bytes` |

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetScript` | `script_id`, optional `expected_content_hash` | `metadata: FormulaScriptMetadata`, `script: bytes` |
| `ListScripts` | `query`, `page_size`, `page_token`, optional `tag` | `scripts: FormulaScriptMetadata[]`, `next_page_token` |
| `GetFormulaCatalogSnapshot` | *(empty)* | `cursor`, `scripts: FormulaScriptMetadata[]`, `captured_at` |

`FormulaScriptMetadata` = `script_id`, `title`, `description`, `tags[]` (e.g. `"pl-sejm"`, `"pl-president"`, `"referendum"`), `content_hash`, `published_at`.

If `expected_content_hash` is set on `GetScript`, the server MUST return not-found unless the stored bytes match.

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `ScriptPublished` | `metadata`, `occurred_at` | `PublishScript` succeeds |

Each `FormulaCatalogEvent` also carries `cursor: EventCursor`.

## Discovery

Node announcement on the P2P overlay and the public Election Catalog. `proto/suffragio/v1/discovery.proto`.

Suffragio is not a single global network. Like BitTorrent swarms coordinated by different trackers, different organizers may run their election(s) on physically separate P2P networks, each coordinated by its own **tracker** node. A `TrackerRef` (just a Freenet key) identifies which network a piece of data belongs to — so **every** request/response and event below carries one where relevant, letting a client know which network a node or election actually lives on, and which network answered its query.

```proto
message TrackerRef {
  string freenet_key = 1;
}

service Discovery {
  rpc AnnounceNode(AnnounceNodeRequest) returns (AnnounceNodeResponse);
  rpc DiscoverElections(DiscoverElectionsRequest) returns (DiscoverElectionsResponse);
  rpc GetDiscoverySnapshot(GetDiscoverySnapshotRequest) returns (GetDiscoverySnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream DiscoveryEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `AnnounceNode` | `node: NodeInfo` (`node_id`, `roles: NodeRole[]`, `freenet_key`, `tracker: TrackerRef`) | `tracker: TrackerRef` — confirms which network accepted the announcement |

`NodeRole` enumerates `ELECTION_REGISTRY`, `REGISTRATION_ELIGIBILITY`, `BLIND_SIGNATURE_AUTHORITY`, `VOTE_BROADCAST_QUEUE`, `TALLY_ENGINE`, `CATALOG_MIRROR`, `FORMULA_CATALOG`.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `DiscoverElections` | `query: string`, `page_size: int32`, `page_token: string`, `tracker: TrackerRef` (optional — restrict to one network) | `elections: ElectionSummary[]`, `next_page_token: string`, `tracker: TrackerRef` — the network that answered |
| `GetDiscoverySnapshot` | *(empty)* | `cursor`, `nodes: NodeInfo[]`, `captured_at` |

`ElectionSummary` = `election_id`, `title`, `constituencies[]`, `voting_window`, `published: bool`, `tracker: TrackerRef`, `state: ElectionState` — this is the read model backing the [Election Catalog](/suffragio-spec/architecture/#system-components). Two elections in the same response can point to two different trackers, since each election’s services may run on a different network.

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `NodeAnnounced` | `node: NodeInfo`, `occurred_at`, `tracker: TrackerRef` | `AnnounceNode` succeeds |

Each `DiscoveryEvent` also carries `cursor: EventCursor`.
