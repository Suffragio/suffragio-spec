---
title: Verdict
description: Conclusion of the market research — no surveyed system satisfies all of Suffragio's requirements, and why.
---

This page distills the [Findings](/suffragio-spec/market-research/findings/) and [Comparison Matrix](/suffragio-spec/market-research/matrix/) into a direct answer to three questions: which existing system comes closest to satisfying Suffragio's requirements, how far it is from "perfect," and why that gap justifies building a new system rather than adopting or extending an existing one.

## No single system satisfies all 8 requirements

Across all 45 surveyed systems, none scores close to "all ✅" on Suffragio's eight requirements (full names — see [Motivation & Requirements](/suffragio-spec/motivation/)). Two structural clusters emerge:

- **Open-source / academic cryptographic tools** (Helios, Belenios, Zeus family) — strong on verifiability, ballot secrecy, and digital independence, but weak on universality, electoral-formula flexibility, and identity verification/delegation.
- **Government-deployed systems** (Estonia's IVXV, Swiss Post e-voting, Geneva's CHVote) — strong on universality, identity verification, and real-world production scale, but weak or failing on digital independence, and in some cases on verifiability or secrecy guarantees (e.g. Estonia's client-side attack surface, the 2019 Scytl/Swiss Post cryptographic flaw fixed before deployment).

## Best off-the-shelf candidate

**Belenios** (or its close siblings Helios and Zeus) is the best generic candidate if developing a new system is not an option:

| Requirement | Belenios |
|---|---|
| Low-cost administration | ✅ free, self-hostable |
| Universality | ⚠️ several ballot types supported, but not full multi-constituency complexity |
| Any electoral formula | ⚠️ limited to the voting types the software supports |
| Verifiability and openness | ✅ AGPLv3 license, formally verified protocol security |
| One ballot per voter + revocation | ⚠️ manually configured voter list, no revocation mechanism |
| Ballot secrecy | ✅ homomorphic encryption with distributed trustees |
| Identity verification and delegation | ❌ email / Google account / Inria CAS login only |
| Digital independence | ✅ self-hostable, no vendor lock-in |

A runner-up worth naming is **TrustTheVote / OSET Institute (ElectOS)** (see `@/home/witek/Projekty/suffragio/src/content/docs/market-research/findings.md:281-295` for the full assessment) — the only surveyed system with **zero outright ❌** verdicts, because it is a full election-administration suite (voter registration, tabulation, and voting-device firmware) rather than a narrow cryptographic protocol. It is shallow rather than excellent on most axes (mostly ⚠️), but that breadth across the whole election-administration stack is itself notable.

For a real, government-grade production deployment specifically, **Swiss Post e-voting** is the most rigorously engineered system found in this survey — published cryptographic components, annual public intrusion tests with bug bounties up to CHF 250,000, and both individual and universal verifiability. But it fails digital independence outright (a single vendor holds exclusive rights to the code needed for independent development) and its administration cost is very high.

## How far Belenios is from "perfect"

- **No identity/delegation layer.** Authentication is by email, a Google account, or Inria's CAS — not a strong e-ID — and there is no concept of delegated or liquid voting rights.
- **No revocation.** Voter lists are static, manually configured snapshots; there is no mechanism to revoke a voter's credential mid-election.
- **Narrow universality and electoral-formula support.** Belenios supports the voting types its authors implemented (single/multiple-choice, some referendum variants), not an arbitrary electoral-formula engine — no native support for STV, party-list apportionment, or similar.
- **Trust-model gap.** Homomorphic tallying with distributed trustees gives strong cryptographic secrecy for the content of a vote, but says nothing about voter-eligibility integrity, which remains externally trusted (the organizer-supplied list).

## Justification for building a new system

The gaps above are not superficial missing features that could be patched onto Belenios (or any other single surveyed system) — they stem from fundamental architectural choices:

- **Identity, delegation, and revocation** require a permission/credential layer *underneath* the cryptographic voting layer. Belenios, like Helios and Zeus, was never designed for this: it assumes an externally supplied, static, trusted voter list.
- **Any electoral formula** requires treating tabulation as a pluggable, generic component. Belenios (and its siblings) instead hardcodes a small, fixed set of supported ballot semantics into the same codebase as the cryptography.
- **Digital independence combined with strong identity assurance** is essentially unaddressed by any surveyed system. Systems that achieve independence (self-hosted, open-source, vendor-free) do so by sacrificing identity assurance (Belenios, Helios, Zeus). Systems that achieve strong identity assurance (Estonia, Switzerland) do so by binding tightly to a single nation's PKI/e-ID infrastructure — which is precisely what digital independence rules out.

No surveyed system combines all three of: (a) generic, pluggable electoral-formula support, (b) an identity, delegation, and revocation layer, and (c) full cryptographic verifiability and secrecy while remaining self-hostable and free of single-vendor or single-nation lock-in. That combination is exactly the gap Suffragio's requirements are designed to close — see [Motivation & Requirements](/suffragio-spec/motivation/) for the full requirement set and rationale.
