---
title: Findings
description: Detailed assessment of 45 e-voting systems against Suffragio's requirements, with sources.
---

Below is the complete list of the 45 surveyed systems. For each, we give the country of origin, implementation language (where publicly known), a short description, and an assessment of Suffragio's eight requirements (full names — see [Motivation & Requirements](/suffragio-spec/motivation/)), with links to the sources used for the assessment.

**Verdict legend:** ✅ yes · ❌ no · ⚠️ partial/conditional · ❓ no public information found.

> **Methodology note:** this is a market survey, not a security audit. Where a specific, citable source could not be found during this research for a given verdict, this is explicitly marked as "no verified source" — the verdict then relies on general public knowledge of the project rather than a specific document.

## Open-source / academic / civic-tech projects

### 1. Helios Voting

**Country:** USA (Ben Adida) · **Implementation language:** Python, JavaScript

Web-based, E2E-verifiable system built on homomorphic encryption and mixnets, used for academic elections (e.g. IACR, ACM) and association elections.

- Low-cost administration: ✅ — free, self-hostable web software. ([github.com/benadida/helios-server](https://github.com/benadida/helios-server))
- Universality: ⚠️ — mainly supports single/multiple-choice (approval) voting, no support for complex ballots (party lists, different constituencies). ([heliosvoting.org](http://heliosvoting.org))
- Any electoral formula: ⚠️ — only produces a result for simple methods itself; not a general tabulation engine.
- Verifiability and openness: ✅ — Apache-licensed code, cryptographic E2E verifiability. ([github.com/benadida/helios-server](https://github.com/benadida/helios-server))
- One ballot per voter + revocation: ⚠️ — the voter list is configured manually by the organizer, no built-in revocation mechanism.
- Ballot secrecy: ✅ — homomorphic encryption and mixnets guarantee secrecy. ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Identity verification and delegation: ❌ — no built-in e-ID/in-person verification integration.
- Digital independence: ✅ — open source, self-hostable, no single-vendor dependency. ([github.com/benadida/helios-server](https://github.com/benadida/helios-server))

### 2. Belenios

**Country:** France (Inria) · **Implementation language:** OCaml, JavaScript

Academic E2E-verifiable system partly implementing the Helios-C protocol, with formally verified security.

- Low-cost administration: ✅ — free, a hosted platform is also available. ([belenios.org](https://www.belenios.org/))
- Universality: ⚠️ — supports several ballot types (including referendums), but not the full complexity of multi-constituency national ballots. ([belenios.org](https://www.belenios.org/))
- Any electoral formula: ⚠️ — same limitation, restricted to supported voting types.
- Verifiability and openness: ✅ — AGPLv3 license, formally verified protocol security. ([github.com/glondu/belenios](https://github.com/glondu/belenios))
- One ballot per voter + revocation: ⚠️ — voter list configured manually, no built-in revocation.
- Ballot secrecy: ✅ — homomorphic encryption with distributed trust (trustees). ([belenios.org](https://www.belenios.org/))
- Identity verification and delegation: ❌ — login via email/Google account/Inria CAS, no strong identity verification. ([belenios.org](https://www.belenios.org/))
- Digital independence: ✅ — open source (AGPLv3), self-hostable. ([github.com/glondu/belenios](https://github.com/glondu/belenios))

### 3. Zeus

**Country:** Greece (GRNET) · **Implementation language:** Python

Fork of Helios developed by the Greek academic network GRNET, used by Greek universities and trade unions; instead of a result, it produces an auditable tally for further processing.

- Low-cost administration: ✅ — free, open source. ([github.com/grnet/zeus](https://github.com/grnet/zeus))
- Universality: ⚠️ — supports more voting systems than Helios by decoupling vote collection from tabulation, but still limited to simple ballots. ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Any electoral formula: ⚠️ — Zeus deliberately only produces a tally that can be fed into any external tabulation engine. ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Verifiability and openness: ✅ — open source, cryptographically verifiable. ([github.com/grnet/zeus](https://github.com/grnet/zeus))
- One ballot per voter + revocation: ⚠️ — as in Helios, manual voter list configuration.
- Ballot secrecy: ✅ — inherits Helios's cryptography (mixnets/homomorphic encryption). ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Identity verification and delegation: ❌ — no built-in e-ID integration.
- Digital independence: ✅ — open source, self-hostable. ([github.com/grnet/zeus](https://github.com/grnet/zeus))

### 4. Microsoft ElectionGuard

**Country:** USA (Microsoft, with Galois) · **Implementation language:** C++, C#, Python

An open-source cryptographic SDK (not a full election system) using homomorphic encryption, meant to be embedded into existing voting machines to enable E2E verifiability and ballot-comparison audits.

- Low-cost administration: ⚠️ — the SDK itself is free, but requires integration with existing hardware/election systems. ([github.com/microsoft/electionguard](https://github.com/microsoft/electionguard))
- Universality: ❓ — depends on the system it's embedded into; the SDK itself imposes no ballot-type restrictions.
- Any electoral formula: ❓ — likewise, dependent on integration.
- Verifiability and openness: ✅ — MIT license, full source code of the cryptographic layer. ([electionguard.vote](https://www.electionguard.vote/), [github.com/microsoft/electionguard](https://github.com/microsoft/electionguard))
- One ballot per voter + revocation: ❌ — out of scope for the SDK (handled by the system it's embedded into).
- Ballot secrecy: ✅ — homomorphic encryption guarantees secrecy of encrypted ballots. ([electionguard.vote](https://www.electionguard.vote/))
- Identity verification and delegation: ❌ — out of scope for the SDK.
- Digital independence: ✅ — open source (MIT), no licensing fees. ([github.com/microsoft/electionguard](https://github.com/microsoft/electionguard))

### 5. STAR-Vote

**Country:** USA (Travis County, Texas) · **Implementation language:** unknown (project abandoned before full implementation)

A planned system combining a DRE with commercial-off-the-shelf hardware and E2E verifiability, based on concepts related to Helios/Scantegrity; abandoned before deployment.

- Low-cost administration: ❓ — the project was never completed.
- Universality: ❓ — unknown, project abandoned.
- Any electoral formula: ❓ — unknown.
- Verifiability and openness: ⚠️ — planned as open source, but never fully deployed or published. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- One ballot per voter + revocation: ❓ — unknown.
- Ballot secrecy: ✅ — assumed by design (E2E-verifiable concept). ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Identity verification and delegation: ❓ — unknown.
- Digital independence: ❓ — project abandoned before deployment, no published code. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))

### 6. Scantegrity II

**Country:** USA (academic, University of Waterloo / Rice University) · **Implementation language:** not publicly known

An E2E-verifiable system based on paper ballots with a hidden confirmation code revealed by marking a candidate with a special pen; actually used in the Takoma Park (Maryland, USA) municipal elections of 2009 and 2011.

- Low-cost administration: ✅ — based on standard paper-ballot scanning hardware.
- Universality: ❌ — designed and used only for simple, local single-choice elections.
- Any electoral formula: ❌ — not a general tabulation engine.
- Verifiability and openness: ✅ — E2E-verifiable concept, published academically.
- One ballot per voter + revocation: ❓ — no verified source found this session.
- Ballot secrecy: ✅ — preserving ballot secrecy is one of the system's design goals.
- Identity verification and delegation: ❌ — does not cover the identity layer, relies on the traditional polling-place process.
- Digital independence: ✅ — academic concept, no vendor lock-in.

> Note: no directly citable online source for Scantegrity II could be found this session — the above assessment relies on general academic knowledge of the system.

### 7. Prêt à Voter

**Country:** United Kingdom (University of Surrey) · **Implementation language:** concept/reference prototypes, various languages

A concept for a cryptographic, tear-off ballot form with a randomly permuted candidate list — the voter marks the ballot on paper, the tear-off list is destroyed, and the remainder is scanned as an encrypted, verifiable vote.

- Low-cost administration: ❓ — a research concept, no data on the cost of a large-scale real deployment.
- Universality: ❌ — originally designed for single-choice voting; extensions to other formulas required separate research work. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Any electoral formula: ⚠️ — extended to include STV/IRV as part of the Victorian adaptation (see vVote), but not a universal engine. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Verifiability and openness: ✅ — academic publications describing the full cryptographic protocol. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- One ballot per voter + revocation: ❓ — the concept focuses on vote verifiability, not permission management.
- Ballot secrecy: ✅ — a key design feature (the tear-off candidate list is destroyed before leaving the polling place). ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Identity verification and delegation: ❌ — out of scope, assumes a traditional polling-place process.
- Digital independence: ✅ — academic concept, publicly described, no vendor lock-in.

### 8. vVote

**Country:** Australia (State of Victoria; team: University of Surrey, Melbourne, Luxembourg) · **Implementation language:** Java (among others)

A real-world deployment of Prêt à Voter in the November 2014 Victoria state election — the first worldwide use of an E2E-verifiable system in a binding political election, supporting blind and remote voters among others.

- Low-cost administration: ❌ — required a dedicated academic team and roughly two years of development for a single election. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Universality: ❌ — specifically adapted to Victoria's electoral system (IRV + STV), not a general-purpose universal system. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Any electoral formula: ⚠️ — supports IRV and STV with above/below-the-line lists, but required dedicated adaptation work. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Verifiability and openness: ✅ — open-source code, published after the election. ([arxiv.org/abs/1404.6822 — code: bitbucket.org/vvote](https://doi.org/10.48550/arxiv.1404.6822))
- One ballot per voter + revocation: ⚠️ — handled procedurally by the electoral commission (VEC), not cryptographically. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Ballot secrecy: ✅ — full secrecy per the Prêt à Voter design, confirmed in a real deployment. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Identity verification and delegation: ⚠️ — traditional in-person verification (the system was only available at supervised polling places). ([past.electionwatch.edu.au](http://past.electionwatch.edu.au/victoria-2014/click-here-democracy-e-vote-explained))
- Digital independence: ❌ — dependent on contracts with specific vendors (University of Surrey as "SuVote", Cryptoworkshop.com for the mixnet). ([arxiv.org/abs/1404.6822](https://doi.org/10.48550/arxiv.1404.6822))

### 9. Civitas

**Country:** USA (Cornell University) · **Implementation language:** Java

An academic E2E-verifiable system specifically designed for coercion resistance — addressing Suffragio's optional coercion-prevention goal. Never used in a binding election.

- Low-cost administration: ❓ — no verified source this session regarding deployment costs.
- Universality: ❌ — focused on simple votes, not complex multi-constituency ballots.
- Any electoral formula: ❌ — not a general tabulation engine.
- Verifiability and openness: ✅ — academic publications with a full protocol description.
- One ballot per voter + revocation: ❓ — no verified source this session.
- Ballot secrecy: ✅ — ballot secrecy and coercion resistance are the system's main design goals.
- Identity verification and delegation: ❌ — out of scope for the research project.
- Digital independence: ✅ — academic concept, no vendor lock-in.

> Note: no directly citable online source for Civitas could be found this session — the assessment relies on general academic knowledge of the project.

### 10. Selene

**Country:** academic (Europe, incl. IT University of Copenhagen) · **Implementation language:** concept/research prototypes

An academic E2E-verifiable protocol using voter pseudonyms to simplify verification without needing to track complex cryptographic proofs.

- Low-cost administration: ❓ — research concept.
- Universality: ❌ — focused on simplifying verification of simple votes.
- Any electoral formula: ❌ — not a general tabulation engine.
- Verifiability and openness: ✅ — academic publications with a full protocol description.
- One ballot per voter + revocation: ❓ — outside the main research scope.
- Ballot secrecy: ✅ — preserving secrecy while enabling easy verifiability is the core design goal.
- Identity verification and delegation: ❌ — out of scope for the concept.
- Digital independence: ✅ — academic concept, publicly described.
- Sources: no verified link found this session — assessment based on general academic knowledge of the Selene protocol.

### 11. DEMOS / DEMOS-2

**Country:** Greece (academic) · **Implementation language:** concept/research prototypes

Greek academic E2E-verifiable protocols, developed in parallel to the Helios/Zeus family, with an alternative approach to cryptographic proofs avoiding the random-oracle model.

- Low-cost administration: ❓ — research concept.
- Universality: ❌ — focused on simple votes.
- Any electoral formula: ❌ — not a general tabulation engine.
- Verifiability and openness: ✅ — academic publications with a full description.
- One ballot per voter + revocation: ❓ — outside the main research scope.
- Ballot secrecy: ✅ — main design goal.
- Identity verification and delegation: ❌ — out of scope for the concept.
- Digital independence: ✅ — academic concept.
- Sources: no verified link found this session — assessment based on general academic knowledge of the DEMOS protocol family.

### 12. CIVS (Condorcet Internet Voting System)

**Country:** USA (Cornell University) · **Implementation language:** Perl (historically)

A simple, free Condorcet-method polling and voting engine, used by organizations and open-source communities for internal votes, not for binding public elections.

- Low-cost administration: ✅ — free web tool. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Universality: ❌ — Condorcet-method ranked polls only, no support for complex ballots. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Any electoral formula: ❌ — supports the Condorcet method only.
- Verifiability and openness: ❌ — no cryptographic E2E verifiability; trust is placed in the service operator.
- One ballot per voter + revocation: ⚠️ — a one-time link/email per participant, no formal permission management.
- Ballot secrecy: ⚠️ — email-based pseudonymization, not cryptographic secrecy.
- Identity verification and delegation: ❌ — none.
- Digital independence: ⚠️ — historically ran mainly as a centrally hosted service at Cornell, not a typical self-hosted open-source deployment.
- Sources: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 13. Agora Voting

**Country:** Spain · **Implementation language:** Python, JavaScript (declared open source)

A Spanish open-source blockchain-based system, used in Podemos party primaries and some citizen consultations.

- Low-cost administration: ✅ — declared free and self-hostable.
- Universality: ⚠️ — supports several voting types, but not the full complexity of multi-constituency ballots.
- Any electoral formula: ⚠️ — limited to supported methods.
- Verifiability and openness: ⚠️ — declared open source, but without an independent cryptographic audit comparable to Helios.
- One ballot per voter + revocation: ⚠️ — deployment-dependent.
- Ballot secrecy: ⚠️ — claimed, but not independently cryptographically verified this session.
- Identity verification and delegation: ❌ — no built-in strong e-ID integration.
- Digital independence: ✅ — open source, self-hosted.
- Sources: no verified link found this session — assessment based on general public knowledge of the Agora Voting project.

### 14. Decidim

**Country:** Spain (Barcelona en Comú / City of Barcelona) · **Implementation language:** Ruby on Rails

An open-source citizen-participation platform (consultations, participatory budgets, simple votes), used by many cities and institutions in Europe.

- Low-cost administration: ✅ — free, self-hosted, AGPL license.
- Universality: ⚠️ — handles simple votes/participatory budgets well, but not designed for full popular elections (party lists, different constituencies).
- Any electoral formula: ❌ — not a general tabulation engine for formal electoral formulas.
- Verifiability and openness: ⚠️ — fully open-source code, but votes are not cryptographically E2E-verifiable.
- One ballot per voter + revocation: ❌ — permission management is part of the participation platform, not a dedicated election system.
- Ballot secrecy: ⚠️ — depends on the voting module's configuration, no cryptographic guarantee.
- Identity verification and delegation: ⚠️ — supports various login/verification methods depending on the institution's deployment.
- Digital independence: ✅ — open source (AGPL), self-hosted, large community.
- Sources: [handwiki.org — Comparison of civic technology platforms](https://handwiki.org/wiki/Software:Comparison_of_civic_technology_platforms)

### 15. Sovereign (Democracy Earth)

**Country:** USA/international (Democracy Earth Foundation) · **Implementation language:** JavaScript/blockchain (Bitcoin-based)

An experimental, open-source "liquid democracy" platform built on blockchain technology, meant to increase trust in the political process; no widespread binding deployments.

- Low-cost administration: ✅ — declared free and open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Universality: ❌ — focused on vote-delegation (liquid democracy), not classic ballots.
- Any electoral formula: ❌ — not a general tabulation engine for traditional formulas.
- Verifiability and openness: ✅ — open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- One ballot per voter + revocation: ❓ — no verified source this session.
- Ballot secrecy: ❓ — blockchain mechanisms can complicate full secrecy without added cryptography; no verified source.
- Identity verification and delegation: ❌ — no strong e-ID integration.
- Digital independence: ✅ — open source, no vendor lock-in.
- Sources: [osvtac.github.io — State of the Art Briefing on Sovereign/Democracy Earth](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 16. Follow My Vote

**Country:** USA (Virginia) · **Implementation language:** C++/blockchain

An unfinished blockchain-voting project intended to offer fully open-source, E2E-verifiable voting; remained at the proof-of-concept stage.

- Low-cost administration: ❓ — the project never reached production maturity. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Universality: ❓ — unknown, no completed implementation.
- Any electoral formula: ❓ — unknown.
- Verifiability and openness: ⚠️ — declared open source (MIT/Unlicense), but incomplete. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- One ballot per voter + revocation: ❓ — unknown.
- Ballot secrecy: ❓ — unknown, no completed deployment.
- Identity verification and delegation: ❌ — not implemented.
- Digital independence: ✅ — declared open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Sources: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 17. VotingWorks (VxSuite)

**Country:** USA · **Implementation language:** TypeScript, Rust (open hardware)

A nonprofit providing open-source hardware and software for scanning paper ballots at polling places (not internet voting), used in Mississippi and New Hampshire counties, among others.

- Low-cost administration: ✅ — a cheaper alternative to proprietary DRE/scanner systems. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Universality: ⚠️ — handles typical American multi-race ballots, but designed for in-person, not internet, voting.
- Any electoral formula: ⚠️ — supports several tabulation methods via optical scanning, but not a universal engine for any electoral formula.
- Verifiability and openness: ✅ — hardware and software fully open source (AGPL). ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- One ballot per voter + revocation: ❌ — does not manage voter registration/permissions, only in-person ballot scanning.
- Ballot secrecy: ✅ — paper ballots scanned offline, unlinked to identity.
- Identity verification and delegation: ❌ — out of scope (handled by the local election administration).
- Digital independence: ✅ — open source, independently extensible. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Sources: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 18. TrustTheVote / OSET Institute (ElectOS)

**Country:** USA (Palo Alto, CA) · **Implementation language:** Ruby, Java, PHP, C#

An open-source election administration suite (voter registration, tabulation, voting-device firmware), partially used in Los Angeles County (VSAP project).

- Low-cost administration: ⚠️ — free software, but deployment (as in LA County) required dedicated, costly hardware (BMDs). ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Universality: ⚠️ — handles typical American multi-race ballots, not tested for full universality (referendums, scales, etc.).
- Any electoral formula: ⚠️ — limited to formulas typical of US elections.
- Verifiability and openness: ✅ — open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- One ballot per voter + revocation: ⚠️ — includes a voter registration module, but dependent on integration with local registries.
- Ballot secrecy: ⚠️ — depends on deployment mode (BMD + offline scanner).
- Identity verification and delegation: ⚠️ — a registration module exists, but no strong e-ID integration outside the US context.
- Digital independence: ✅ — open source, though high hardware cost limits ease of independent deployment. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Sources: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 19. Free & Fair / ColoradoRLA

**Country:** USA (Galois, Portland OR) · **Implementation language:** Java

Open-source tools for statistical risk-limiting election audits, used by the state of Colorado among others; not a full election system, just a supporting tool.

- Low-cost administration: ✅ — free audit tool. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Universality: ❌ — a supporting audit tool, not an election-running system.
- Any electoral formula: ❌ — out of scope for the tool.
- Verifiability and openness: ✅ — fully open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- One ballot per voter + revocation: ❌ — out of scope for the tool.
- Ballot secrecy: ❌ — out of scope for the tool (operates on already tallied/scanned votes).
- Identity verification and delegation: ❌ — out of scope for the tool.
- Digital independence: ✅ — open source, independently deployable. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Sources: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 20–23. Commercial SaaS for organizational voting: OpaVote, ElectionBuddy, Simply Voting, BigPulse

**Country:** USA/Canada · **Implementation language:** not publicly known (closed services)

Four similar commercial SaaS platforms for running preferential/ranked votes for organizations, associations, and trade unions (not for binding public elections). Simply Voting is used by, among others, some organizations in Canada.

- Low-cost administration: ✅ — low subscription prices, easy to set up for small organizations.
- Universality: ⚠️ — support various preferential methods, but not the full complexity of popular elections (different constituencies, party lists).
- Any electoral formula: ⚠️ — a few built-in tabulation methods (incl. STV), but a closed catalog of options.
- Verifiability and openness: ❌ — closed source code, trust placed in the service provider.
- One ballot per voter + revocation: ⚠️ — handled via a participant list uploaded by the organizer, no formal revocation mechanism.
- Ballot secrecy: ⚠️ — claimed by the provider, but not independently cryptographically verified.
- Identity verification and delegation: ❌ — usually just a one-time email/link, no strong e-ID integration.
- Digital independence: ❌ — closed, hosted exclusively by the provider, no self-hosting option.
- Sources: no verified link found this session for any of the four services — assessment based on general public knowledge of the organizational-voting SaaS category.

### 24. Loomio

**Country:** New Zealand · **Implementation language:** Ruby on Rails

An open-source tool for collaborative group decision-making (discussion + simple voting), used by civil-society organizations and some local governments; not a dedicated election system.

- Low-cost administration: ✅ — open source, a hosted version is also available.
- Universality: ❌ — designed for simple group/consensus votes, not formal popular elections.
- Any electoral formula: ❌ — not a tabulation engine for formal electoral formulas.
- Verifiability and openness: ⚠️ — open-source code, but no cryptographic E2E vote verifiability.
- One ballot per voter + revocation: ❌ — permission management limited to discussion-group membership.
- Ballot secrecy: ⚠️ — votes can be public or anonymous depending on configuration, without a cryptographic guarantee.
- Identity verification and delegation: ❌ — no strong e-ID integration.
- Digital independence: ✅ — open source, self-hosted.
- Sources: [handwiki.org — Comparison of civic technology platforms](https://handwiki.org/wiki/Software:Comparison_of_civic_technology_platforms)

## Government / commercial platforms used in elections worldwide

### 25. Estonia — IVXV (i-Voting)

**Country:** Estonia (RIA — Estonian Information System Authority) · **Implementation language:** Go, Java, Python, Android/iOS (verification)

The world's only i-voting system used without restriction in all types of national elections, since 2005 (current generation IVXV since 2017). Source code published on GitHub.

- Low-cost administration: ❓ — no verified source this session regarding operational costs.
- Universality: ✅ — used for all types of national and local elections in Estonia. ([valimised.ee — e-voting in other countries](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries))
- Any electoral formula: ⚠️ — adapted to Estonia's electoral system, not a general engine configurable for any electoral formula.
- Verifiability and openness: ⚠️ — source code published on GitHub for public inspection, but development is overseen by the state election office (not a fully open, community-driven development model). ([github.com/valimised/ivxv](https://github.com/valimised/ivxv), [valimised.ee — documents about internet voting](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting))
- One ballot per voter + revocation: ✅ — a voter may vote multiple times (the last vote counts), integration with the Estonian population registry ensures permission verification. ([valimised.ee](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting))
- Ballot secrecy: ⚠️ — criticized by security researchers for potential client-device attack vectors, despite no confirmed impact on election results.
- Identity verification and delegation: ✅ — strong integration with the Estonian electronic ID card (ID-kaart) and mobile-ID. ([valimised.ee](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting))
- Digital independence: ❌ — tightly dependent on Estonia's PKI/e-ID infrastructure, impossible to deploy without this infrastructure in another country. ([github.com/valimised](https://github.com/valimised))
- Sources: [github.com/valimised/ivxv](https://github.com/valimised/ivxv), [valimised.ee — documents about internet voting](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting), [github.com/valimised/ivotingverification](https://github.com/valimised/ivotingverification)

### 26. Switzerland — Swiss Post e-voting

**Country:** Switzerland (cantons Basel-Stadt, St. Gallen, Thurgau, since 2023) · **Implementation language:** Java, TypeScript

Successor system to Scytl/CHVote developed independently since 2019 by Swiss Post, with publicly available code and regular annual penetration tests ("public intrusion tests") offering bounties up to CHF 250,000.

- Low-cost administration: ❌ — very high certification and maintenance costs (bug bounties reaching hundreds of thousands of francs, a dedicated cryptography team). ([swisspost-digital.ch — source code](https://swisspost-digital.ch/en/solutions/e-voting/publications-and-source-code))
- Universality: ✅ — used for referendums and federal/cantonal elections of varying structure. ([swisspost-digital.ch](https://swisspost-digital.ch/en/digital-blog/e-government/the-source-code-of-the-future-e-voting-system-is-publicly-accessible))
- Any electoral formula: ⚠️ — adapted to the Swiss electoral system, not declared as a universal engine.
- Verifiability and openness: ✅ — key components (cryptography, verifier) published under Apache 2, full public documentation, reproducible test environment. ([gitlab.com/swisspost-evoting/e-voting/e-voting](https://gitlab.com/swisspost-evoting/e-voting/e-voting), [security whitepaper](https://gitlab.com/swisspost-evoting/e-voting/e-voting-documentation/-/blob/master/Product/Security%20Whitepaper%20of%20the%20Swiss%20Post%20Voting%20System.md))
- One ballot per voter + revocation: ✅ — integrated with cantonal voter registries. ([swisspost-digital.ch — FAQ](https://swisspost-digital.ch/en/evoting-community/help-and-contact/faq))
- Ballot secrecy: ⚠️ — declared full secrecy (individual and universal verifiability), but the previous version (developed with Scytl) had a serious cryptographic flaw detected in 2019 before production deployment. ([swisspost-digital.ch — publications and code](https://swisspost-digital.ch/en/solutions/e-voting/publications-and-source-code))
- Identity verification and delegation: ✅ — integrated with voting cards sent by mail together with verification codes. ([swisspost-digital.ch — FAQ](https://swisspost-digital.ch/en/evoting-community/help-and-contact/faq))
- Digital independence: ❌ — a single vendor (Swiss Post) holds exclusive rights to the code needed for independent system development. ([swisspost-digital.ch — FAQ](https://swisspost-digital.ch/en/evoting-community/help-and-contact/faq))
- Sources: [swisspost-digital.ch — publications and source code](https://swisspost-digital.ch/en/solutions/e-voting/publications-and-source-code), [gitlab.com/swisspost-evoting](https://gitlab.com/swisspost-evoting/e-voting/e-voting), [security whitepaper](https://gitlab.com/swisspost-evoting/e-voting/e-voting-documentation/-/blob/master/Product/Security%20Whitepaper%20of%20the%20Swiss%20Post%20Voting%20System.md)

### 27. Geneva — CHVote

**Country:** Switzerland (Canton of Geneva) · **Implementation language:** Java (per the canton's repository)

An open-source cantonal system developed since 2003, publishing code on the Canton of Geneva's GitHub; retired in 2020 for budgetary reasons and security concerns.

- Low-cost administration: ❌ — the canton discontinued the system partly due to high maintenance costs.
- Universality: ✅ — used for referendums and cantonal elections of varying structure for over a decade. ([valimised.ee — e-voting in other countries](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries))
- Any electoral formula: ⚠️ — adapted to the Swiss/Geneva electoral system.
- Verifiability and openness: ✅ — code published openly on the canton's GitHub. ([republique-et-canton-de-geneve.github.io/chvote-1-0](https://republique-et-canton-de-geneve.github.io/chvote-1-0/index-en.html))
- One ballot per voter + revocation: ✅ — integrated with the cantonal voter registry.
- Ballot secrecy: ⚠️ — declared, but the canton withdrew due to security concerns about the entire family of Swiss systems in 2019. ([valimised.ee](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries))
- Identity verification and delegation: ✅ — integrated with voting cards and verification codes.
- Digital independence: ❌ — despite open code, the system was tightly tied to the Geneva canton's infrastructure and was ultimately abandoned.
- Sources: [republique-et-canton-de-geneve.github.io/chvote-1-0](https://republique-et-canton-de-geneve.github.io/chvote-1-0/index-en.html), [valimised.ee — e-voting in other countries](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries)

### 28. Norway (2011–2013 pilot)

**Country:** Norway · **Implementation language:** not publicly known (vendors: Scytl, ErgoGroup)

An i-voting pilot run in selected municipalities in the 2011 local and 2013 parliamentary elections, discontinued after two cycles due to political and technical concerns.

- Low-cost administration: ❓ — no verified source this session regarding costs.
- Universality: ⚠️ — limited to selected municipalities and election types covered by the pilot.
- Any electoral formula: ⚠️ — adapted to the Norwegian electoral formula.
- Verifiability and openness: ⚠️ — some code published as a public-contract condition, but the system was not fully open source.
- One ballot per voter + revocation: ✅ — integrated with the Norwegian voter registry, allowed repeat voting overwriting the previous vote.
- Ballot secrecy: ❓ — no verified source this session.
- Identity verification and delegation: ✅ — integration with Norwegian electronic identification infrastructure.
- Digital independence: ❌ — dependent on external vendors (Scytl, ErgoGroup), ultimately discontinued.
- Sources: [valimised.ee — e-voting in other countries](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 29. New South Wales (Australia) — iVote

**Country:** Australia (NSW) · **Implementation language:** not publicly known (vendor: Scytl, later internal development)

An internet-voting system used 2011–2021, mainly for disabled voters, those living far from polling places, and voters abroad; withdrawn after a system failure in the December 2021 local elections.

- Low-cost administration: ❌ — high maintenance costs over more than a decade without resolving technical issues.
- Universality: ⚠️ — limited to specific voter categories (disabled, remote areas, overseas voters), not for the general population. ([rappler.com](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/))
- Any electoral formula: ⚠️ — adapted to NSW's Australian preferential electoral formula.
- Verifiability and openness: ❌ — closed source code (owned by the vendor/NSW electoral commission).
- One ballot per voter + revocation: ✅ — integrated with the NSW voter registry.
- Ballot secrecy: ❓ — no verified independent source this session.
- Identity verification and delegation: ⚠️ — verification via personal data submitted online, without strong e-ID.
- Digital independence: ❌ — a closed single-vendor system, ultimately withdrawn after the 2021 failure. ([rappler.com](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/))
- Sources: [rappler.com — Which countries have conducted online elections](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 30. France — internet voting for citizens abroad

**Country:** France (vendor: Voxaly/Docaposte) · **Implementation language:** not publicly known

An internet-voting system for French citizens living abroad, used in elections to the National Assembly (foreign constituencies) and partially for consular elections.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ❌ — limited exclusively to foreign-citizen constituencies. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Any electoral formula: ⚠️ — adapted to the French electoral formula for foreign constituencies.
- Verifiability and openness: ❌ — proprietary system, non-public code.
- One ballot per voter + revocation: ✅ — integrated with the French consular voter registry for citizens abroad.
- Ballot secrecy: ❓ — no verified source this session.
- Identity verification and delegation: ✅ — verification via the consular registry/personal data.
- Digital independence: ❌ — dependent on a single private vendor.
- Sources: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 31–32. Panama and Mexico (INE) — internet voting for citizens abroad

**Country:** Panama, Mexico · **Implementation language:** not publicly known

Internet-voting systems enabling citizens living abroad to cast a ballot without returning to the country or visiting a consular office.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ❌ — limited exclusively to voters abroad. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Any electoral formula: ⚠️ — adapted to local electoral formulas.
- Verifiability and openness: ❌ — proprietary systems; Mexico's INE publishes some audit documentation, but not the full source code.
- One ballot per voter + revocation: ✅ — integrated with voter registries for citizens abroad.
- Ballot secrecy: ❓ — no verified source this session.
- Identity verification and delegation: ✅ — verification via the consular registry/identity documents.
- Digital independence: ❌ — dependent on external vendors/state infrastructure closed to public audit.
- Sources: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 33. Armenia — internet-voting pilots for the diaspora

**Country:** Armenia · **Implementation language:** not publicly known

Pilot internet-voting deployments for the Armenian diaspora, listed as one of the "established cases" of i-voting in the International IDEA report.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ❌ — limited to the diaspora/voters abroad. ([idea.int — database](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349))
- Any electoral formula: ❓ — no verified source this session.
- Verifiability and openness: ❓ — very limited public technical documentation.
- One ballot per voter + revocation: ❓ — no verified source this session.
- Ballot secrecy: ❓ — no verified source this session.
- Identity verification and delegation: ❓ — no verified source this session.
- Digital independence: ❓ — no verified source this session.
- Sources: [idea.int — Database: internet voting systems by country](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 34. United Arab Emirates — mobile voting app

**Country:** UAE · **Implementation language:** not publicly known

A mobile app with facial-recognition functionality used for elections to the Federal National Council, for a limited electorate eligible to vote.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ❌ — a very limited electorate (the Federal National Council has limited powers and electorate), not applicable to full popular elections. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Any electoral formula: ❌ — adapted exclusively to the UAE's specific, limited electoral formula.
- Verifiability and openness: ❌ — proprietary system, no public source code.
- One ballot per voter + revocation: ✅ — integrated with the registry of eligible voters.
- Ballot secrecy: ❓ — no verified source this session.
- Identity verification and delegation: ✅ — strong biometric verification (facial recognition). ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Digital independence: ❌ — closed government system.
- Sources: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650)

### 35. India — EVM (ECIL/BEL)

**Country:** India (manufactured by Bharat Electronics Limited and Electronics Corporation of India Limited) · **Implementation language:** non-public firmware burned onto one-time-programmable microcontrollers

Standalone, non-networked Electronic Voting Machines used in all national elections since the 1990s, universally equipped with VVPAT (paper confirmation) since 2017.

- Low-cost administration: ❓ — no verified source this session regarding total cost (manufacturing and maintenance of a fleet of millions of machines).
- Universality: ❌ — supports only simple single-candidate button voting, no support for party lists, scales, or multi-option referendums. ([Supreme Court of India ruling, 26.04.2024](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf))
- Any electoral formula: ❌ — designed exclusively for India's FPTP system.
- Verifiability and openness: ❌ — the Supreme Court of India has repeatedly ruled that the EVM source code may not be disclosed publicly. ([indianexpress.com](https://preprod.indianexpress.com/article/india/supreme-court-vvpat-order-evm-9287698/), [thehindu.com](https://www.thehindu.com/news/national/sc-asks-ec-five-queries-in-evm-vvpat-case-is-a-microcontroller-used-in-evms-is-one-time-programmable-or-not/article68100796.ece))
- One ballot per voter + revocation: ✅ — a single button press verified by a polling-station official against the voter list.
- Ballot secrecy: ✅ — the machines operate fully offline, without network connectivity, eliminating remote de-anonymization vectors. ([Supreme Court of India ruling](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf))
- Identity verification and delegation: ✅ — traditional verification against the voter list at the polling station by an election official.
- Digital independence: ❌ — closed, one-time-programmable firmware produced exclusively by two state-owned companies, deliberately kept secret by court order. ([Supreme Court of India ruling](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf))
- Sources: [adrindia.org — Supreme Court of India ruling on EVM/VVPAT (26.04.2024)](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf), [indianexpress.com — SC says source code cannot be disclosed](https://preprod.indianexpress.com/article/india/supreme-court-vvpat-order-evm-9287698/), [thehindu.com](https://www.thehindu.com/news/national/sc-asks-ec-five-queries-in-evm-vvpat-case-is-a-microcontroller-used-in-evms-is-one-time-programmable-or-not/article68100796.ece)

### 36. Brazil — Urna Eletrônica (TSE)

**Country:** Brazil (Tribunal Superior Eleitoral) · **Implementation language:** historically proprietary VirtuOS/Windows CE, GNU/Linux-based since 2008

Standalone DRE machines (offline during voting) used at all polling places since 2000; have no paper trail (VVPAT rejected by the TSE).

- Low-cost administration: ❓ — no verified source this session regarding total fleet cost.
- Universality: ⚠️ — handles different levels of elections (presidential, parliamentary, local) via candidate-number entry, but doesn't support multi-option referendums or rating scales. ([tse.jus.br — auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability))
- Any electoral formula: ⚠️ — designed for Brazil's open-list and majoritarian system, not a universal engine.
- Verifiability and openness: ❌ — code shared only with accredited auditors under NDA, in a controlled, internet-free environment, without publishing audit results. ([ndi.org — Brazil case study](https://www.ndi.org/sites/default/files/4_Brazil.pdf), [tse.jus.br — auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability))
- One ballot per voter + revocation: ✅ — verification against the Brazilian voter registry at the polling place.
- Ballot secrecy: ✅ — machines offline during voting, votes stored in random order unlinked to the voting sequence. ([ndi.org — Brazil case study](https://www.ndi.org/sites/default/files/4_Brazil.pdf))
- Identity verification and delegation: ✅ — traditional verification at the polling place.
- Digital independence: ❌ — a single, closed system maintained centrally by the TSE, no possibility of independent deployment. ([tse.jus.br — auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability))
- Sources: [international.tse.jus.br — Auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability), [ndi.org — Case Study Report on Brazil Electronic Voting](https://www.ndi.org/sites/default/files/4_Brazil.pdf), [tse.jus.br — Resolução nº 23.673/2021](https://www.tse.jus.br/legislacao/compilada/res/2021/resolucao-no-23-673-14-de-dezembro-de-2021)

### 37. Belgium — electronic ballot-printing machines

**Country:** Belgium · **Implementation language:** not publicly known

A system where the voter makes a selection on a screen and a machine prints a ballot with an encoded barcode, which the voter drops into a ballot box and which is then scanned.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ⚠️ — supports Belgium's party-list system, adapted to the local electoral formula. ([idea.int — database](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349))
- Any electoral formula: ⚠️ — adapted exclusively to Belgium's party-list system.
- Verifiability and openness: ❌ — proprietary system.
- One ballot per voter + revocation: ✅ — verification at the polling place.
- Ballot secrecy: ✅ — the barcode ballot contains no data identifying the voter.
- Identity verification and delegation: ✅ — traditional verification at the polling place.
- Digital independence: ❌ — closed vendor system.
- Sources: [idea.int — Database: voting technology by country](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349)

### 38. Venezuela — Smartmatic and local vendors

**Country:** Venezuela · **Implementation language:** not publicly known

DRE machines with printed paper confirmation, supplied by Smartmatic until 2017, then by other/local vendors in the context of politically disputed elections.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ⚠️ — adapted to Venezuela's electoral system (president, national assembly). ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Any electoral formula: ⚠️ — adapted exclusively to the local electoral formula.
- Verifiability and openness: ❌ — proprietary system, subject to numerous political disputes over audit credibility.
- One ballot per voter + revocation: ✅ — verification against the national voter registry.
- Ballot secrecy: ✅ — DRE machines with paper confirmation, offline during voting.
- Identity verification and delegation: ✅ — biometric (fingerprint) verification at the polling place.
- Digital independence: ❌ — strong dependence on a single, foreign technology vendor, a subject of political controversy. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Sources: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650)

### 39. Philippines — Smartmatic

**Country:** Philippines · **Implementation language:** not publicly known

Optical mark recognition (OMR) ballot scanners supplied by Smartmatic, used in all national elections since 2010.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ⚠️ — adapted to the Philippine electoral system with a very large number of elected positions on a single ballot.
- Any electoral formula: ⚠️ — adapted exclusively to the local electoral formula.
- Verifiability and openness: ❌ — proprietary system.
- One ballot per voter + revocation: ✅ — verification against the national voter registry (COMELEC).
- Ballot secrecy: ✅ — paper ballots scanned offline at the polling place.
- Identity verification and delegation: ✅ — traditional verification at the polling place.
- Digital independence: ❌ — strong dependence on a single foreign vendor.
- Sources: no additional verified link found this session beyond general industry knowledge of the Smartmatic vendor.

### 40. USA — Dominion / ES&S / Hart InterCivic / Unisyn / MicroVote / Clear Ballot

**Country:** USA (various states) · **Implementation language:** not publicly known (closed systems, except partly-open Clear Ballot components)

Various DRE machines and optical scanners used across individual US counties/states; most states now require a paper trail (VVPAT) and some conduct statistical risk-limiting audits.

- Low-cost administration: ❌ — very high costs of election contracts in the US. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Universality: ⚠️ — support typical American multi-race ballots, different states have different configurations.
- Any electoral formula: ⚠️ — limited to formulas used in US elections (FPTP, some RCV).
- Verifiability and openness: ❌ — mostly closed systems; independent audits hindered by vendor trade secrecy. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- One ballot per voter + revocation: ✅ — integrated with state voter registries.
- Ballot secrecy: ✅ — machines offline at the polling place, paper ballots/VVPAT in most states.
- Identity verification and delegation: ✅ — traditional verification at the polling place per state law.
- Digital independence: ❌ — strong dependence on a handful of vendors, no interoperability between different manufacturers' systems.
- Sources: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 41. South Korea — K-Voting

**Country:** South Korea (National Election Commission) · **Implementation language:** not publicly known

An internet-voting system run by the Korean election commission for organizational/university/cooperative votes — not for binding national elections, which remain paper-based.

- Low-cost administration: ✅ — relatively cheap for smaller organizations using the government service.
- Universality: ⚠️ — supports various types of organizational votes, but not used for full national elections. ([idea.int — database](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349))
- Any electoral formula: ⚠️ — limited to methods typical of organizational votes.
- Verifiability and openness: ❌ — proprietary/government system, non-public code.
- One ballot per voter + revocation: ✅ — integrated with the membership lists of organizations using the service.
- Ballot secrecy: ❓ — no verified source this session.
- Identity verification and delegation: ✅ — integration with Korean digital identity infrastructure.
- Digital independence: ❌ — a centralized service run exclusively by a single government institution.
- Sources: [idea.int — Online Voting: Current and Future Practices (South Korea — "under discussion")](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 42. Canada — commercial SaaS in local elections (Ontario)

**Country:** Canada · **Implementation language:** not publicly known

Local elections in the province of Ontario and other Canadian regions use commercial SaaS platforms (including Simply Voting, Dominion Internet Voting) for internet voting; there is no unified national system.

- Low-cost administration: ⚠️ — relatively cheap for small municipalities compared to building an in-house system, but generates recurring vendor fees.
- Universality: ⚠️ — supports typical Canadian municipal ballots (multiple candidates for multiple seats), configurations vary by vendor.
- Any electoral formula: ⚠️ — limited to methods supported by the given SaaS vendor.
- Verifiability and openness: ❌ — proprietary systems from different vendors, no unified, public, cryptographic verifiability.
- One ballot per voter + revocation: ✅ — integrated with local municipal voter registries.
- Ballot secrecy: ❓ — claimed by vendors, no independently verified source this session.
- Identity verification and delegation: ⚠️ — usually a PIN mailed by post, without strong integration with a federal e-ID.
- Digital independence: ❌ — dependence on external commercial vendors, different for different municipalities.
- Sources: [rappler.com — Which countries have conducted online elections (Ontario, Canada)](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/)

### 43. Russia — Moscow blockchain e-voting system

**Country:** Russia (Moscow City Department of Information Technology) · **Implementation language:** JavaScript (client), PHP (server), Solidity (Ethereum smart contracts)

An i-voting system partially used for the Moscow City Duma elections in September 2019 (3 of 45 districts), based on the Ethereum blockchain; code partially published for public testing before the election.

- Low-cost administration: ❓ — no verified source this session regarding costs.
- Universality: ❌ — tested only in selected districts of a local city election.
- Any electoral formula: ❌ — adapted exclusively to specific municipal elections.
- Verifiability and openness: ⚠️ — some code published on GitHub before the election for testing, but without a formal protocol specification and without publication in any language other than Russian. ([members.loria.fr — Gaudry, Breaking the encryption scheme](https://members.loria.fr/PGaudry/moscow/), [arxiv.org/pdf/1908.05127](https://arxiv.org/pdf/1908.05127))
- One ballot per voter + revocation: ❓ — no verified source this session.
- Ballot secrecy: ❌ — French researchers (Pierrick Gaudry, Alexander Golovnev) showed that the first version of the encryption scheme could be broken in about 20 minutes on an ordinary computer, allowing every voter's ballot to be revealed immediately after casting; after the fix, a second flaw was demonstrated allowing partial recovery of the tally. ([members.loria.fr](https://members.loria.fr/PGaudry/moscow/), [arxiv.org/pdf/1908.05127](https://arxiv.org/pdf/1908.05127), [zdnet.com](https://www.zdnet.com/article/moscows-blockchain-voting-system-cracked-a-month-before-election/), [coindesk.com](https://www.coindesk.com/markets/2019/08/16/moscow-blockchain-voting-system-completely-insecure-says-researcher))
- Identity verification and delegation: ❓ — no verified source this session.
- Digital independence: ❌ — a closed system run centrally by the city IT department, with protocol changes introduced shortly before the election without full documentation. ([members.loria.fr](https://members.loria.fr/PGaudry/moscow/))
- Sources: [members.loria.fr — Breaking the encryption scheme of the Moscow internet voting system (Pierrick Gaudry)](https://members.loria.fr/PGaudry/moscow/), [arxiv.org/pdf/1908.05127](https://arxiv.org/pdf/1908.05127), [fc20.ifca.ai — full peer-reviewed publication](https://fc20.ifca.ai/preproceedings/178.pdf), [zdnet.com](https://www.zdnet.com/article/moscows-blockchain-voting-system-cracked-a-month-before-election/), [coindesk.com](https://www.coindesk.com/markets/2019/08/16/moscow-blockchain-voting-system-completely-insecure-says-researcher)

### 44. Voatz

**Country:** USA (Boston, Massachusetts) · **Implementation language:** mobile app (obfuscated code, not publicly disclosed)

A closed, commercial mobile voting app using a permissioned blockchain, biometrics, and hardware modules for key storage; used in pilots including West Virginia (overseas soldiers, 2018 midterms), Denver County, and Utah County.

- Low-cost administration: ❌ — pilot and technical-support costs borne by jurisdictions, while there is no public documentation to facilitate independent deployment. ([usenix.org — The Ballot is Busted Before the Blockchain](https://www.usenix.org/system/files/sec20-specter.pdf))
- Universality: ❌ — limited to narrow pilots, not tested at full scale or ballot diversity.
- Any electoral formula: ❓ — no verified source this session.
- Verifiability and openness: ❌ — the app's code was deliberately obfuscated, the server was never made available for analysis; MIT researchers had to reverse-engineer the system's behavior. ([internetpolicy.mit.edu — FAQ](https://internetpolicy.mit.edu/faq-on-the-security-analysis-of-voatz/), [csail-dev-2025.csail.mit.edu](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app))
- One ballot per voter + revocation: ⚠️ — claimed by the company, but independently unverified due to lack of server access. ([usenix.org — The Ballot is Busted Before the Blockchain](https://www.usenix.org/system/files/sec20-specter.pdf))
- Ballot secrecy: ❌ — MIT researchers showed that a passive network observer (e.g. an internet provider) could, in some configurations, determine how a user voted via a side-channel attack. ([usenix.org — The Ballot is Busted Before the Blockchain](https://www.usenix.org/system/files/sec20-specter.pdf), [csail-dev-2025.csail.mit.edu](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app))
- Identity verification and delegation: ⚠️ — verification via a third-party identity provider, which itself created additional privacy risk (third-party access to ID photos/data). ([csail-dev-2025.csail.mit.edu](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app))
- Digital independence: ❌ — fully closed, dependent on a single company, which refused to publish source code despite appeals from the security community. ([internetpolicy.mit.edu — FAQ](https://internetpolicy.mit.edu/faq-on-the-security-analysis-of-voatz/))
- Sources: [usenix.org — The Ballot is Busted Before the Blockchain: A Security Analysis of Voatz](https://www.usenix.org/system/files/sec20-specter.pdf), [internetpolicy.mit.edu — FAQ on the Security Analysis of Voatz](https://internetpolicy.mit.edu/faq-on-the-security-analysis-of-voatz/), [csail-dev-2025.csail.mit.edu — MIT researchers identify security vulnerabilities](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app)

### 45. Kaspersky Polys

**Country:** Russia · **Implementation language:** not publicly known

A commercial blockchain-voting SaaS offered by Kaspersky, used partly in pilot consultations in Moscow (2019); the main 2019/2020 Moscow city vote, however, used the city's own separate system (see item 43), not Polys itself.

- Low-cost administration: ❓ — no verified source this session.
- Universality: ❓ — no verified source this session.
- Any electoral formula: ❓ — no verified source this session.
- Verifiability and openness: ❌ — proprietary system, non-public code.
- One ballot per voter + revocation: ❓ — no verified source this session.
- Ballot secrecy: ❓ — claimed by the vendor (homomorphic encryption on the blockchain), but not independently verified this session.
- Identity verification and delegation: ❓ — no verified source this session.
- Digital independence: ❌ — a closed, single-vendor commercial product.
- Sources: no verified link found this session — assessment based on general public knowledge of the Kaspersky Polys product; see also the Moscow pilot context under item 43.
