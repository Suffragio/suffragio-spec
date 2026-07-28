---
title: gRPC API Reference
description: Full gRPC protocol specification for every Suffragio service — commands, queries, and event streams, generated from the canonical .proto definitions.
---

This page documents the gRPC protocol for every service described in [System Architecture](/suffragio-spec/architecture/). It mirrors the canonical, implementation-ready `.proto` files in [`proto/suffragio/v1/`](https://github.com/Suffragio/suffragio-spec/tree/main/proto/suffragio/v1) — treat the `.proto` files as the source of truth and this page as the human-readable reference. See [`proto/README.md`](https://github.com/Suffragio/suffragio-spec/blob/main/proto/README.md) for how to lint and generate code from them with [buf](https://buf.build).

Every service follows the same shape:

- **Commands** — unary RPCs that mutate state.
- **Queries** — unary or server-streaming RPCs that read state without mutating it.
- **Events** — a single `WatchEvents` server-streaming RPC returning a `oneof`-wrapped event message, so any node or auditor can subscribe to just this service's events and independently reconstruct its state.

All messages are defined in package `suffragio.v1`. Shared types (`ElectionId`, `ConstituencyId`, `EligibilityToken`, `ElectoralFormula`, `Constituency`, `VotingWindow`, `BallotTemplate`) live in `common.proto` and are reused across every service below.

## ElectionRegistry

Source of truth for immutable election configuration. `proto/suffragio/v1/election_registry.proto`.

```proto
service ElectionRegistry {
  rpc CreateElection(CreateElectionRequest) returns (CreateElectionResponse);
  rpc DefineBallotTemplate(DefineBallotTemplateRequest) returns (DefineBallotTemplateResponse);
  rpc SetElectoralFormula(SetElectoralFormulaRequest) returns (SetElectoralFormulaResponse);
  rpc ScheduleElection(ScheduleElectionRequest) returns (ScheduleElectionResponse);
  rpc PublishElection(PublishElectionRequest) returns (PublishElectionResponse);
  rpc GetElection(GetElectionRequest) returns (GetElectionResponse);
  rpc ListElections(ListElectionsRequest) returns (ListElectionsResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream ElectionRegistryEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `CreateElection` | `title: string`, `constituencies: Constituency[]` | `election_id: ElectionId` |
| `DefineBallotTemplate` | `election_id`, `template: BallotTemplate` | *(empty)* |
| `SetElectoralFormula` | `election_id`, `formula: ElectoralFormula`, `params: map<string,string>` | *(empty)* |
| `ScheduleElection` | `election_id`, `voting_window: VotingWindow` | *(empty)* |
| `PublishElection` | `election_id` | *(empty)* |

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetElection` | `election_id` | `election: ElectionConfig` |
| `ListElections` | `page_size: int32`, `page_token: string` | `elections: ElectionConfig[]`, `next_page_token: string` |

`ElectionConfig` = `election_id`, `title`, `constituencies[]`, `ballot_templates[]`, `formula`, `voting_window`, `published: bool`.

### Events (`WatchEvents`)

Subscribe with an empty `election_id` for all elections, or a specific one to filter.

| Event | Fields | Emitted when |
| --- | --- | --- |
| `ElectionCreated` | `election_id`, `title`, `occurred_at` | `CreateElection` succeeds |
| `BallotTemplateDefined` | `election_id`, `template`, `occurred_at` | `DefineBallotTemplate` succeeds |
| `ElectoralFormulaSet` | `election_id`, `formula`, `occurred_at` | `SetElectoralFormula` succeeds |
| `ElectionScheduled` | `election_id`, `voting_window`, `occurred_at` | `ScheduleElection` succeeds |
| `ElectionPublished` | `election_id`, `occurred_at` | `PublishElection` succeeds |

## RegistrationEligibility

Validates voter identity/constituency and issues single-use `EligibilityToken`s. Never talks to an external identity system directly — see [External identity integration](/suffragio-spec/architecture/#external-identity-integration-anti-corruption-layer). `proto/suffragio/v1/registration_eligibility.proto`.

```proto
service RegistrationEligibility {
  rpc RegisterVoterRoll(RegisterVoterRollRequest) returns (RegisterVoterRollResponse);
  rpc VerifyIdentity(VerifyIdentityRequest) returns (VerifyIdentityResponse);
  rpc RevokeVotingRights(RevokeVotingRightsRequest) returns (RevokeVotingRightsResponse);
  rpc GetVoterStatus(GetVoterStatusRequest) returns (GetVoterStatusResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream RegistrationEligibilityEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `RegisterVoterRoll` | `election_id`, `voters: VoterEntry[]` (`voter_id`, `constituency_id`) | `registered_count: int32` |
| `VerifyIdentity` | `election_id`, `proof: IdentityProof` (`adapter: string`, `assertion: bytes`) | `token: EligibilityToken`, `constituency_id`, `expires_at` |
| `RevokeVotingRights` | `election_id`, `voter_id`, `reason: string` | *(empty)* |

`IdentityProof.adapter` identifies which [External Identity Adapter](/suffragio-spec/architecture/#external-identity-integration-anti-corruption-layer) produced the assertion, e.g. `"gov-registry"`, `"mobywatel-eidas"`, `"ldap"`, `"oidc"`.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetVoterStatus` | `election_id`, `voter_id` | `registered`, `eligible`, `revoked: bool`, `constituency_id` |

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `VoterRegistered` | `election_id`, `constituency_id`, `count: int32`, `occurred_at` | `RegisterVoterRoll` succeeds |
| `VoterEligibilityVerified` | `election_id`, `constituency_id`, `occurred_at` | `VerifyIdentity` succeeds — **no `voter_id`**, to avoid correlating identity verification with vote casting |
| `VoterRightsRevoked` | `election_id`, `occurred_at` | `RevokeVotingRights` succeeds |

## BlindSignatureAuthority

Consumes an `EligibilityToken` and issues a blind signature over a ballot it cannot read. See [Blind-signature ballot issuance](/suffragio-spec/architecture/#blind-signature-ballot-issuance). `proto/suffragio/v1/blind_signature.proto`.

```proto
service BlindSignatureAuthority {
  rpc RequestBlindSignature(RequestBlindSignatureRequest) returns (RequestBlindSignatureResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream BlindSignatureEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `RequestBlindSignature` | `election_id`, `token: EligibilityToken`, `blinded_ballot: bytes` | `blind_signature: bytes` |

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `BlindSignatureIssued` | `election_id`, `occurred_at` | `RequestBlindSignature` succeeds — no token, ballot content, or signature included |

## VoteBroadcastQueue

The public, append-only, replicated log of every cast vote. `proto/suffragio/v1/vote_queue.proto`.

```proto
service VoteBroadcastQueue {
  rpc SubmitVote(SubmitVoteRequest) returns (SubmitVoteResponse);
  rpc StreamVotes(StreamVotesRequest) returns (stream SignedVote);
  rpc WatchEvents(WatchEventsRequest) returns (stream VoteQueueEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `SubmitVote` | `election_id`, `constituency_id`, `ballot: bytes`, `signature: bytes` | `sequence: uint64`, `receipt_hash: bytes` |

### Queries

| RPC | Request | Response (stream) |
| --- | --- | --- |
| `StreamVotes` | `election_id`, `after_sequence: uint64`, `follow: bool` | `SignedVote` (`election_id`, `constituency_id`, `ballot`, `signature`, `sequence`, `received_at`) |

`StreamVotes` replays the full log from `after_sequence` and, if `follow` is `true`, keeps streaming new votes live — this is how the Tally Engine and auditors consume the queue.

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `VoteCast` | `election_id`, `sequence: uint64`, `occurred_at` | `SubmitVote` succeeds |

## TallyEngine

Applies the configured electoral formula to the vote log after the voting window closes. `proto/suffragio/v1/tally.proto`.

```proto
service TallyEngine {
  rpc CloseVotingWindow(CloseVotingWindowRequest) returns (CloseVotingWindowResponse);
  rpc ComputeResults(ComputeResultsRequest) returns (ComputeResultsResponse);
  rpc PublishResults(PublishResultsRequest) returns (PublishResultsResponse);
  rpc GetResults(GetResultsRequest) returns (GetResultsResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream TallyEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `CloseVotingWindow` | `election_id` | *(empty)* |
| `ComputeResults` | `election_id` | `results: ElectionResults` |
| `PublishResults` | `election_id` | *(empty)* |

`ElectionResults` = `election_id`, `formula`, `constituency_results: ConstituencyResult[]`, `total_votes_counted: uint64`, `computed_at`. `ConstituencyResult` = `constituency_id`, `tally_by_choice: map<string,int64>`.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `GetResults` | `election_id` | `results: ElectionResults`, `published: bool` |

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `VotingWindowClosed` | `election_id`, `occurred_at` | `CloseVotingWindow` succeeds |
| `ResultsPublished` | `election_id`, `occurred_at` | `PublishResults` succeeds |

## Discovery

Node announcement on the P2P overlay and the public Election Catalog. `proto/suffragio/v1/discovery.proto`.

Suffragio is not a single global network. Like BitTorrent swarms coordinated by different trackers, different organizers may run their election(s) on physically separate P2P networks, each coordinated by its own **tracker** node. A `TrackerRef` (just a Freenet key) identifies which network a piece of data belongs to — so **every** request/response and event below carries one, letting a client know which network a node or election actually lives on, and which network answered its query.

```proto
message TrackerRef {
  string freenet_key = 1;
}

service Discovery {
  rpc AnnounceNode(AnnounceNodeRequest) returns (AnnounceNodeResponse);
  rpc DiscoverElections(DiscoverElectionsRequest) returns (DiscoverElectionsResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream DiscoveryEvent);
}
```

### Commands

| RPC | Request | Response |
| --- | --- | --- |
| `AnnounceNode` | `node: NodeInfo` (`node_id`, `roles: NodeRole[]`, `freenet_key`, `tracker: TrackerRef`) | `tracker: TrackerRef` — confirms which network accepted the announcement |

`NodeRole` enumerates `ELECTION_REGISTRY`, `REGISTRATION_ELIGIBILITY`, `BLIND_SIGNATURE_AUTHORITY`, `VOTE_BROADCAST_QUEUE`, `TALLY_ENGINE`, `CATALOG_MIRROR`.

### Queries

| RPC | Request | Response |
| --- | --- | --- |
| `DiscoverElections` | `query: string`, `page_size: int32`, `page_token: string`, `tracker: TrackerRef` (optional — restrict to one network) | `elections: ElectionSummary[]`, `next_page_token: string`, `tracker: TrackerRef` — the network that answered |

`ElectionSummary` = `election_id`, `title`, `constituencies[]`, `voting_window`, `published: bool`, `tracker: TrackerRef` — this is the read model backing the [Election Catalog](/suffragio-spec/architecture/#system-components). Two elections in the same response can point to two different trackers, since each election's services may run on a different network.

### Events (`WatchEvents`)

| Event | Fields | Emitted when |
| --- | --- | --- |
| `NodeAnnounced` | `node: NodeInfo`, `occurred_at`, `tracker: TrackerRef` | `AnnounceNode` succeeds |
