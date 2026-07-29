---
title: Summary
description: A condensed summary of the e-voting market research against Suffragio's requirements.
---

This section compares Suffragio's requirements (see [Motivation & Requirements](/suffragio-spec/motivation/)) against existing e-voting/i-voting systems on the market — both open-source/academic projects and government or commercial platforms used in real elections around the world.

The full write-up with per-requirement verdicts and sources is in [Findings](/suffragio-spec/market-research/findings/), and a tabular view is in the [Comparison Matrix](/suffragio-spec/market-research/matrix/).

## Condensed requirements list

1. **Low-cost administration** — an organizer can run an election without significant cost.
2. **Universality** — any question/ballot type (referendums, party lists, scales, different constituencies).
3. **Any electoral formula** — results computed from any formula, without building a separate system.
4. **Verifiability and openness** — full access to cast ballots and to the source code of every component.
5. **One ballot per eligible voter + revocation** — one-time ballot issuance, ability to revoke voting rights.
6. **Ballot secrecy** — no one, including the government, can determine how anyone voted.
7. **Identity verification and delegation** — electronic and in-person verification, fine-grained permission model.
8. **Digital independence** — open protocols, free/copyleft license, no single-vendor or infrastructure lock-in.

(The full, unabridged requirements list — including full auditability, software integrity, and public electoral rolls — is in [Motivation & Requirements](/suffragio-spec/motivation/).)

## Systems surveyed (45)

Below is the list of all surveyed systems with a one-sentence summary of each. See [Findings](/suffragio-spec/market-research/findings/) for the detailed per-requirement assessment and sources.

### Open-source / academic / civic-tech projects

1. **Helios Voting** — web-based, E2E-verifiable system built on homomorphic encryption, used for academic and association elections.
2. **Belenios** — French academic successor to Helios with formally verified protocol security.
3. **Zeus** — Greek fork of Helios used by Greek universities and trade unions.
4. **Microsoft ElectionGuard** — open-source cryptographic SDK (homomorphic encryption) meant to be embedded into existing voting machines.
5. **STAR-Vote** — abandoned Travis County (Texas) project meant to combine a DRE with E2E verifiability.
6. **Scantegrity II** — academic E2E-verifiable system based on paper ballots with hidden confirmation codes, used in Takoma Park, Maryland.
7. **Prêt à Voter** — UK academic concept of a cryptographic, tear-off ballot form.
8. **vVote** — real-world deployment of Prêt à Voter in the 2014 Victoria (Australia) state election.
9. **Civitas** — Cornell academic system designed for coercion resistance.
10. **Selene** — academic E2E protocol using pseudonyms to simplify voter-side verification.
11. **DEMOS / DEMOS-2** — Greek academic E2E-verifiable protocols.
12. **CIVS** (Cornell) — simple, open-source Condorcet-method polling engine.
13. **Agora Voting** — Spanish open-source, blockchain-based system used in Podemos primaries.
14. **Decidim** — Barcelona's open-source citizen-participation platform (participatory budgets).
15. **Sovereign (Democracy Earth)** — experimental open-source blockchain "liquid democracy" platform.
16. **Follow My Vote** — unfinished blockchain-voting project.
17. **VotingWorks (VxSuite)** — nonprofit, open-source hardware/software for scanning paper ballots in the US.
18. **TrustTheVote / OSET Institute** — open-source election administration suite (ElectOS), partially used in Los Angeles County.
19. **Free & Fair / ColoradoRLA** — open-source tools for statistical risk-limiting election audits.
20. **OpaVote** — commercial SaaS for preferential voting for organizations.
21. **ElectionBuddy** — commercial SaaS similar to OpaVote.
22. **Simply Voting** — Canadian commercial SaaS used by, among others, trade unions.
23. **BigPulse** — commercial SaaS for organizational voting.
24. **Loomio** — open-source group decision-making tool (not strictly electoral).

### Government / commercial platforms used in elections worldwide

25. **Estonia — IVXV** — the only i-voting system used without restrictions in all national elections worldwide, source published on GitHub.
26. **Switzerland — Swiss Post e-voting** — cantonal system with publicly available source code and recurring public intrusion tests.
27. **Geneva — CHVote** — open-source cantonal system, discontinued in 2020.
28. **Norway** — i-voting pilot 2011–2013, discontinued.
29. **New South Wales — iVote** — Australian state system, retired in 2021 after an outage.
30. **France** — i-voting for citizens abroad (vendor Voxaly/Docaposte).
31. **Panama** — i-voting for citizens abroad.
32. **Mexico — INE** — internet voting system for Mexicans living abroad.
33. **Armenia** — i-voting pilots for the diaspora.
34. **United Arab Emirates** — mobile app with facial recognition for Federal National Council elections.
35. **India — EVM (ECIL/BEL)** — offline voting machines with VVPAT, firmware not public.
36. **Brazil — Urna Eletrônica (TSE)** — offline DRE without VVPAT, source available only to accredited auditors.
37. **Belgium** — system with a barcode-scanned printed confirmation.
38. **Venezuela** — DRE supplied by Smartmatic (until 2017) and local vendors.
39. **Philippines — Smartmatic** — optical scanners used since 2010.
40. **USA — Dominion / ES&S / Hart InterCivic / Unisyn / MicroVote / Clear Ballot** — various DRE/optical-scan machines used across states.
41. **South Korea — K-Voting** — used for organizational votes, not national elections.
42. **Canada** — commercial SaaS (Simply Voting and others) used in Ontario municipal elections.
43. **Russia — Moscow blockchain e-voting** — Ethereum/Exonum-based system, broken by researchers in 2019.
44. **Voatz** — US closed-source, blockchain-based mobile app used in pilots; serious security flaws documented.
45. **Kaspersky Polys** — Russian commercial SaaS for blockchain-based voting.
