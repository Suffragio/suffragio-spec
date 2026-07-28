---
title: Why Not Blockchain?
description: Why Suffragio is not built on blockchain — the fundamental conflict between eligibility and anonymity, the mitigations that were considered, and why they fall short.
---

At first glance, blockchain looks like an ideal fit for e-voting. This page explains why Suffragio doesn't use it anyway.

## The appeal

Blockchain guarantees that contracts cannot be tampered with and that the entire electoral process is fully auditable. A less obvious, but equally important, advantage is access to distributed infrastructure whose operation cannot easily be disrupted — and which government institutions do not need to bear the cost of building themselves.

## The core conflict

Unfortunately, no available blockchain implementation can resolve a fundamental conflict at the heart of any nationwide electoral system: it must simultaneously guarantee that

- **only eligible voters receive a ballot**, and
- **no one can determine who cast which vote** (anonymity).

A public, transparent ledger is very good at the first property and very bad at the second — every transaction is, by design, traceable back to its sender.

## Attempted mitigations

### Mixers (e.g. Tornado Cash)

Mixers provide a very high degree of anonymity. An alternative smart-contract implementation of this pattern can achieve full anonymity, but it introduces a new problem: once a mixer has been used, there is no way to guarantee that government institutions won't generate their own votes and "stuff the ballot box" alongside the real ones.

### Oracles

Using oracles inside a smart contract can guarantee full anonymity *and* confirm that only one person redeemed a given ballot. Unfortunately, in an e-voting system the center of gravity shifts so heavily toward the oracle that we lose blockchain's key advantages while keeping its drawbacks. The oracle would need to be controlled by government institutions to the same degree as in a traditional, centralized electoral system — defeating the point of using a blockchain in the first place.

## Other considerations

These aren't decisive on their own, but they are worth weighing too:

- **Environmental cost.** Executing smart contracts is simply energy-hungry. Equivalent solutions can often be implemented far more cheaply, with a much smaller carbon footprint.
- **Unpredictable infrastructure costs.** It's theoretically possible to write smart contracts that don't require gas fees for every transaction. In practice, though, guaranteeing that operations complete within a time acceptable to an average user can make the cost of running a nationwide election disproportionately high — not always, but the lack of predictability itself is the problem.
- **High system requirements for auditors.** Someone wanting to audit a single election is forced to run a full node (e.g. an Ethereum node) and download a long transaction history largely unrelated to the election they actually care about. When designing an e-voting system, we wanted to make verification accessible to an average citizen with an ordinary home laptop — so that anyone can go through the entire verification process from their living room, without buying an absurd amount of disk space or memory, and without a complicated infrastructure setup.
