---
title: Comparing Anonymizing Networks
description: A comparison of Tor, I2P, mixnets, Hyphanet, and Freenet against e-voting's two conflicting requirements — anonymous ballot casting and a durable, publicly trackable vote log — and why Suffragio splits them across two mechanisms.
---

Suffragio needs an anonymizing network for two jobs that look similar but pull in opposite directions. This page compares the major candidates and explains which one (or which combination) fits each job.

## Two requirements, one apparent contradiction

1. **Anonymous casting.** A voter must be able to submit a ballot without anyone — including the infrastructure operators — being able to link the network request back to their identity.
2. **Public, durable, near-live tracking.** Anyone must be able to download the full list of cast votes, follow it as it grows (with acceptable delay), and independently compute the result. That list must be **append-only**: no vote may ever be modified or deleted once recorded.

These pull against each other: anonymity favors ephemeral, hard-to-correlate traffic with no persistent record of *who sent what, when*; a trustworthy public tally favors a persistent, replicated, tamper-evident record that many people can read and verify. A single mechanism rarely optimizes for both, which is why it is worth asking whether Suffragio should use **one network for both jobs, or split them**.

## Candidate networks

### Tor

The best-studied low-latency anonymity network, with millions of daily users and thousands of volunteer relays. Traffic is routed through a fixed 3-hop bidirectional circuit; a hidden service adds another 3 hops on the responder side. Tor's huge anonymity set and battle-tested software are strong points, but it is a **pure transport**: it has no notion of durable, replicated, publicly-queryable storage. If the relay serving a hidden service goes offline, the service disappears. Tor is also known to be vulnerable to traffic-correlation attacks by an adversary who can observe both ends of a circuit (a "global passive adversary"), though this requires substantial resources.

### I2P

A packet-switched peer-to-peer overlay purpose-built for internal hidden services. Every participant routes traffic for others (a more "democratic" model than Tor's client/relay split), and it uses unidirectional tunnels (typically 6 in + 6 out), so a single compromised node only ever sees one direction of a flow. This gives I2P somewhat stronger unlinkability per-connection than Tor for hidden-service-style traffic, at the cost of a much smaller anonymity set (tens of thousands of routers vs. Tor's millions) and less independent security review. Like Tor, I2P is a transport layer — durable, publicly-readable, append-only storage is not part of its design.

### Mixnets (e.g. Nym, Loopix)

Mixnets add cover traffic and per-packet mixing delay at each hop, specifically to resist the timing-correlation attacks that both Tor and I2P are vulnerable to against a global passive adversary. This makes them the strongest anonymity option for the *casting* step. The cost is latency (packets are deliberately delayed and padded) and, for networks like Nym, an economic/token layer needed to incentivize node operators. Mixnets are also newer and less battle-tested than Tor, and — like Tor and I2P — are pure transports with no built-in durable public ledger.

### Hyphanet (formerly Freenet, the original Java network)

The original Freenet project — renamed **Hyphanet** to avoid confusion with the unrelated Rust rewrite described below — is a distributed datastore for anonymous publishing and browsing ("freesites"). Content is split into encrypted chunks and spread across peers based on a key-based, small-world routing scheme, so no single node holds or knows the content of a complete file, and no one operator can be held responsible for what their node stores. Popular content is kept and unpopular content is evicted automatically to make room, and routing/retrieval latency is measured in minutes rather than milliseconds. Hyphanet has no notion of subscribing to live updates on a piece of content — a reader has to re-fetch a key to see if it changed — which makes it a poor fit for a list that needs to be watched growing near-live. Its active core is measured in the thousands of nodes, far smaller than Tor's, and its routing-layer anonymity guarantees have received much less independent cryptographic scrutiny than Tor's.

### Freenet (the unrelated Rust re-implementation Suffragio targets)

Despite sharing a name (and some routing lineage) with Hyphanet, the actively developed Rust **Freenet** (`freenet-core`) is a different network with a different core primitive: a distributed, content-addressed key-value store built on WebAssembly **contracts**, rather than a static file-publishing overlay. Requests are still routed through a small-world peer-to-peer overlay that hides the requester's network origin, but a contract's **state is replicated and durably stored** by peers near its location on the ring, it is readable by anyone who can route to it, and peers can **subscribe** to receive near-real-time updates whenever the state changes. This is precisely the shape of an append-only, publicly-auditable, live-updating vote log: the contract's validity predicate can reject any update that isn't a properly signed new vote, so peers cannot modify or delete existing entries, only append new ones.

The trade-off is similar to Hyphanet's: the network is young, its active core is likely still measured in the thousands of nodes rather than Tor's millions, and its routing-layer anonymity guarantees have received much less independent cryptographic scrutiny than Tor's.

## Comparison

| Property | Tor | I2P | Mixnet (Nym/Loopix) | Hyphanet | Freenet (Rust) |
| --- | --- | --- | --- | --- | --- |
| Primary role | Anonymous transport | Anonymous transport (internal services) | Anonymous transport | Anonymous static publishing/storage | Distributed store + transport |
| Anonymity set | Millions of users, ~7,000+ relays | ~55,000 routers | Small, growing | Thousands of nodes | Thousands of nodes (young network) |
| Resists timing/traffic-correlation (global adversary) | Weak | Weak-to-moderate | Strong (by design) | Weak-to-moderate | Weak-to-moderate |
| Latency | Low (~200–500 ms) | Low-moderate (1–3 s) | Higher (deliberate mixing delay) | High (minutes) | Higher than Tor/I2P; not benchmarked publicly |
| Durable, publicly-readable storage | No | No | No | Yes (static content chunks) | Yes (contract state) |
| Live subscription / push updates | No (poll only) | No | No | No (poll only) | Yes (native `SUBSCRIBE`) |
| Append-only guarantee | N/A | N/A | N/A | No (content is popularity-evicted) | Yes, enforced by contract validity predicate |
| Maturity / independent review | Very high | Moderate | Low-moderate | Moderate | Low (young network) |

No single row is best everywhere: Tor wins on anonymity-set size and maturity for pure transport; mixnets win on resistance to traffic correlation; Freenet (Rust) is the only candidate that natively provides a durable, append-only, subscribable public record — a capability Hyphanet's static-publishing model does not offer.

## Why not just pick the "best" anonymity network for everything?

Tor, I2P, and mixnets are excellent at hiding *who sent a message*, but none of them durably stores anything. There is no notion in any of these networks of "the canonical, ever-growing, publicly downloadable list of all votes cast so far" — that has to be built on top, typically by running a server that logs what it saw, which reintroduces a single point of trust and failure. That defeats the second requirement: a trustworthy, tamper-evident, censorship-resistant public tally.

Hyphanet does store content durably and anonymously, but it was designed for static, popularity-ranked publishing, not a live, ever-growing, append-only record — there is no subscription mechanism, so tracking it means repeatedly re-fetching a key and diffing the result, and nothing stops old, evicted chunks from disappearing under storage pressure.

Freenet (Rust) solves the *tracking* half elegantly because contracts are exactly that: a decentralized, replicated, append-only, live-updating public ledger with no single operator. But its routing-layer anonymity guarantees are less studied and its anonymity set is likely smaller than Tor's, which matters more for the *casting* step, where a voter's one-time request to obtain a blind signature or submit a ballot is the single most sensitive network event in the whole protocol.

## Recommendation: two mechanisms, not one

Given the comparison above, Suffragio should **not** assume a single anonymizing network is the right tool for both jobs:

- **Casting layer — favor the strongest available anonymous transport.** For requesting a blind signature and submitting the completed ballot, prioritize resistance to traffic-correlation attacks and a large anonymity set. Tor (with its huge, well-audited relay network) or a mixnet (for stronger resistance to a global passive adversary, if the added latency is acceptable during the voting window) are both defensible choices, and are strictly better audited for this purpose than Freenet (Rust)'s routing layer alone. Hyphanet is not a good fit here either — it is designed for storage, not for interactive, low-latency request/response.
- **Tracking layer — use Freenet (Rust)'s contract model.** The public, append-only vote log and the final results belong in a Freenet contract: its durability, content-addressing, and native subscription mechanism give every citizen a way to watch the tally grow live and verify that no entry was ever altered or removed, without depending on any single server. Hyphanet cannot offer the live-subscription half of this, and its popularity-based eviction is directly incompatible with an append-only guarantee.

Concretely, this means routing `RequestBlindSignature` and `SubmitVote` calls through Tor (or a mixnet) as the outer transport, while the **destination** of those calls — the Blind Signature Authority and the Vote Broadcast Queue — continues to be backed by Freenet (Rust) contract state for durable, publicly-auditable storage. The two networks are not mutually exclusive: Freenet's own request routing can also run over Tor for an additional layer of origin-hiding on the casting path, at the cost of extra latency that is acceptable for a once-per-election action.

This split maps directly onto the two competing requirements: **anonymity budget goes where the single most sensitive event happens (casting)**, and **durability/liveness budget goes where the public record must be trustworthy and impossible to quietly edit (tracking)**.

## See also

- [Vote Anonymity](/suffragio-spec/vote-anonymity/) — how blind signatures and network anonymity combine to protect ballot secrecy.
- [Why Not Blockchain?](/suffragio-spec/why-not-blockchain/) — why a public blockchain ledger doesn't resolve the same eligibility/anonymity conflict.
- [System Architecture](/suffragio-spec/architecture/) — the full system architecture, including the current Freenet (Rust)-only transport design this page evaluates.
