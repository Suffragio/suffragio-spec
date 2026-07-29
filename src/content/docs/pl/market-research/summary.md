---
title: Podsumowanie
description: Skrócone podsumowanie badania rynku systemów e-votingowych w kontekście wymagań Suffragio.
---

Ta sekcja zestawia wymagania Suffragio (patrz [Motywacja i wymagania](/suffragio-spec/pl/motivation/)) z istniejącymi na rynku systemami do głosowania elektronicznego/internetowego — zarówno projektami open-source/akademickimi, jak i platformami rządowymi i komercyjnymi używanymi w wyborach na świecie.

Pełne omówienie z ocenami wymagań i źródłami znajduje się w dokumencie [Wyniki](/suffragio-spec/pl/market-research/findings/), a zestawienie tabelaryczne w [Macierzy porównawczej](/suffragio-spec/pl/market-research/matrix/).

## Skrócona lista wymagań

1. **Niski koszt administrowania** — organizator może przeprowadzić wybory bez znaczących kosztów.
2. **Uniwersalność** — dowolny typ pytania/karty (referenda, listy partyjne, skale, różne okręgi).
3. **Dowolna ordynacja wyborcza** — wynik liczony wg dowolnej formuły, bez budowania osobnego systemu.
4. **Weryfikowalność i otwartość** — pełny dostęp do oddanych głosów i kodu źródłowego każdego komponentu.
5. **Jeden głos na uprawnionego + odwoływanie uprawnień** — jednorazowe pobranie karty, możliwość odebrania prawa głosu.
6. **Tajność głosu** — nikt, łącznie z rządem, nie ustali jak kto głosował.
7. **Weryfikacja tożsamości i delegacja uprawnień** — elektroniczna i osobista weryfikacja, drobnoziarnisty model uprawnień.
8. **Niezależność cyfrowa** — otwarte protokoły, wolna/copyleftowa licencja, brak przywiązania do jednego dostawcy/infrastruktury.

(Pełna, nieskrócona lista wymagań — łącznie z pełną audytowalnością, integralnością oprogramowania i jawnością rejestrów wyborców — znajduje się w [Motywacji i wymaganiach](/suffragio-spec/pl/motivation/).)

## Zbadane systemy (45)

Poniżej lista wszystkich zbadanych systemów z jednozdaniowym podsumowaniem każdego. Szczegółowa ocena wymagań i źródła — patrz [Wyniki](/suffragio-spec/pl/market-research/findings/).

### Projekty open-source / akademickie / obywatelskie

1. **Helios Voting** — webowy system E2E-weryfikowalny na bazie szyfrowania homomorficznego, używany do wyborów akademickich i w stowarzyszeniach.
2. **Belenios** — francuski, akademicki następca Heliosa z formalną weryfikacją bezpieczeństwa protokołu.
3. **Zeus** — grecki fork Heliosa używany przez uczelnie i związki zawodowe w Grecji.
4. **Microsoft ElectionGuard** — open-source'owe SDK kryptograficzne (homomorficzne szyfrowanie) do wbudowania w istniejące maszyny do głosowania.
5. **STAR-Vote** — zarzucony projekt hrabstwa Travis (Teksas), miał łączyć DRE z E2E-weryfikowalnością.
6. **Scantegrity II** — akademicki system E2E-weryfikowalny na bazie papierowych kart z ukrytym kodem, użyty w Takoma Park (Maryland).
7. **Prêt à Voter** — brytyjska koncepcja kryptograficznej karty do głosowania z odrywaną częścią.
8. **vVote** — realne wdrożenie Prêt à Voter w wyborach stanowych Wiktorii (Australia) w 2014 r.
9. **Civitas** — akademicki system Cornell projektowany pod odporność na przymus wyborczy.
10. **Selene** — akademicki protokół E2E z pseudonimami ułatwiającymi weryfikację przez zwykłych wyborców.
11. **DEMOS / DEMOS-2** — greckie akademickie protokoły E2E-weryfikowalne.
12. **CIVS** (Cornell) — prosty, open-source'owy silnik ankiet metodą Condorceta.
13. **Agora Voting** — hiszpański open-source, blockchain, użyty w prawyborach Podemos.
14. **Decidim** — barcelońska platforma partycypacji obywatelskiej (budżety obywatelskie), open-source.
15. **Sovereign (Democracy Earth)** — eksperymentalna, open-source'owa platforma "płynnej demokracji" na blockchainie.
16. **Follow My Vote** — niedokończony projekt blockchain voting.
17. **VotingWorks (VxSuite)** — non-profit, open-source sprzęt i oprogramowanie do skanowania papierowych kart w USA.
18. **TrustTheVote / OSET Institute** — open-source'owy pakiet do administracji wyborami (ElectOS), częściowo użyty w Los Angeles County.
19. **Free & Fair / ColoradoRLA** — open-source'owe narzędzia do statystycznych audytów wyborczych (risk-limiting audits).
20. **OpaVote** — komercyjny SaaS do głosowań preferencyjnych dla organizacji.
21. **ElectionBuddy** — komercyjny SaaS podobny do OpaVote.
22. **Simply Voting** — kanadyjski komercyjny SaaS używany m.in. przez związki zawodowe.
23. **BigPulse** — komercyjny SaaS do głosowań organizacyjnych.
24. **Loomio** — open-source'owe narzędzie do podejmowania decyzji grupowych (nie ściśle wyborcze).

### Platformy rządowe / komercyjne używane w wyborach na świecie

25. **Estonia — IVXV** — jedyny na świecie system i-votingu używany bez ograniczeń we wszystkich wyborach krajowych, kod publikowany na GitHub.
26. **Szwajcaria — Swiss Post e-voting** — system kantonalny z publicznie dostępnym kodem i regularnymi testami penetracyjnymi.
27. **Genewa — CHVote** — open-source'owy system kantonalny, wycofany w 2020 r.
28. **Norwegia** — pilotaż i-votingu 2011–2013, zaniechany.
29. **Nowa Południowa Walia — iVote** — australijski system stanowy, wycofany w 2021 r. po awarii.
30. **Francja** — i-voting dla obywateli za granicą (dostawca Voxaly/Docaposte).
31. **Panama** — i-voting dla obywateli za granicą.
32. **Meksyk — INE** — system głosowania internetowego dla Meksykanów za granicą.
33. **Armenia** — pilotaże i-votingu dla diaspory.
34. **Zjednoczone Emiraty Arabskie** — aplikacja mobilna z rozpoznawaniem twarzy do wyborów Federalnej Rady Narodowej.
35. **Indie — EVM (ECIL/BEL)** — offline'owe maszyny do głosowania z VVPAT, kod niepubliczny.
36. **Brazylia — Urna Eletrônica (TSE)** — offline'owe DRE bez VVPAT, kod dostępny tylko akredytowanym audytorom.
37. **Belgia** — system z drukowanym potwierdzeniem skanowanym kodem kreskowym.
38. **Wenezuela** — DRE dostarczane przez Smartmatic (do 2017) i lokalnych dostawców.
39. **Filipiny — Smartmatic** — skanery optyczne używane od 2010 r.
40. **USA — Dominion / ES&S / Hart InterCivic / Unisyn / MicroVote / Clear Ballot** — różne maszyny DRE/skanery optyczne używane w poszczególnych stanach.
41. **Korea Południowa — K-Voting** — system do głosowań organizacyjnych, nie do wyborów krajowych.
42. **Kanada** — komercyjne SaaS (Simply Voting i inne) używane w wyborach samorządowych Ontario.
43. **Rosja — moskiewski system blockchain e-voting** — system na Ethereum/Exonum, złamany przez badaczy w 2019 r.
44. **Voatz** — amerykańska, zamknięta aplikacja mobilna typu blockchain, użyta w pilotażach; wykazano poważne luki bezpieczeństwa.
45. **Kaspersky Polys** — rosyjski, komercyjny SaaS do głosowań na blockchainie.
