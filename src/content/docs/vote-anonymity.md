---
title: Vote Anonymity
description: How Suffragio cryptographically and architecturally protects the secrecy of individual ballots while keeping the election fully auditable.
---

Suffragio must make two strong guarantees at the same time: every cast vote comes from an eligible voter, and nobody — not the government, not the election organizers, not the infrastructure operators — can tell how a specific person voted. This page explains the mechanisms that keep that second promise.

## The core problem

A naive electronic voting system would store, somewhere, a record that looks like *“Alice voted option B”*. That would let election officials, hackers, or future governments reconstruct each voter’s choice. To prevent this, Suffragio separates **eligibility** (the right to cast a vote) from **the vote itself** at every layer of the system.

The goal is to ensure three forms of unlinkability:

1. **Identity–ballot unlinkability.** The authority that confirms you are allowed to vote never sees the content of your ballot.
2. **Issuance–cast unlinkability.** The act of obtaining an authorized ballot cannot be correlated with the act of submitting a completed vote.
3. **Network origin–vote unlinkability.** The network path you use to submit the vote cannot reveal who you are.

## Blind signatures: eligibility without exposure

The first separation is enforced by a **blind signature** scheme, originally introduced by David Chaum.

In a normal digital signature the signer reads the message, signs it, and the signature proves both authorship and message content. In a blind signature the signer verifies only an *obscured* token and produces a signature on that obscured value. The recipient can later remove the obscuring factor and obtain a signature on the real message — but the signer never saw the real message.

Suffragio uses this as follows (normative detail: [Protocol v1](/suffragio-spec/protocol-v1/)):

1. The voter authenticates to the **Registration & Eligibility Service** and receives a random single-use `EligibilityToken`. This step is identity-linked inside RegSvc only.
2. The voter **fills the complete ballot**; the client encodes it as deterministic CBOR and blinds a suite-specific encoding of **those full bytes** (default `BLIND_SIG_RSA_FDH_3072_SHA256`).
3. Over **Freenet**, the client sends the blinded value plus the token to the **BSA**.
4. The BSA **atomically consumes** the token via RegSvc (response has **no** `voter_id`), then signs the blinded value. It never sees the unblinded ballot or the voter’s identity.
5. The client unblinds locally. The result is a BSA signature on the **entire** filled ballot.

The voter holds proof that *“the BSA authorized this exact ballot content once”* without the BSA learning which voter or what choices.

## Temporal and process decoupling

Even with blind signatures, an observer who sees both the issuance request and the cast vote at the same instant could try to correlate them by timing. Suffragio makes this harder by design:

- The voter may request the blind signature at any point during the voting window, not necessarily immediately before casting.
- For public elections, **both** blind signing and vote submit **MUST** use Freenet, so the BSA and Queue see overlay origins rather than a direct link to the identity-verification session.
- The `EligibilityToken` is consumed during signing, so the same identity cannot request a second signature. There is therefore exactly one signed ballot per eligible voter, but the BSA does not know which ballot it is.

## Anonymous transport: Freenet

Blind signatures hide *what* is being signed, but they do not by themselves hide *who* contacts the voting infrastructure. Suffragio runs over **[Freenet](https://freenet.org)**, a peer-to-peer overlay network that routes every request through other participants:

- The voter’s client never opens a direct TCP connection to the BSA, the Vote Broadcast Queue, or any other service.
- Each request is forwarded through a small-world path of Freenet nodes, making the network origin opaque to the services.
- Requests are served by replicated contract state; there is no single server whose logs could deanonymize voters.

This means that even if an attacker controls some nodes, correlating a network request with a real-world voter is difficult and would require controlling a large fraction of the overlay.

## The public vote log is anonymous

The completed ballot is submitted to the **Vote Broadcast Queue** over Freenet. The queue is public, multi-writer, hash-chained, and eventually consistent: anyone can download cast votes and verify BSA signatures (using the election’s published `key_id` list) and the official m-of-n results package. Entries contain the CBOR ballot, signature, and chain hashes — **not** voter id, session, or (by default in public elections) a precise public `received_at`.

There is **no** cryptographic `receipt_hash` that a third party can demand as proof of how someone voted. The voter may keep a local copy of their ballot and later confirm that an identical entry appears in the official log.

## What each actor can and cannot see

| Actor | Can see | Cannot see |
| --- | --- | --- |
| Registration & Eligibility Service | Who is eligible, who received a token | The ballot content or the signed ballot identifier |
| Blind Signature Authority | That a valid token was consumed and a blind value was signed | The real ballot identifier, the choices, or the voter’s identity |
| Vote Broadcast Queue / auditors | All cast votes and their signatures | Who cast which vote |
| Network operators / Freenet nodes | Encrypted routed traffic | Which vote belongs to which voter |
| Election Organizer | Aggregate results | Individual votes linked to identities |

## Coercion resistance (optional hardening)

The requirements list *prevention of vote selling and coercion* as an optional goal. The base design avoids issuing a binding third-party receipt (`receipt_hash` was explicitly removed in v1). Stronger coercion resistance (deniable credentials, etc.) remains future work.

## Summary

Suffragio protects ballot secrecy through a combination of cryptographic and architectural choices:

- **Blind signatures** separate the authority that checks eligibility from the authority that signs the ballot.
- **Single-use eligibility tokens** ensure one-ballot-per-voter while the token itself reveals nothing about the ballot.
- **Freenet’s peer-to-peer overlay** hides the voter’s network origin from the services they use.
- **An anonymous, append-only vote log** makes every vote auditable without making any vote traceable.

Together, these properties mean that the system can prove the election result was computed from exactly the set of valid, eligible votes — without ever exposing who voted for whom.
