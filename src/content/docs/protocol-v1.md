---
title: Protocol v1 (normative)
description: Normative protocol decisions for implementing a Suffragio-compatible backend — cryptography, ballots, auth, state machine, transport, and audit.
---

This page is the **normative** companion to [System Architecture](/suffragio-spec/architecture/) and the [gRPC API Reference](/suffragio-spec/api-reference/). Where this page conflicts with older prose, **this page wins for v1**.

Implementers (human or automated) MUST treat the rules below as requirements, not suggestions.

## Design decisions summary

| Area | v1 decision |
| --- | --- |
| Blind signature suite | Versioned; default `BLIND_SIG_RSA_FDH_3072_SHA256` |
| What is signed | **Entire** filled ballot (canonical CBOR bytes) |
| BSA keys | List in election config (`key_id`, material, validity); submit carries `key_id` |
| Eligibility token | Random ID + server-side state in Registration & Eligibility (RegSvc) |
| Vote receipt | **No** `receipt_hash`; voter keeps local ballot copy and searches the public log |
| Ballot template | Small custom DSL (JSON), not raw JSON Schema |
| Filled ballot encoding | Deterministic CBOR |
| Choice identifiers | Stable `id` fields in the DSL; tallies key only by `id` |
| Electoral formula | **Lua script** (no built-in formulas); inline or catalog ref; always `content_hash` |
| Formula discovery | Dedicated `FormulaCatalog` gRPC service |
| Invalid ballots | Vote Queue **rejects** at submit (even if BSA signature verifies) |
| Admin auth | Pluggable AuthZ port; default OIDC JWT → action strings; not a replacement IdP |
| AuthZ actions | Arbitrary strings (documented conventions) |
| Election lifecycle | Normative state machine |
| Rights revoke | Affects rolls / new tokens only; does not pull votes from the log; mid-election loss applies to **later** elections |
| Vote log | Multi-writer, append-only **hash chain**, eventual consistency |
| Results timing | May wait hours for sync; official only after PKW/commission m-of-n signatures |
| Transport | Freenet **required** for `RequestBlindSignature` and `SubmitVote` |
| Voter identity proof | Client assertion **or** server-side auth session (per adapter) |
| Published results | Results + log head + Lua `content_hash` + m-of-n signatures |
| Official clock | Signed `CloseVotingWindow` (organizer/PKW), not peer NTP |
| `voter_id` | Stable within one election; unlinkable across elections for outsiders |
| RPC retries | Client `idempotency-key` (UUID) in gRPC metadata |
| Token single-use | Atomic `ConsumeEligibilityToken` on RegSvc only (no BSA-side spent ledger) |
| Event consumption | `WatchEvents` cursor **and** snapshot APIs |
| Public `received_at` | Per-election config; default **off** for public elections |

## Cryptography

### Algorithm registry

Crypto mechanisms are **versioned**. Each election pins the suites it accepts.

| Identifier | Role | v1 status |
| --- | --- | --- |
| `BLIND_SIG_RSA_FDH_3072_SHA256` | Blind signature (RSA Full Domain Hash, 3072-bit modulus, SHA-256) | **Required default** |
| Future IDs | Additional blind-sig or hash suites | Optional; never renumber semantics of old IDs |

Implementations MUST reject unknown suite IDs for elections that do not list them.

### What the BSA signs

1. The voter client builds the **complete** filled ballot as deterministic CBOR (see [Ballots](#ballots)).
2. The client blinds a suite-specific encoding of those bytes (for RSA-FDH: typically `H(domain_separation || ballot_cbor)` mapped to the RSA message representative per the suite profile).
3. BSA signs the **blinded** value; after unblinding, `Verify(bsa_public_key, ballot_cbor, signature)` MUST succeed.
4. `SubmitVote` carries the same `ballot` bytes, `signature`, and `key_id` of the BSA key used.

The BSA MUST NOT learn the unblinded ballot. The signature binds the **entire** ballot content; changing any choice invalidates the signature.

Domain separation MUST include at least `election_id` (and the suite ID) so signatures are not transferable across elections.

### BSA key list

`ElectionConfig` carries a repeated `BsaPublicKey`:

- `key_id` — stable string within the election  
- `suite_id` — e.g. `BLIND_SIG_RSA_FDH_3072_SHA256`  
- `public_key` — encoded public key material for that suite  
- `not_before` / `not_after` — validity window (timestamps)

After the election enters `PUBLISHED`, existing key rows SHOULD be treated as immutable except for appending a new `key_id` (rotation). Votes MUST carry the `key_id` used so auditors pick the correct verification key.

### EligibilityToken

- Opaque random identifier (`EligibilityToken.value`), high entropy (e.g. 128+ bits).  
- All semantics live in RegSvc state: `election_id`, `constituency_id`, `voter_id` (internal), `expires_at`, `consumed`, `revoked_or_invalid`.  
- Issued only by `VerifyIdentity`.  
- Consumed only via atomic `ConsumeEligibilityToken` (see below).  
- BSA receives only consume success/failure and the constituency/election binding needed to sign — **never** `voter_id`.

### No binding vote receipt

`SubmitVote` MUST NOT return a cryptographic receipt that third parties can use to prove how someone voted. The voter retains a local copy of `ballot` + `signature` and, after the official log is published, checks that an identical entry appears in the public log.

## Ballots

### Template DSL

`BallotTemplate` does **not** embed arbitrary JSON Schema as the sole format. It carries a Suffragio ballot DSL document (JSON text) plus metadata:

- `dsl_version` — e.g. `suffragio-ballot-dsl/1`  
- `document_json` — DSL document  

Minimal DSL shape (`suffragio-ballot-dsl/1`):

```json
{
  "dsl_version": "suffragio-ballot-dsl/1",
  "questions": [
    {
      "id": "q_president",
      "type": "single_choice",
      "label": "President",
      "required": true,
      "options": [
        { "id": "cand_a", "label": "Candidate A" },
        { "id": "cand_b", "label": "Candidate B" }
      ]
    }
  ]
}
```

Rules:

- Every question and option MUST have a stable unique `id` (string).  
- Labels are display-only and MUST NOT be used as tally keys.  
- Allowed `type` values in v1 profile: `single_choice`, `multi_choice`, `yes_no`, `ranked` (extend later without breaking ids).  
- Frontend builders and official presets (e.g. Polish Sejm/Senat/presidential/referendum layouts) are UX concerns; the wire format is this DSL.

### Filled ballot encoding

- Encoding: **CBOR** (RFC 8949), **deterministic** / canonical encoding (RFC 8949 §4.2 preferred serialization).  
- Logical content maps question `id` → selected option `id`(s) (or ranked list of option ids).  
- These exact bytes are what the blind signature covers and what the vote log stores.

### Validation at the Vote Queue

On `SubmitVote`, after signature verification, the Queue MUST validate `ballot` against the election’s ballot template DSL for that `constituency_id`. Invalid structure, unknown ids, overvotes, or broken required fields → **reject** (do not append). A valid BSA signature does not override schema validity.

Because the signature covers the full ballot, a rejected submit cannot be “fixed” without a new token and new blind signature. Clients SHOULD validate locally before requesting a blind signature.

## Electoral formulas (Lua)

The core system MUST NOT hardcode FPTP, D’Hondt, STV, etc.

- Tally input: official vote log + election config + Lua script bytes.  
- Script identity: always `content_hash = H(script_bytes)` (suite-defined hash, default SHA-256).  
- Attachment modes:  
  - **inline** — script bytes stored with the election config; or  
  - **catalog** — `script_id` from `FormulaCatalog` plus the same `content_hash` (hash is authoritative if catalog content drifts).  
- Frontend MAY offer presets (including Polish parliamentary/presidential/referendum scripts) that publish into the catalog or inline on create.  
- Execution: sandboxed Lua (no net/FS; deterministic; gas/step limits). Exact sandbox profile is implementation-defined but MUST be documented for auditors reproducing results.

### FormulaCatalog service

Dedicated gRPC service for publishing and discovering scripts (id, metadata, bytes, hash). It does not replace the hash pin on the election.

## Authorization (organizers & officers)

Suffragio does **not** replace enterprise IdP/RBAC.

### AuthZ port

```text
Authorize(principal_proof, action: string, resource) -> allow | deny
```

- Default adapter: **OIDC JWT** bearer; map claims/roles/groups → allow/deny for `action` + resource (`election_id`, …).  
- Additional adapters MAY exist (including national eID brokers for officers).  
- **Voter** eligibility remains on the separate **IdentityProvider** port (mObywatel, OIDC, LDAP, in-person, …) — not the same as organizer AuthZ.

### Actions

Action names are **opaque strings**. Spec conventions (non-exhaustive):

- `election.create`, `election.edit`, `election.publish`, `election.schedule`  
- `roll.register`, `identity.verify`, `rights.revoke`  
- `tally.close`, `tally.compute`, `tally.publish`  
- `formula_catalog.publish`  
- `eligibility_token.consume` (BSA service account only)

Public unauthenticated (for published elections / public log): e.g. `SubmitVote`, `StreamVotes`, `GetElection` (if published), `DiscoverElections` — subject to transport rules below.

### Idempotency

Clients MUST send gRPC metadata `idempotency-key: <uuid>` on mutating RPCs. Servers MUST return the original result for a replayed key within a retention window (implementation-defined, SHOULD be ≥ 24h for voting paths).

## Election state machine

Normative states:

```text
DRAFT → READY → PUBLISHED → VOTING → CLOSED → TALLIED → RESULTS_PUBLISHED
```

| State | Meaning |
| --- | --- |
| `DRAFT` | Mutable config (constituencies, templates, keys, formula script, window) |
| `READY` | Required config complete; not yet public |
| `PUBLISHED` | Discoverable; config frozen except allowed key append / roll updates per policy |
| `VOTING` | Accepting `VerifyIdentity` / BSA / `SubmitVote` per rules; entered at signed window start or explicit transition |
| `CLOSED` | After authorized signed `CloseVotingWindow`; no new votes |
| `TALLIED` | Results computed from committed log + Lua |
| `RESULTS_PUBLISHED` | Official package published with m-of-n signatures |

Illegal transitions MUST fail with a stable error reason (e.g. `INVALID_STATE`).

Config immutability from `PUBLISHED` onward: ballot DSL, formula `content_hash`, constituency set, and voting window MUST NOT change. BSA keys: append-only rotation only.

## Registration, revoke, voter_id

### Identity verification modes

`VerifyIdentity` accepts either:

1. `IdentityProof` with `adapter` + `assertion` bytes (client-held JWT, e-signature, …), or  
2. `auth_session_ref` for server-side flows (OIDC redirect, mObywatel/node broker, LDAP bind, in-person registrar session).

Adapters implement:

```text
IdentityProvider.complete(election_id, proof_or_session) -> { eligible, voter_id, constituency_id }
IdentityProvider.is_revoked(election_id, voter_id) -> bool  // optional global flag
```

### voter_id

- Opaque string in API/rolls — never raw PESEL/national id.  
- **Stable within one election** for the same person.  
- **Different across elections** from an external observer’s perspective (e.g. `HMAC(national_id, per_election_secret)` or random id assigned on first roll insert for that election).

### Revoke / loss of rights

- Modeled as: remove from roll and/or mark ineligible for **new** tokens.  
- Checked at token issuance (`VerifyIdentity`), not inside the vote log.  
- Does **not** invalidate ballots already blindly signed or already in the log.  
- If rights are lost mid-election after a token/signature was issued, the person is blocked from **future** elections’ new tokens; the current election is not surgically rewritten.

### ConsumeEligibilityToken

Public RPC on `RegistrationEligibility`:

- Caller MUST be authorized as BSA (`eligibility_token.consume` or equivalent).  
- Request: `election_id`, `token`.  
- Response on success: `constituency_id`, `expires` confirmation fields as needed — **no `voter_id`**.  
- MUST be atomic: concurrent consumes → exactly one success.  
- BSA MUST NOT keep a parallel spent-token ledger; RegSvc is sole source of single-use truth.

## Vote log integrity

- Append-only log of verified, schema-valid, BSA-signed ballots.  
- **Multi-writer** allowed; each entry links into a **hash chain**:  
  `entry_hash_i = H(prev_hash || canonical_entry_i)` (hash suite pinned per election).  
- **Eventual consistency**: nodes may lag; official results MAY wait hours for convergence.  
- Canonical official log is the set committed in the **signed results package** (see below), not “whatever one peer returned first”.  
- Protocol MAY emit auxiliary signals (e.g. head hash reports, quiescence hints). **Binding** finality is only the commission’s m-of-n signature on the results package.  
- Duplicate detection: reject a second append of the same ballot bytes / same valid signature payload.

### received_at

Whether public log entries expose a receive timestamp is an **election config flag**. For public political elections the default MUST be **false** (omit or strip from public `StreamVotes`). Operators MAY keep private operational timestamps off-protocol.

## Tally & official results

### Close

`CloseVotingWindow` is authorized and **signed** by the organizer/commission policy (m-of-n as configured). Peer wall clocks are not the authority for official close.

### Compute

Run the pinned Lua script over the vote set the commission intends to commit (typically after sync signals look healthy). Output: `ElectionResults` with `tally_by_choice` keyed by DSL option/question ids as the script defines (stable ids only).

### Publish package (normative contents)

Official publication MUST include:

1. Canonical `ElectionResults`  
2. Vote log **head** / commitment (`log_head_hash`, final chain tip, and/or equivalent) identifying the exact ballot set  
3. Lua script `content_hash` (and script id if from catalog)  
4. **m-of-n signatures** from commission/PKW keys over the canonical package bytes  

`ResultsPublished` events MUST carry enough of this package (or a content-addressed reference to it) for auditors to fetch and verify.

### Auditor checklist

1. Load `ElectionConfig` (templates, BSA keys, formula hash).  
2. Fetch official package; verify m-of-n signatures.  
3. Replay log to `log_head_hash`; verify each BSA signature and DSL validity.  
4. Execute Lua with same script hash; compare to published results.  
5. Optionally compare counts of `BlindSignatureIssued` vs cast ballots (unsigned ballots may remain unused — more signatures than votes is allowed; more valid votes than successful consumes is not).

## Transport: Freenet vs gRPC

Logical API remains gRPC-shaped (see `.proto` files). Deployment profiles:

| Path | v1 transport rule |
| --- | --- |
| `RequestBlindSignature` | **MUST** use Freenet (or equivalent anonymizing overlay binding) for public elections |
| `SubmitVote` | **MUST** use Freenet (same) |
| `VerifyIdentity`, organizer admin RPCs, tally, catalog, discovery | MAY use plain gRPC on the organizer network |
| `StreamVotes` / auditor fetch | MAY use plain gRPC |

`ConsumeEligibilityToken` is service-to-service (BSA → RegSvc) and MUST NOT be exposed on a path that deanonymizes voters; it is not called by the voter client.

Freenet binding details (contract layout, correlation of request/response) are specified per implementation but MUST preserve the unlinkability goals in [Vote Anonymity](/suffragio-spec/vote-anonymity/).

## Eventing & snapshots

Every service that exposes `WatchEvents`:

- MUST support a resume **cursor** / sequence (`after_cursor`).  
- MUST support a **snapshot** RPC (or equivalent) of rebuildable state at a version compatible with that cursor.  

Consumers choose snapshot+catch-up or cursor-only replay.

## Inter-service notes for implementers

```text
Voter --VerifyIdentity--> RegSvc  (identity-linked; gRPC OK)
Voter --RequestBlindSignature--> BSA via Freenet
BSA  --ConsumeEligibilityToken--> RegSvc  (service auth; no voter_id in response)
Voter --SubmitVote--> Queue via Freenet
Queue: verify sig + DSL; append hash chain
Commission: wait for sync signals; Close (signed); Compute (Lua); Publish (m-of-n package)
Auditor: verify package + recompute
```

## Open for later revisions (explicitly out of v1 freeze)

- Coercion-resistant / deniable credentials beyond removing receipts  
- Built-in non-Lua formula engines  
- Full Freenet contract ABI standardization across languages  
- Threshold BSA / multi-party blind signing  
- Formal verification of the Lua sandbox  
