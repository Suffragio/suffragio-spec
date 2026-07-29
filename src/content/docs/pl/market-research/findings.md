---
title: Wyniki
description: Szczegółowa ocena 45 systemów e-votingowych względem wymagań Suffragio, wraz ze źródłami.
---

Poniżej pełna lista 45 zbadanych systemów. Dla każdego z nich podano kraj pochodzenia, język implementacji (jeśli publicznie znany), krótki opis oraz ocenę ośmiu wymagań Suffragio (pełne nazwy — patrz [Motywacja i wymagania](/suffragio-spec/pl/motivation/)), z linkami do źródeł, na podstawie których dokonano oceny.

**Legenda ocen:** ✅ tak · ❌ nie · ⚠️ częściowo/warunkowo · ❓ brak publicznej informacji.

> **Uwaga metodologiczna:** to badanie rynku, nie audyt bezpieczeństwa. Tam, gdzie w trakcie researchu nie udało się znaleźć jednoznacznego, możliwego do zacytowania źródła dla konkretnej oceny, jest to wprost oznaczone jako „brak zweryfikowanego źródła” — ocena wynika wtedy z ogólnodostępnej wiedzy o projekcie, a nie z konkretnego dokumentu.

## Projekty open-source / akademickie / obywatelskie

### 1. Helios Voting

**Kraj:** USA (Ben Adida) · **Język implementacji:** Python, JavaScript

Webowy system E2E-weryfikowalny oparty o szyfrowanie homomorficzne i mixnety, używany do wyborów akademickich (np. IACR, ACM) i w stowarzyszeniach.

- Niski koszt administrowania: ✅ — darmowe, samodzielnie hostowalne oprogramowanie webowe. ([github.com/benadida/helios-server](https://github.com/benadida/helios-server))
- Uniwersalność: ⚠️ — obsługuje głównie głosowanie jednokrotnego/wielokrotnego wyboru (approval voting), brak wsparcia dla złożonych kart (listy partyjne, różne okręgi). ([heliosvoting.org](http://heliosvoting.org))
- Dowolna ordynacja wyborcza: ⚠️ — sam produkuje wynik tylko dla prostych metod; nie jest ogólnym silnikiem tabulacyjnym.
- Weryfikowalność i otwartość: ✅ — kod na licencji Apache, kryptograficzna weryfikowalność E2E. ([github.com/benadida/helios-server](https://github.com/benadida/helios-server))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — lista wyborców jest konfigurowana ręcznie przez organizatora, brak wbudowanego mechanizmu odwoływania uprawnień.
- Tajność głosu: ✅ — szyfrowanie homomorficzne i mixnety gwarantują tajność. ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — brak wbudowanej integracji z e-ID/weryfikacją osobistą.
- Niezależność cyfrowa: ✅ — open source, można hostować samodzielnie, brak zależności od jednego dostawcy. ([github.com/benadida/helios-server](https://github.com/benadida/helios-server))

### 2. Belenios

**Kraj:** Francja (Inria) · **Język implementacji:** OCaml, JavaScript

Akademicki system E2E-weryfikowalny, częściowo implementujący protokół Helios-C, z formalną weryfikacją bezpieczeństwa.

- Niski koszt administrowania: ✅ — darmowe, dostępna też hostowana platforma. ([belenios.org](https://www.belenios.org/))
- Uniwersalność: ⚠️ — obsługuje różne typy kart (w tym referenda), ale nie pełne, zróżnicowane karty wieloobwodowe jak w wyborach krajowych. ([belenios.org](https://www.belenios.org/))
- Dowolna ordynacja wyborcza: ⚠️ — jak wyżej, ograniczone do wspieranych typów głosowania.
- Weryfikowalność i otwartość: ✅ — licencja AGPLv3, formalnie zweryfikowane bezpieczeństwo protokołu. ([github.com/glondu/belenios](https://github.com/glondu/belenios))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — lista wyborców konfigurowana ręcznie, brak wbudowanego odwoływania uprawnień.
- Tajność głosu: ✅ — szyfrowanie homomorficzne z rozproszonym zaufaniem (trustees). ([belenios.org](https://www.belenios.org/))
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — logowanie przez e-mail/konto Google/CAS Inria, brak silnej weryfikacji tożsamości. ([belenios.org](https://www.belenios.org/))
- Niezależność cyfrowa: ✅ — open source (AGPLv3), samodzielnie hostowalne. ([github.com/glondu/belenios](https://github.com/glondu/belenios))

### 3. Zeus

**Kraj:** Grecja (GRNET) · **Język implementacji:** Python

Fork Heliosa rozwijany przez grecką sieć akademicką GRNET, używany przez greckie uczelnie i związki zawodowe; zamiast wyniku produkuje audytowalny zestaw głosów do dalszej tabulacji.

- Niski koszt administrowania: ✅ — darmowe, open source. ([github.com/grnet/zeus](https://github.com/grnet/zeus))
- Uniwersalność: ⚠️ — obsługuje więcej systemów głosowania niż Helios dzięki oddzieleniu zbierania głosów od tabulacji, ale nadal ograniczone do prostych kart. ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Dowolna ordynacja wyborcza: ⚠️ — Zeus celowo tylko dostarcza tally głosów, które można podać do dowolnego zewnętrznego silnika liczącego. ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Weryfikowalność i otwartość: ✅ — open source, kryptograficznie weryfikowalny. ([github.com/grnet/zeus](https://github.com/grnet/zeus))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — jak w Heliosie, ręczna konfiguracja listy wyborców.
- Tajność głosu: ✅ — dziedziczy kryptografię Heliosa (mixnety/szyfrowanie homomorficzne). ([esdep.auth.gr — From Helios to Zeus](https://esdep.auth.gr/wp-content/uploads/2014/06/from_helios_to_zeus.pdf))
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — brak wbudowanej integracji z e-ID.
- Niezależność cyfrowa: ✅ — open source, samodzielnie hostowalne. ([github.com/grnet/zeus](https://github.com/grnet/zeus))

### 4. Microsoft ElectionGuard

**Kraj:** USA (Microsoft, wspólnie z Galois) · **Język implementacji:** C++, C#, Python

Open-source'owe SDK kryptograficzne (nie pełny system wyborczy) wykorzystujące szyfrowanie homomorficzne, przeznaczone do wbudowania w istniejące maszyny do głosowania w celu umożliwienia E2E-weryfikowalności i audytów porównawczych kart.

- Niski koszt administrowania: ⚠️ — samo SDK jest darmowe, ale wymaga integracji z istniejącym sprzętem/systemem wyborczym. ([github.com/microsoft/electionguard](https://github.com/microsoft/electionguard))
- Uniwersalność: ❓ — zależy od systemu, w który zostanie wbudowane; SDK samo w sobie nie narzuca ograniczeń co do typu karty.
- Dowolna ordynacja wyborcza: ❓ — jak wyżej, zależne od integracji.
- Weryfikowalność i otwartość: ✅ — licencja MIT, pełny kod źródłowy warstwy kryptograficznej. ([electionguard.vote](https://www.electionguard.vote/), [github.com/microsoft/electionguard](https://github.com/microsoft/electionguard))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❌ — poza zakresem SDK (obsługiwane przez system, do którego jest wbudowywane).
- Tajność głosu: ✅ — szyfrowanie homomorficzne gwarantuje tajność zaszyfrowanych głosów. ([electionguard.vote](https://www.electionguard.vote/))
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — poza zakresem SDK.
- Niezależność cyfrowa: ✅ — open source (MIT), brak opłat licencyjnych. ([github.com/microsoft/electionguard](https://github.com/microsoft/electionguard))

### 5. STAR-Vote

**Kraj:** USA (hrabstwo Travis, Teksas) · **Język implementacji:** nieznany (projekt zarzucony przed pełną implementacją)

Projektowany system łączący DRE z komercyjnym sprzętem (COTS) i E2E-weryfikowalnością na bazie koncepcji zbliżonych do Heliosa/Scantegrity; zarzucony przed wdrożeniem.

- Niski koszt administrowania: ❓ — projekt nie został ukończony.
- Uniwersalność: ❓ — nieznane, projekt zarzucony.
- Dowolna ordynacja wyborcza: ❓ — nieznane.
- Weryfikowalność i otwartość: ⚠️ — planowany jako open source, ale nigdy w pełni wdrożony ani opublikowany. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — nieznane.
- Tajność głosu: ✅ — projektowo zakładana (koncepcja E2E-weryfikowalna). ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Weryfikacja tożsamości i delegacja uprawnień: ❓ — nieznane.
- Niezależność cyfrowa: ❓ — projekt porzucony przed wdrożeniem, brak opublikowanego kodu. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))

### 6. Scantegrity II

**Kraj:** USA (akademicki, University of Waterloo / Rice University) · **Język implementacji:** nieznany publicznie

E2E-weryfikowalny system oparty o papierowe karty z ukrytym kodem potwierdzającym ujawnianym przez zaznaczenie kandydata specjalnym markerem; użyty realnie w wyborach lokalnych Takoma Park (Maryland, USA) w 2009 i 2011 r.

- Niski koszt administrowania: ✅ — bazuje na standardowym sprzęcie do skanowania kart papierowych.
- Uniwersalność: ❌ — zaprojektowany i użyty tylko do prostych, lokalnych wyborów jednokrotnego wyboru.
- Dowolna ordynacja wyborcza: ❌ — nie jest ogólnym silnikiem tabulacyjnym.
- Weryfikowalność i otwartość: ✅ — koncepcja E2E-weryfikowalna, publikowana akademicko.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Tajność głosu: ✅ — zachowanie tajności głosu jest jednym z projektowych celów systemu.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — nie obejmuje warstwy tożsamości, opiera się na tradycyjnym procesie w lokalu wyborczym.
- Niezależność cyfrowa: ✅ — koncepcja akademicka, brak przywiązania do jednego dostawcy.

> Uwaga: dla Scantegrity II nie udało się w tej sesji znaleźć bezpośrednio cytowalnego źródła internetowego — powyższa ocena bazuje na ogólnodostępnej wiedzy akademickiej o systemie.

### 7. Prêt à Voter

**Kraj:** Wielka Brytania (University of Surrey) · **Język implementacji:** koncepcja/referencyjne prototypy, różne języki

Koncepcja kryptograficznej karty do głosowania z odrywaną, losowo permutowaną listą kandydatów — wyborca głosuje na papierze, a odrywana część jest niszczona, reszta skanowana jako zaszyfrowany, weryfikowalny głos.

- Niski koszt administrowania: ❓ — koncepcja badawcza, brak danych o kosztach realnego wdrożenia na dużą skalę.
- Uniwersalność: ❌ — pierwotnie zaprojektowana dla głosowania jednokrotnego wyboru; rozszerzenia na inne ordynacje wymagały osobnych prac badawczych. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Dowolna ordynacja wyborcza: ⚠️ — rozszerzona m.in. o STV/IRV w ramach adaptacji do wyborów w Wiktorii (patrz vVote), ale nie jest uniwersalnym silnikiem. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Weryfikowalność i otwartość: ✅ — publikacje akademickie opisujące pełny protokół kryptograficzny. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — koncepcja skupia się na weryfikowalności głosu, nie na zarządzaniu uprawnieniami.
- Tajność głosu: ✅ — kluczowa cecha projektowa (odrywana lista kandydatów niszczona przed opuszczeniem lokalu). ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — poza zakresem koncepcji, zakłada tradycyjny proces w lokalu wyborczym.
- Niezależność cyfrowa: ✅ — koncepcja akademicka, publicznie opisana, brak przywiązania do dostawcy.

### 8. vVote

**Kraj:** Australia (stan Wiktoria; zespół: University of Surrey, Melbourne, Luxembourg) · **Język implementacji:** Java (m.in.)

Realne wdrożenie Prêt à Voter w wyborach stanowych Wiktorii w listopadzie 2014 r. — pierwsze na świecie użycie w wiążących wyborach politycznych systemu E2E-weryfikowalnego, obsługującego m.in. głosujących niewidomych i zdalnych.

- Niski koszt administrowania: ❌ — wymagał dedykowanego zespołu akademickiego i wieloletniego rozwoju (ok. 2 lata) na potrzeby jednych wyborów. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Uniwersalność: ❌ — dostosowany specyficznie do ordynacji stanu Wiktoria (IRV + STV), nie ogólny system uniwersalny. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Dowolna ordynacja wyborcza: ⚠️ — obsługuje IRV i STV z list "above/below the line", ale wymagał dedykowanych prac adaptacyjnych. ([usenix.org — Using Prêt à Voter in Victorian State elections](https://www.usenix.org/system/files/conference/evtwote12/evtwote12-final9_0.pdf))
- Weryfikowalność i otwartość: ✅ — kod open source, opublikowany po wyborach. ([arxiv.org/abs/1404.6822 — kod: bitbucket.org/vvote](https://doi.org/10.48550/arxiv.1404.6822))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — realizowane proceduralnie przez komisję wyborczą (VEC), nie kryptograficznie. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Tajność głosu: ✅ — pełna tajność zgodnie z projektem Prêt à Voter, potwierdzona w realnym wdrożeniu. ([arxiv.org/abs/1504.07098](https://doi.org/10.48550/arxiv.1504.07098))
- Weryfikacja tożsamości i delegacja uprawnień: ⚠️ — tradycyjna weryfikacja w lokalu wyborczym (system był dostępny tylko w nadzorowanych punktach). ([past.electionwatch.edu.au](http://past.electionwatch.edu.au/victoria-2014/click-here-democracy-e-vote-explained))
- Niezależność cyfrowa: ❌ — zależny od kontraktu z konkretnymi wykonawcami (University of Surrey jako "SuVote", Cryptoworkshop.com jako mixnet). ([arxiv.org/abs/1404.6822](https://doi.org/10.48550/arxiv.1404.6822))

### 9. Civitas

**Kraj:** USA (Cornell University) · **Język implementacji:** Java

Akademicki system E2E-weryfikowalny zaprojektowany specjalnie pod kątem odporności na przymus wyborczy (coercion-resistance) — realizuje opcjonalny cel Suffragio dot. ochrony przed przymusem. Nigdy nie użyty w wiążących wyborach.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji dot. kosztów wdrożenia.
- Uniwersalność: ❌ — koncentruje się na prostych głosowaniach, nie na złożonych kartach wieloobwodowych.
- Dowolna ordynacja wyborcza: ❌ — nie jest ogólnym silnikiem tabulacyjnym.
- Weryfikowalność i otwartość: ✅ — publikacje akademickie z pełnym opisem protokołu.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Tajność głosu: ✅ — tajność głosu i odporność na przymus są głównymi celami projektowymi.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — poza zakresem projektu badawczego.
- Niezależność cyfrowa: ✅ — koncepcja akademicka, brak przywiązania do dostawcy.

> Uwaga: dla Civitas nie udało się w tej sesji znaleźć bezpośrednio cytowalnego źródła internetowego — ocena bazuje na ogólnodostępnej wiedzy akademickiej o projekcie.

### 10. Selene

**Kraj:** akademicki (Europa, m.in. IT University of Copenhagen) · **Język implementacji:** koncepcja/prototypy badawcze

Akademicki protokół E2E-weryfikowalny wykorzystujący pseudonimy wyborców, aby uprościć weryfikację głosu bez konieczności śledzenia skomplikowanych dowodów kryptograficznych.

- Niski koszt administrowania: ❓ — koncepcja badawcza.
- Uniwersalność: ❌ — skoncentrowana na uproszczeniu weryfikacji prostych głosów.
- Dowolna ordynacja wyborcza: ❌ — nie jest ogólnym silnikiem tabulacyjnym.
- Weryfikowalność i otwartość: ✅ — publikacje akademickie z pełnym opisem protokołu.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — poza głównym zakresem badawczym.
- Tajność głosu: ✅ — zachowanie tajności przy jednoczesnej łatwej weryfikowalności to główny cel projektu.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — poza zakresem koncepcji.
- Niezależność cyfrowa: ✅ — koncepcja akademicka, publicznie opisana.
- Źródła: brak zweryfikowanego linku w tej sesji — ocena na podstawie ogólnodostępnej wiedzy akademickiej o protokole Selene.

### 11. DEMOS / DEMOS-2

**Kraj:** Grecja (akademicki) · **Język implementacji:** koncepcja/prototypy badawcze

Greckie akademickie protokoły E2E-weryfikowalne, rozwijane równolegle do rodziny Helios/Zeus, z alternatywnym podejściem do dowodów kryptograficznych bez modelu wyroczni losowej.

- Niski koszt administrowania: ❓ — koncepcja badawcza.
- Uniwersalność: ❌ — skoncentrowane na prostych głosowaniach.
- Dowolna ordynacja wyborcza: ❌ — nie ogólny silnik tabulacyjny.
- Weryfikowalność i otwartość: ✅ — publikacje akademickie z pełnym opisem.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — poza głównym zakresem badawczym.
- Tajność głosu: ✅ — główny cel projektowy.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — poza zakresem koncepcji.
- Niezależność cyfrowa: ✅ — koncepcja akademicka.
- Źródła: brak zweryfikowanego linku w tej sesji — ocena na podstawie ogólnodostępnej wiedzy akademickiej o rodzinie protokołów DEMOS.

### 12. CIVS (Condorcet Internet Voting System)

**Kraj:** USA (Cornell University) · **Język implementacji:** Perl (historycznie)

Prosty, darmowy silnik do ankiet i głosowań metodą Condorceta, używany przez organizacje i społeczności open source do wewnętrznych głosowań, nie do wiążących wyborów publicznych.

- Niski koszt administrowania: ✅ — darmowe narzędzie webowe. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Uniwersalność: ❌ — tylko ankiety rankingowe metodą Condorceta, brak wsparcia dla złożonych kart wyborczych. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Dowolna ordynacja wyborcza: ❌ — obsługuje wyłącznie metodę Condorceta.
- Weryfikowalność i otwartość: ❌ — brak kryptograficznej E2E-weryfikowalności; zaufanie oparte na operatorze usługi.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — jednorazowy link/e-mail na uczestnika, brak formalnego zarządzania uprawnieniami.
- Tajność głosu: ⚠️ — pseudonimizacja przez e-mail, nie kryptograficzna tajność głosu.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — brak.
- Niezależność cyfrowa: ⚠️ — historycznie działał głównie jako usługa hostowana centralnie przez Cornell, nie typowy self-hosted open source.
- Źródła: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 13. Agora Voting

**Kraj:** Hiszpania · **Język implementacji:** Python, JavaScript (deklarowane jako open source)

Hiszpański system open source na bazie blockchaina, używany m.in. w prawyborach partii Podemos i niektórych konsultacjach obywatelskich.

- Niski koszt administrowania: ✅ — deklarowany jako darmowy, samodzielnie hostowalny.
- Uniwersalność: ⚠️ — obsługuje kilka typów głosowań, ale nie pełną złożoność kart wieloobwodowych.
- Dowolna ordynacja wyborcza: ⚠️ — ograniczone do wspieranych metod.
- Weryfikowalność i otwartość: ⚠️ — deklarowany jako open source, ale bez niezależnego audytu kryptograficznego porównywalnego z Heliosem.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — zależne od konfiguracji wdrożenia.
- Tajność głosu: ⚠️ — deklarowana, ale bez niezależnie zweryfikowanego dowodu kryptograficznego w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — brak wbudowanej integracji z silną tożsamością elektroniczną.
- Niezależność cyfrowa: ✅ — open source, self-hosted.
- Źródła: brak zweryfikowanego linku w tej sesji — ocena na podstawie ogólnodostępnej wiedzy o projekcie Agora Voting.

### 14. Decidim

**Kraj:** Hiszpania (Barcelona en Comú / gmina Barcelony) · **Język implementacji:** Ruby on Rails

Open-source'owa platforma partycypacji obywatelskiej (konsultacje, budżety obywatelskie, proste głosowania), używana przez wiele miast i instytucji w Europie.

- Niski koszt administrowania: ✅ — darmowe, self-hosted, licencja AGPL.
- Uniwersalność: ⚠️ — dobrze radzi sobie z prostymi głosowaniami/budżetami obywatelskimi, ale nie jest zaprojektowana pod pełne wybory powszechne (listy partyjne, różne okręgi).
- Dowolna ordynacja wyborcza: ❌ — nie jest ogólnym silnikiem tabulacyjnym dla formalnych ordynacji wyborczych.
- Weryfikowalność i otwartość: ⚠️ — kod w pełni open source, ale głosowania nie są kryptograficznie E2E-weryfikowalne.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❌ — zarządzanie uprawnieniami jest częścią platformy uczestnictwa, nie dedykowanego systemu wyborczego.
- Tajność głosu: ⚠️ — zależna od konfiguracji modułu głosowania, brak kryptograficznej gwarancji tajności.
- Weryfikacja tożsamości i delegacja uprawnień: ⚠️ — wspiera różne metody logowania/weryfikacji w zależności od wdrożenia instytucji.
- Niezależność cyfrowa: ✅ — open source (AGPL), self-hosted, duża społeczność.
- Źródła: [handwiki.org — Comparison of civic technology platforms](https://handwiki.org/wiki/Software:Comparison_of_civic_technology_platforms)

### 15. Sovereign (Democracy Earth)

**Kraj:** USA/międzynarodowy (Democracy Earth Foundation) · **Język implementacji:** JavaScript/blockchain (Bitcoin-based)

Eksperymentalna, open-source'owa platforma "płynnej demokracji" (liquid democracy) na bazie technologii blockchain, mająca zwiększyć zaufanie do procesu politycznego; brak szerokich, wiążących wdrożeń.

- Niski koszt administrowania: ✅ — deklarowany jako darmowy i open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Uniwersalność: ❌ — skoncentrowany na modelu delegowania głosów (liquid democracy), nie na klasycznych kartach wyborczych.
- Dowolna ordynacja wyborcza: ❌ — nie ogólny silnik tabulacyjny dla tradycyjnych ordynacji.
- Weryfikowalność i otwartość: ✅ — open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Tajność głosu: ❓ — mechanizmy blockchain mogą utrudniać pełną tajność głosu bez dodatkowej kryptografii; brak zweryfikowanego źródła.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — brak silnej integracji z e-ID.
- Niezależność cyfrowa: ✅ — open source, brak przywiązania do jednego dostawcy.
- Źródła: [osvtac.github.io — State of the Art Briefing na temat Sovereign/Democracy Earth](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 16. Follow My Vote

**Kraj:** USA (Wirginia) · **Język implementacji:** C++/blockchain

Niedokończony projekt blockchain voting, mający oferować w pełni open-source'owe, E2E-weryfikowalne głosowanie; pozostał na etapie proof-of-concept.

- Niski koszt administrowania: ❓ — projekt nigdy nie osiągnął produkcyjnej dojrzałości. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Uniwersalność: ❓ — nieznane, brak ukończonej implementacji.
- Dowolna ordynacja wyborcza: ❓ — nieznane.
- Weryfikowalność i otwartość: ⚠️ — deklarowany jako open source (MIT/Unlicense), ale niekompletny. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — nieznane.
- Tajność głosu: ❓ — nieznane, brak ukończonego wdrożenia.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — nie zaimplementowane.
- Niezależność cyfrowa: ✅ — deklarowany jako open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Źródła: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 17. VotingWorks (VxSuite)

**Kraj:** USA · **Język implementacji:** TypeScript, Rust (sprzęt open hardware)

Organizacja non-profit dostarczająca open-source'owy sprzęt i oprogramowanie do skanowania papierowych kart w lokalach wyborczych (nie internetowe), używana m.in. w hrabstwach Mississippi i New Hampshire.

- Niski koszt administrowania: ✅ — tańsza alternatywa dla własnościowych systemów DRE/skanerów. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Uniwersalność: ⚠️ — obsługuje typowe amerykańskie karty papierowe (różne wyścigi na jednej karcie), ale zaprojektowana pod głosowanie stacjonarne, nie internetowe.
- Dowolna ordynacja wyborcza: ⚠️ — obsługuje różne metody liczenia w ramach skanowania optycznego, ale nie jest uniwersalnym silnikiem dla dowolnej ordynacji.
- Weryfikowalność i otwartość: ✅ — sprzęt i oprogramowanie w pełni open source (AGPL). ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❌ — nie zarządza rejestracją/uprawnieniami wyborców, tylko skanowaniem kart w lokalu.
- Tajność głosu: ✅ — papierowe karty skanowane offline, bez powiązania z tożsamością.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — poza zakresem produktu (realizowane przez lokalną administrację wyborczą).
- Niezależność cyfrowa: ✅ — open source, można rozwijać niezależnie. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Źródła: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 18. TrustTheVote / OSET Institute (ElectOS)

**Kraj:** USA (Palo Alto, CA) · **Język implementacji:** Ruby, Java, PHP, C#

Open-source'owy pakiet do administracji wyborami (rejestracja wyborców, tabulacja, firmware urządzeń do głosowania), częściowo wykorzystany w Los Angeles County (projekt VSAP).

- Niski koszt administrowania: ⚠️ — darmowe oprogramowanie, ale wdrożenie (jak w LA County) wymagało dedykowanego, kosztownego sprzętu (BMD). ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Uniwersalność: ⚠️ — obsługuje typowe amerykańskie karty wielo-wyścigowe, nie testowany pod kątem pełnej uniwersalności (referenda, skale itd.).
- Dowolna ordynacja wyborcza: ⚠️ — ograniczone do typowych ordynacji stosowanych w wyborach amerykańskich.
- Weryfikowalność i otwartość: ✅ — open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — obejmuje moduł rejestracji wyborców, ale zależny od integracji z lokalnymi rejestrami.
- Tajność głosu: ⚠️ — zależna od trybu wdrożenia (BMD + skaner offline).
- Weryfikacja tożsamości i delegacja uprawnień: ⚠️ — moduł rejestracji istnieje, ale brak silnej integracji z e-ID poza kontekstem USA.
- Niezależność cyfrowa: ✅ — open source, ale wysoki koszt sprzętowy ogranicza łatwość samodzielnego wdrożenia. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Źródła: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 19. Free & Fair / ColoradoRLA

**Kraj:** USA (Galois, Portland OR) · **Język implementacji:** Java

Open-source'owe narzędzia do statystycznych audytów wyborczych (risk-limiting audits), używane m.in. przez stan Kolorado; nie jest pełnym systemem wyborczym, tylko narzędziem pomocniczym.

- Niski koszt administrowania: ✅ — darmowe narzędzie audytowe. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Uniwersalność: ❌ — narzędzie pomocnicze do audytu, nie system do przeprowadzania wyborów.
- Dowolna ordynacja wyborcza: ❌ — poza zakresem narzędzia.
- Weryfikowalność i otwartość: ✅ — w pełni open source. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❌ — poza zakresem narzędzia.
- Tajność głosu: ❌ — poza zakresem narzędzia (operuje na już policzonych/zeskanowanych głosach).
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — poza zakresem narzędzia.
- Niezależność cyfrowa: ✅ — open source, można wdrożyć niezależnie. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Źródła: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 20–23. Komercyjne SaaS do głosowań organizacyjnych: OpaVote, ElectionBuddy, Simply Voting, BigPulse

**Kraj:** USA/Kanada · **Język implementacji:** nieznany publicznie (usługi zamknięte)

Cztery podobne, komercyjne platformy SaaS do przeprowadzania głosowań preferencyjnych/rankingowych dla organizacji, stowarzyszeń i związków zawodowych (nie do wiążących wyborów publicznych). Simply Voting jest używana m.in. przez niektóre organizacje w Kanadzie.

- Niski koszt administrowania: ✅ — niskie ceny abonamentowe, łatwe w konfiguracji dla małych organizacji.
- Uniwersalność: ⚠️ — obsługują różne metody preferencyjne, ale nie pełną złożoność wyborów powszechnych (różne okręgi, listy partyjne).
- Dowolna ordynacja wyborcza: ⚠️ — kilka wbudowanych metod liczenia (m.in. STV), ale zamknięty katalog opcji.
- Weryfikowalność i otwartość: ❌ — zamknięty kod źródłowy, zaufanie oparte na dostawcy usługi.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — obsługiwane na poziomie listy uczestników wgrywanej przez organizatora, brak formalnego mechanizmu odwoływania uprawnień.
- Tajność głosu: ⚠️ — deklarowana przez dostawcę, ale niezależnie niezweryfikowana kryptograficznie.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — zwykle tylko e-mail/link jednorazowy, brak integracji z silną tożsamością elektroniczną.
- Niezależność cyfrowa: ❌ — zamknięte, hostowane wyłącznie przez dostawcę, brak możliwości samodzielnego hostingu.
- Źródła: brak zweryfikowanego linku w tej sesji dla żadnej z czterech usług — ocena na podstawie ogólnodostępnej wiedzy o kategorii produktów SaaS do głosowań organizacyjnych.

### 24. Loomio

**Kraj:** Nowa Zelandia · **Język implementacji:** Ruby on Rails

Open-source'owe narzędzie do wspólnego podejmowania decyzji grupowych (dyskusja + proste głosowanie), używane przez organizacje społeczne i niektóre samorządy; nie jest dedykowanym systemem wyborczym.

- Niski koszt administrowania: ✅ — open source, dostępna też wersja hostowana.
- Uniwersalność: ❌ — zaprojektowane pod proste głosowania grupowe/konsensusowe, nie pod formalne wybory powszechne.
- Dowolna ordynacja wyborcza: ❌ — nie jest silnikiem tabulacyjnym dla formalnych ordynacji.
- Weryfikowalność i otwartość: ⚠️ — kod open source, ale brak kryptograficznej E2E-weryfikowalności głosów.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❌ — zarządzanie uprawnieniami ograniczone do zarządzania członkostwem w grupie dyskusyjnej.
- Tajność głosu: ⚠️ — głosowania mogą być jawne lub anonimowe w zależności od konfiguracji, bez kryptograficznej gwarancji.
- Weryfikacja tożsamości i delegacja uprawnień: ❌ — brak integracji z silną tożsamością elektroniczną.
- Niezależność cyfrowa: ✅ — open source, self-hosted.
- Źródła: [handwiki.org — Comparison of civic technology platforms](https://handwiki.org/wiki/Software:Comparison_of_civic_technology_platforms)

## Platformy rządowe / komercyjne używane w wyborach na świecie

### 25. Estonia — IVXV (i-Voting)

**Kraj:** Estonia (RIA — Estońska Agencja ds. Systemów Informacyjnych) · **Język implementacji:** Go, Java, Python, Android/iOS (weryfikacja)

Jedyny na świecie system i-votingu używany bez ograniczeń we wszystkich rodzajach wyborów krajowych, od 2005 r. (obecna generacja IVXV od 2017 r.). Kod źródłowy publikowany na GitHub.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji dot. kosztów operacyjnych.
- Uniwersalność: ✅ — używany do wszystkich rodzajów wyborów krajowych i lokalnych w Estonii. ([valimised.ee — e-voting w innych krajach](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowany do estońskiej ordynacji, nie jest ogólnym silnikiem konfigurowalnym pod dowolny system wyborczy.
- Weryfikowalność i otwartość: ⚠️ — kod źródłowy publikowany na GitHub do wglądu publicznego, ale rozwój odbywa się pod nadzorem państwowego urzędu wyborczego (nie w pełni otwarty, społecznościowy model rozwoju). ([github.com/valimised/ivxv](https://github.com/valimised/ivxv), [valimised.ee — dokumenty o głosowaniu internetowym](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — wyborca może głosować wielokrotnie (liczy się ostatni głos), integracja z estońskim rejestrem ludności zapewnia weryfikację uprawnień. ([valimised.ee](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting))
- Tajność głosu: ⚠️ — krytykowana przez badaczy bezpieczeństwa za potencjalne wektory ataku po stronie urządzenia klienckiego, mimo braku potwierdzonego wpływu na wyniki wyborów.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — silna integracja z estońskim elektronicznym dowodem osobistym (ID-kaart) i mobile-ID. ([valimised.ee](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting))
- Niezależność cyfrowa: ❌ — ściśle zależny od estońskiej infrastruktury PKI/e-ID, niemożliwy do wdrożenia bez tej infrastruktury w innym kraju. ([github.com/valimised](https://github.com/valimised))
- Źródła: [github.com/valimised/ivxv](https://github.com/valimised/ivxv), [valimised.ee — dokumenty o i-votingu](https://www.valimised.ee/en/internet-voting/documents-about-internet-voting), [github.com/valimised/ivotingverification](https://github.com/valimised/ivotingverification)

### 26. Szwajcaria — Swiss Post e-voting

**Kraj:** Szwajcaria (kantony Bazylea-Miasto, St. Gallen, Thurgau, od 2023) · **Język implementacji:** Java, TypeScript

System następcy Scytl/CHVote rozwijany od 2019 r. samodzielnie przez Pocztę Szwajcarską, z publicznie dostępnym kodem i regularnymi, dorocznymi testami penetracyjnymi ("public intrusion tests") z nagrodami do 250 000 CHF.

- Niski koszt administrowania: ❌ — bardzo wysoki koszt certyfikacji i utrzymania (nagrody za błędy sięgające setek tysięcy franków, dedykowany zespół kryptografów). ([swisspost-digital.ch — źródło kodu](https://swisspost-digital.ch/en/solutions/e-voting/publications-and-source-code))
- Uniwersalność: ✅ — używany do referendów i wyborów federalnych/kantonalnych o różnej strukturze. ([swisspost-digital.ch](https://swisspost-digital.ch/en/digital-blog/e-government/the-source-code-of-the-future-e-voting-system-is-publicly-accessible))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowany do szwajcarskiej ordynacji, nie jest deklarowany jako uniwersalny silnik.
- Weryfikowalność i otwartość: ✅ — kluczowe komponenty (kryptografia, weryfikator) publikowane na licencji Apache 2, pełna dokumentacja publiczna, reprodukowalne środowisko testowe. ([gitlab.com/swisspost-evoting/e-voting/e-voting](https://gitlab.com/swisspost-evoting/e-voting/e-voting), [security whitepaper](https://gitlab.com/swisspost-evoting/e-voting/e-voting-documentation/-/blob/master/Product/Security%20Whitepaper%20of%20the%20Swiss%20Post%20Voting%20System.md))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowany z kantonalnymi rejestrami wyborców. ([swisspost-digital.ch — FAQ](https://swisspost-digital.ch/en/evoting-community/help-and-contact/faq))
- Tajność głosu: ⚠️ — deklarowana pełna tajność (indywidualna i uniwersalna weryfikowalność), ale poprzednia wersja (rozwijana ze Scytl) miała w 2019 r. wykrytą poważną lukę kryptograficzną przed wdrożeniem produkcyjnym. ([swisspost-digital.ch — publikacje i kod](https://swisspost-digital.ch/en/solutions/e-voting/publications-and-source-code))
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — zintegrowany z kartami do głosowania wysyłanymi pocztą wraz z kodami weryfikacyjnymi. ([swisspost-digital.ch — FAQ](https://swisspost-digital.ch/en/evoting-community/help-and-contact/faq))
- Niezależność cyfrowa: ❌ — jeden dostawca (Poczta Szwajcarska) posiada wyłączne prawa do kodu niezbędnego do niezależnego rozwoju systemu. ([swisspost-digital.ch — FAQ](https://swisspost-digital.ch/en/evoting-community/help-and-contact/faq))
- Źródła: [swisspost-digital.ch — publikacje i kod źródłowy](https://swisspost-digital.ch/en/solutions/e-voting/publications-and-source-code), [gitlab.com/swisspost-evoting](https://gitlab.com/swisspost-evoting/e-voting/e-voting), [security whitepaper](https://gitlab.com/swisspost-evoting/e-voting/e-voting-documentation/-/blob/master/Product/Security%20Whitepaper%20of%20the%20Swiss%20Post%20Voting%20System.md)

### 27. Genewa — CHVote

**Kraj:** Szwajcaria (kanton Genewa) · **Język implementacji:** Java (zgodnie z repozytorium kantonu)

Open-source'owy system kantonalny rozwijany od 2003 r., publikujący kod na GitHub kantonu Genewa; wycofany w 2020 r. z powodów budżetowych i obaw o bezpieczeństwo.

- Niski koszt administrowania: ❌ — kanton zrezygnował z systemu m.in. z powodu wysokich kosztów utrzymania.
- Uniwersalność: ✅ — używany do referendów i wyborów kantonalnych o różnej strukturze przez kilkanaście lat. ([valimised.ee — e-voting w innych krajach](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowany do szwajcarskiej/genewskiej ordynacji.
- Weryfikowalność i otwartość: ✅ — kod publikowany otwarcie na GitHub kantonu. ([republique-et-canton-de-geneve.github.io/chvote-1-0](https://republique-et-canton-de-geneve.github.io/chvote-1-0/index-en.html))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowany z kantonalnym rejestrem wyborców.
- Tajność głosu: ⚠️ — deklarowana, ale kanton wycofał się z powodu obaw o bezpieczeństwo całej rodziny systemów szwajcarskich w 2019 r. ([valimised.ee](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries))
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — zintegrowany z kartami do głosowania i kodami weryfikacyjnymi.
- Niezależność cyfrowa: ❌ — mimo otwartego kodu, system był ściśle powiązany z infrastrukturą kantonu Genewa i ostatecznie porzucony.
- Źródła: [republique-et-canton-de-geneve.github.io/chvote-1-0](https://republique-et-canton-de-geneve.github.io/chvote-1-0/index-en.html), [valimised.ee — e-voting w innych krajach](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries)

### 28. Norwegia (pilotaż 2011–2013)

**Kraj:** Norwegia · **Język implementacji:** nieznany publicznie (dostawcy: Scytl, ErgoGroup)

Pilotaż i-votingu przeprowadzony w wybranych gminach w wyborach lokalnych 2011 i parlamentarnych 2013, zaniechany po dwóch cyklach z powodu obaw politycznych i technicznych.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji dot. kosztów.
- Uniwersalność: ⚠️ — ograniczone do wybranych gmin i typów wyborów objętych pilotażem.
- Dowolna ordynacja wyborcza: ⚠️ — dostosowane do norweskiej ordynacji.
- Weryfikowalność i otwartość: ⚠️ — częściowo publikowany kod jako warunek kontraktu publicznego, ale system nie był w pełni open source.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowany z norweskim rejestrem wyborców, dopuszczał ponowne głosowanie nadpisujące poprzednie.
- Tajność głosu: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — integracja z norweską infrastrukturą identyfikacji elektronicznej.
- Niezależność cyfrowa: ❌ — zależny od zewnętrznych dostawców (Scytl, ErgoGroup), ostatecznie zaniechany.
- Źródła: [valimised.ee — e-voting w innych krajach](https://www.valimised.ee/en/internet-voting/more-about-i-voting/e-voting-other-countries), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 29. Nowa Południowa Walia (Australia) — iVote

**Kraj:** Australia (stan NSW) · **Język implementacji:** nieznany publicznie (dostawca: Scytl, później rozwój wewnętrzny)

System internetowego głosowania używany 2011–2021, głównie przez wyborców niepełnosprawnych, mieszkających daleko od lokali wyborczych i za granicą; wycofany po awarii systemu w wyborach samorządowych w grudniu 2021 r.

- Niski koszt administrowania: ❌ — wysokie koszty utrzymania systemu przez ponad dekadę bez rozwiązania problemów technicznych.
- Uniwersalność: ⚠️ — ograniczony do określonych kategorii wyborców (niepełnosprawni, odległe tereny, wyborcy za granicą), nie dla ogółu społeczeństwa. ([rappler.com](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowany do australijskiej ordynacji preferencyjnej stanu NSW.
- Weryfikowalność i otwartość: ❌ — kod źródłowy zamknięty (własność dostawcy/komisji wyborczej NSW).
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowany z rejestrem wyborców NSW.
- Tajność głosu: ❓ — brak zweryfikowanego, niezależnego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ⚠️ — weryfikacja przez dane osobowe podawane online, bez silnego e-ID.
- Niezależność cyfrowa: ❌ — zamknięty system jednego dostawcy, ostatecznie wycofany z powodu awarii w 2021 r. ([rappler.com](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/))
- Źródła: [rappler.com — Which countries have conducted online elections](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 30. Francja — i-voting dla obywateli za granicą

**Kraj:** Francja (dostawca: Voxaly/Docaposte) · **Język implementacji:** nieznany publicznie

System głosowania internetowego dla Francuzów mieszkających za granicą, używany w wyborach do Zgromadzenia Narodowego (okręgi zagraniczne) oraz częściowo konsularnych.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ❌ — ograniczony wyłącznie do okręgów wyborczych obywateli za granicą. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowany do francuskiej ordynacji dla okręgów zagranicznych.
- Weryfikowalność i otwartość: ❌ — system własnościowy, kod niepubliczny.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowany z francuskim rejestrem konsularnym wyborców za granicą.
- Tajność głosu: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — weryfikacja przez rejestr konsularny/dane osobowe.
- Niezależność cyfrowa: ❌ — zależny od jednego prywatnego dostawcy.
- Źródła: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 31–32. Panama i Meksyk (INE) — i-voting dla obywateli za granicą

**Kraj:** Panama, Meksyk · **Język implementacji:** nieznany publicznie

Systemy głosowania internetowego umożliwiające obywatelom mieszkającym za granicą oddanie głosu bez konieczności powrotu do kraju lub wizyty w placówce konsularnej.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ❌ — ograniczone wyłącznie do wyborców za granicą. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowane do lokalnych ordynacji.
- Weryfikowalność i otwartość: ❌ — systemy własnościowe; meksykański INE publikuje część dokumentacji audytowej, ale nie pełny kod źródłowy.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowane z rejestrami wyborców za granicą.
- Tajność głosu: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — weryfikacja przez rejestr konsularny/dokumenty tożsamości.
- Niezależność cyfrowa: ❌ — zależne od zewnętrznych dostawców/infrastruktury państwowej zamkniętej dla audytu publicznego.
- Źródła: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 33. Armenia — pilotaże i-votingu dla diaspory

**Kraj:** Armenia · **Język implementacji:** nieznany publicznie

Pilotażowe wdrożenia głosowania internetowego dla ormiańskiej diaspory, wymieniane jako jeden z "utrwalonych przypadków" i-votingu w raporcie International IDEA.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ❌ — ograniczone do diaspory/wyborców za granicą. ([idea.int — baza danych](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349))
- Dowolna ordynacja wyborcza: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikowalność i otwartość: ❓ — bardzo ograniczona publiczna dokumentacja techniczna.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Tajność głosu: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Niezależność cyfrowa: ❓ — brak zweryfikowanego źródła w tej sesji.
- Źródła: [idea.int — Database: internet voting systems by country](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349), [idea.int — Online Voting: Current and Future Practices](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 34. Zjednoczone Emiraty Arabskie — aplikacja mobilna do głosowania

**Kraj:** ZEA · **Język implementacji:** nieznany publicznie

Aplikacja mobilna z funkcją rozpoznawania twarzy używana do wyborów Federalnej Rady Narodowej, dla ograniczonego elektoratu uprawnionego do głosowania.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ❌ — bardzo ograniczony elektorat (Federalna Rada Narodowa ma ograniczone kompetencje i elektorat), nie dotyczy pełnych wyborów powszechnych. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Dowolna ordynacja wyborcza: ❌ — dostosowana wyłącznie do specyficznej, ograniczonej ordynacji ZEA.
- Weryfikowalność i otwartość: ❌ — system własnościowy, brak publicznego kodu źródłowego.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowany z rejestrem uprawnionych wyborców.
- Tajność głosu: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — silna weryfikacja biometryczna (rozpoznawanie twarzy). ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Niezależność cyfrowa: ❌ — zamknięty system rządowy.
- Źródła: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650)

### 35. Indie — EVM (ECIL/BEL)

**Kraj:** Indie (produkcja: Bharat Electronics Limited i Electronics Corporation of India Limited) · **Język implementacji:** niepubliczny firmware wypalany na mikrokontrolerach jednorazowo programowalnych

Autonomiczne, niesieciowe maszyny do głosowania (Electronic Voting Machines) używane we wszystkich wyborach krajowych od lat 90., od 2017 r. powszechnie wyposażone w VVPAT (papierowe potwierdzenie).

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji dot. kosztów całkowitych (produkcja i utrzymanie floty milionów maszyn).
- Uniwersalność: ❌ — obsługuje wyłącznie proste głosowanie na jednego kandydata przyciskiem, brak wsparcia dla list partyjnych, skal czy referendów wieloopcjowych. ([wyrok Sądu Najwyższego Indii, 26.04.2024](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf))
- Dowolna ordynacja wyborcza: ❌ — zaprojektowane wyłącznie pod indyjski system FPTP.
- Weryfikowalność i otwartość: ❌ — Sąd Najwyższy Indii wielokrotnie orzekł, że kod źródłowy EVM nie może zostać ujawniony publicznie. ([indianexpress.com](https://preprod.indianexpress.com/article/india/supreme-court-vvpat-order-evm-9287698/), [thehindu.com](https://www.thehindu.com/news/national/sc-asks-ec-five-queries-in-evm-vvpat-case-is-a-microcontroller-used-in-evms-is-one-time-programmable-or-not/article68100796.ece))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — jednorazowe naciśnięcie przycisku weryfikowane przez urzędnika komisji na podstawie listy wyborców w lokalu.
- Tajność głosu: ✅ — maszyny działają w pełni offline, bez łączności sieciowej, co eliminuje zdalne wektory deanonimizacji. ([wyrok Sądu Najwyższego Indii](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf))
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — tradycyjna weryfikacja na liście wyborców w lokalu wyborczym przez urzędnika komisji.
- Niezależność cyfrowa: ❌ — zamknięty, jednorazowo programowalny firmware produkowany wyłącznie przez dwie państwowe spółki, kod celowo utajniony decyzją sądu. ([wyrok Sądu Najwyższego Indii](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf))
- Źródła: [adrindia.org — wyrok Sądu Najwyższego Indii ws. EVM/VVPAT (26.04.2024)](https://www.adrindia.org/sites/default/files/VVPAT_Judgment_dated_26-04-24.pdf), [indianexpress.com — SC says source code cannot be disclosed](https://preprod.indianexpress.com/article/india/supreme-court-vvpat-order-evm-9287698/), [thehindu.com](https://www.thehindu.com/news/national/sc-asks-ec-five-queries-in-evm-vvpat-case-is-a-microcontroller-used-in-evms-is-one-time-programmable-or-not/article68100796.ece)

### 36. Brazylia — Urna Eletrônica (TSE)

**Kraj:** Brazylia (Tribunal Superior Eleitoral) · **Język implementacji:** historycznie własnościowy VirtuOS/Windows CE, od 2008 system na bazie GNU/Linux

Autonomiczne maszyny DRE (bez połączenia z internetem podczas głosowania) używane we wszystkich lokalach wyborczych od 2000 r.; nie posiadają papierowego potwierdzenia głosu (VVPAT odrzucony przez TSE).

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji dot. całkowitych kosztów floty maszyn.
- Uniwersalność: ⚠️ — obsługuje różne poziomy wyborów (prezydenckie, parlamentarne, lokalne) przez wpisywanie numeru kandydata, ale nie obsługuje referendów wieloopcjowych czy skal ocen. ([tse.jus.br — auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability))
- Dowolna ordynacja wyborcza: ⚠️ — zaprojektowana pod brazylijski system list otwartych i wybory większościowe, nie jest uniwersalnym silnikiem.
- Weryfikowalność i otwartość: ❌ — kod udostępniany wyłącznie akredytowanym audytorom pod NDA, w kontrolowanym środowisku bez internetu, bez publikacji wyników audytu. ([ndi.org — case study Brazil](https://www.ndi.org/sites/default/files/4_Brazil.pdf), [tse.jus.br — auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — weryfikacja na podstawie brazylijskiego rejestru wyborców w lokalu.
- Tajność głosu: ✅ — maszyny offline podczas głosowania, głosy zapisywane w losowej kolejności niepowiązanej z kolejnością głosowania. ([ndi.org — case study Brazil](https://www.ndi.org/sites/default/files/4_Brazil.pdf))
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — tradycyjna weryfikacja w lokalu wyborczym.
- Niezależność cyfrowa: ❌ — jeden, zamknięty system utrzymywany centralnie przez TSE, bez możliwości niezależnego wdrożenia. ([tse.jus.br — auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability))
- Źródła: [international.tse.jus.br — Auditability](https://international.tse.jus.br/en/electronic-ballot-box/auditability), [ndi.org — Case Study Report on Brazil Electronic Voting](https://www.ndi.org/sites/default/files/4_Brazil.pdf), [tse.jus.br — Resolução nº 23.673/2021](https://www.tse.jus.br/legislacao/compilada/res/2021/resolucao-no-23-673-14-de-dezembro-de-2021)

### 37. Belgia — elektroniczne drukarki kart do głosowania

**Kraj:** Belgia · **Język implementacji:** nieznany publicznie

System, w którym wyborca dokonuje wyboru na ekranie, a maszyna drukuje kartę z zakodowanym kodem kreskowym, którą wyborca wrzuca do urny i która jest następnie skanowana.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ⚠️ — obsługuje belgijski system list partyjnych, dostosowany do lokalnej ordynacji. ([idea.int — baza danych](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowany wyłącznie do belgijskiego systemu list partyjnych.
- Weryfikowalność i otwartość: ❌ — system własnościowy.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — weryfikacja w lokalu wyborczym.
- Tajność głosu: ✅ — karta z kodem kreskowym nie zawiera danych identyfikujących wyborcę.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — tradycyjna weryfikacja w lokalu.
- Niezależność cyfrowa: ❌ — zamknięty system dostawcy.
- Źródła: [idea.int — Database: voting technology by country](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349)

### 38. Wenezuela — Smartmatic i lokalni dostawcy

**Kraj:** Wenezuela · **Język implementacji:** nieznany publicznie

DRE z drukowanym potwierdzeniem papierowym, dostarczane przez Smartmatic do 2017 r., następnie przez innych/lokalnych dostawców w kontekście politycznie spornych wyborów.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ⚠️ — dostosowane do wenezuelskiego systemu wyborczego (prezydent, zgromadzenie narodowe). ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Dowolna ordynacja wyborcza: ⚠️ — dostosowane wyłącznie do lokalnej ordynacji.
- Weryfikowalność i otwartość: ❌ — system własnościowy, przedmiot licznych sporów politycznych o wiarygodność audytu.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — weryfikacja na podstawie krajowego rejestru wyborców.
- Tajność głosu: ✅ — maszyny DRE z papierowym potwierdzeniem, offline podczas głosowania.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — weryfikacja biometryczna w lokalu wyborczym (odcisk palca).
- Niezależność cyfrowa: ❌ — silne uzależnienie od jednego, zagranicznego dostawcy technologii, przedmiot kontrowersji politycznych. ([mdpi.com — Electronic Voting Worldwide](https://www.mdpi.com/2078-2489/16/8/650))
- Źródła: [mdpi.com — Electronic Voting Worldwide: The State of the Art](https://www.mdpi.com/2078-2489/16/8/650)

### 39. Filipiny — Smartmatic

**Kraj:** Filipiny · **Język implementacji:** nieznany publicznie

Skanery optyczne kart do głosowania (OMR) dostarczane przez Smartmatic, używane we wszystkich wyborach krajowych od 2010 r.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ⚠️ — dostosowane do filipińskiego systemu wyborczego z bardzo dużą liczbą stanowisk wybieralnych na jednej karcie.
- Dowolna ordynacja wyborcza: ⚠️ — dostosowane wyłącznie do lokalnej ordynacji.
- Weryfikowalność i otwartość: ❌ — system własnościowy.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — weryfikacja na podstawie krajowego rejestru wyborców (COMELEC).
- Tajność głosu: ✅ — karty papierowe skanowane offline w lokalu.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — tradycyjna weryfikacja w lokalu wyborczym.
- Niezależność cyfrowa: ❌ — silne uzależnienie od jednego zagranicznego dostawcy.
- Źródła: brak dodatkowego zweryfikowanego linku w tej sesji poza ogólną wiedzą branżową o dostawcy Smartmatic.

### 40. USA — Dominion / ES&S / Hart InterCivic / Unisyn / MicroVote / Clear Ballot

**Kraj:** USA (różne stany) · **Język implementacji:** nieznany publicznie (systemy zamknięte, poza częściowo otwartymi komponentami Clear Ballot)

Różnorodne maszyny DRE i skanery optyczne używane w poszczególnych hrabstwach/stanach USA; większość stanów wymaga obecnie papierowego śladu (VVPAT) i część prowadzi statystyczne audyty ryzyka (risk-limiting audits).

- Niski koszt administrowania: ❌ — bardzo wysokie koszty kontraktów wyborczych w USA. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Uniwersalność: ⚠️ — obsługują typowe amerykańskie karty wielo-wyścigowe, różne stany różne konfiguracje.
- Dowolna ordynacja wyborcza: ⚠️ — ograniczone do metod stosowanych w wyborach amerykańskich (FPTP, niektóre RCV).
- Weryfikowalność i otwartość: ❌ — w większości systemy zamknięte; niezależne audyty utrudnione przez tajemnicę handlową dostawców. ([osvtac.github.io — State of the Art Briefing](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — integracja ze stanowymi rejestrami wyborców.
- Tajność głosu: ✅ — maszyny offline w lokalu, karty papierowe/VVPAT w większości stanów.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — tradycyjna weryfikacja w lokalu wyborczym wg przepisów stanowych.
- Niezależność cyfrowa: ❌ — silne uzależnienie od nielicznych dostawców, brak interoperacyjności między systemami różnych producentów.
- Źródła: [osvtac.github.io — State of the Art Briefing on Open Source Voting Systems](https://osvtac.github.io/files/meetings/2020/2020-03-12/packet/DT_OSV_State_of_Art_Briefing_Feb_2020.pdf)

### 41. Korea Południowa — K-Voting

**Kraj:** Korea Południowa (Narodowa Komisja Wyborcza) · **Język implementacji:** nieznany publicznie

System głosowania internetowego prowadzony przez koreańską komisję wyborczą dla głosowań organizacyjnych/uczelnianych/spółdzielczych — nie dla wiążących wyborów krajowych, które pozostają papierowe.

- Niski koszt administrowania: ✅ — relatywnie tani dla mniejszych organizacji korzystających z usługi rządowej.
- Uniwersalność: ⚠️ — obsługuje różne typy głosowań organizacyjnych, ale nie jest używany do pełnych wyborów krajowych. ([idea.int — baza danych](https://www.idea.int/data-tools/data/question-region?countries=all&database_theme=327&question_id=9349))
- Dowolna ordynacja wyborcza: ⚠️ — ograniczone do typowych metod głosowań organizacyjnych.
- Weryfikowalność i otwartość: ❌ — system własnościowy/rządowy, kod niepubliczny.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — zintegrowany z listami członkowskimi organizacji korzystających z usługi.
- Tajność głosu: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ✅ — integracja z koreańską infrastrukturą identyfikacji cyfrowej.
- Niezależność cyfrowa: ❌ — usługa scentralizowana, prowadzona wyłącznie przez jedną instytucję rządową.
- Źródła: [idea.int — Online Voting: Current and Future Practices (Korea Południowa — "under discussion")](https://www.idea.int/sites/default/files/2025-11/online-voting-current-and-future-practices.pdf)

### 42. Kanada — komercyjne SaaS w wyborach samorządowych (Ontario)

**Kraj:** Kanada · **Język implementacji:** nieznany publicznie

Wybory samorządowe w prowincji Ontario i innych regionach Kanady korzystają z komercyjnych platform SaaS (m.in. Simply Voting, Dominion Internet Voting) do głosowania internetowego; brak jednolitego systemu krajowego.

- Niski koszt administrowania: ⚠️ — relatywnie tanie dla małych gmin w porównaniu do budowy własnego systemu, ale generuje powtarzające się opłaty dla dostawcy.
- Uniwersalność: ⚠️ — obsługuje typowe kanadyjskie karty samorządowe (wielu kandydatów na wiele stanowisk), różne konfiguracje w zależności od dostawcy.
- Dowolna ordynacja wyborcza: ⚠️ — ograniczone do metod wspieranych przez danego dostawcę SaaS.
- Weryfikowalność i otwartość: ❌ — systemy własnościowe różnych dostawców, brak jednolitej, publicznej kryptograficznej weryfikowalności.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ✅ — integracja z lokalnymi rejestrami wyborców gminy.
- Tajność głosu: ❓ — deklarowana przez dostawców, brak niezależnie zweryfikowanego źródła w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ⚠️ — zwykle kod PIN wysyłany pocztą, bez silnej integracji z federalnym e-ID.
- Niezależność cyfrowa: ❌ — zależność od komercyjnych dostawców zewnętrznych, różnych dla różnych gmin.
- Źródła: [rappler.com — Which countries have conducted online elections (Ontario, Canada)](https://www.rappler.com/newsbreak/iq/list-countries-conducted-online-elections-have-they-worked/)

### 43. Rosja — moskiewski system blockchain e-voting

**Kraj:** Rosja (Departament Technologii Informacyjnych miasta Moskwy) · **Język implementacji:** JavaScript (klient), PHP (serwer), Solidity (smart kontrakty na Ethereum)

System i-votingu użyty częściowo w wyborach do Dumy Miejskiej Moskwy we wrześniu 2019 r. (3 z 45 okręgów), oparty na blockchainie Ethereum; kod częściowo publikowany do testów publicznych przed wyborami.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji dot. kosztów.
- Uniwersalność: ❌ — testowany wyłącznie na wybranych okręgach lokalnych wyborów miejskich.
- Dowolna ordynacja wyborcza: ❌ — dostosowany wyłącznie do konkretnych wyborów miejskich.
- Weryfikowalność i otwartość: ⚠️ — część kodu publikowana na GitHub przed wyborami do testów, ale bez formalnej specyfikacji protokołu i bez publikacji w języku innym niż rosyjski. ([members.loria.fr — Gaudry, Breaking the encryption scheme](https://members.loria.fr/PGaudry/moscow/), [arxiv.org/pdf/1908.05127](https://arxiv.org/pdf/1908.05127))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Tajność głosu: ❌ — francuscy badacze (Pierrick Gaudry, Alexander Golovnev) wykazali, że pierwsza wersja szyfrowania mogła zostać złamana w ok. 20 minut na zwykłym komputerze, co pozwalało ujawnić głos każdego wyborcy natychmiast po jego oddaniu; po poprawce wykazano drugą lukę pozwalającą częściowo odczytać wynik. ([members.loria.fr](https://members.loria.fr/PGaudry/moscow/), [arxiv.org/pdf/1908.05127](https://arxiv.org/pdf/1908.05127), [zdnet.com](https://www.zdnet.com/article/moscows-blockchain-voting-system-cracked-a-month-before-election/), [coindesk.com](https://www.coindesk.com/markets/2019/08/16/moscow-blockchain-voting-system-completely-insecure-says-researcher))
- Weryfikacja tożsamości i delegacja uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Niezależność cyfrowa: ❌ — system zamknięty, prowadzony centralnie przez miejski departament IT, ze zmianami protokołu wprowadzanymi tuż przed wyborami bez pełnej dokumentacji. ([members.loria.fr](https://members.loria.fr/PGaudry/moscow/))
- Źródła: [members.loria.fr — Breaking the encryption scheme of the Moscow internet voting system (Pierrick Gaudry)](https://members.loria.fr/PGaudry/moscow/), [arxiv.org/pdf/1908.05127](https://arxiv.org/pdf/1908.05127), [fc20.ifca.ai — pełna publikacja recenzowana](https://fc20.ifca.ai/preproceedings/178.pdf), [zdnet.com](https://www.zdnet.com/article/moscows-blockchain-voting-system-cracked-a-month-before-election/), [coindesk.com](https://www.coindesk.com/markets/2019/08/16/moscow-blockchain-voting-system-completely-insecure-says-researcher)

### 44. Voatz

**Kraj:** USA (Boston, Massachusetts) · **Język implementacji:** aplikacja mobilna (zaciemniony/obfuskowany kod, nieujawniony publicznie)

Zamknięta, komercyjna aplikacja mobilna do głosowania wykorzystująca permissioned blockchain, biometrię i moduły sprzętowe do przechowywania kluczy; użyta w pilotażach m.in. w Wirginii Zachodniej (żołnierze za granicą, wybory połówkowe 2018), hrabstwie Denver i Utah County.

- Niski koszt administrowania: ❌ — koszty pilotaży i wsparcia technicznego ponoszone przez jurysdykcje, przy jednoczesnym braku publicznej dokumentacji ułatwiającej samodzielne wdrożenie. ([usenix.org — The Ballot is Busted Before the Blockchain](https://www.usenix.org/system/files/sec20-specter.pdf))
- Uniwersalność: ❌ — ograniczone do wąskich pilotaży, nie testowane na pełną skalę i różnorodność kart wyborczych.
- Dowolna ordynacja wyborcza: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikowalność i otwartość: ❌ — kod aplikacji celowo zaciemniony, serwer nigdy nie został udostępniony do analizy; badacze MIT musieli odtwarzać działanie systemu metodą inżynierii wstecznej. ([internetpolicy.mit.edu — FAQ](https://internetpolicy.mit.edu/faq-on-the-security-analysis-of-voatz/), [csail-dev-2025.csail.mit.edu](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app))
- Jeden głos na uprawnionego + odwoływanie uprawnień: ⚠️ — deklarowane przez firmę, ale niezależnie niezweryfikowane wobec braku dostępu do serwera. ([usenix.org — The Ballot is Busted Before the Blockchain](https://www.usenix.org/system/files/sec20-specter.pdf))
- Tajność głosu: ❌ — badacze MIT wykazali, że pasywny obserwator sieci (np. dostawca internetu) mógł w niektórych konfiguracjach ustalić, jak zagłosował użytkownik, poprzez atak kanałem bocznym. ([usenix.org — The Ballot is Busted Before the Blockchain](https://www.usenix.org/system/files/sec20-specter.pdf), [csail-dev-2025.csail.mit.edu](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app))
- Weryfikacja tożsamości i delegacja uprawnień: ⚠️ — weryfikacja przez zewnętrznego dostawcę tożsamości, co samo w sobie stworzyło dodatkowe ryzyko prywatności (dostęp strony trzeciej do zdjęć/danych z dowodu osobistego). ([csail-dev-2025.csail.mit.edu](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app))
- Niezależność cyfrowa: ❌ — w pełni zamknięty, zależny od jednej firmy, która odmówiła publikacji kodu źródłowego mimo apeli społeczności bezpieczeństwa. ([internetpolicy.mit.edu — FAQ](https://internetpolicy.mit.edu/faq-on-the-security-analysis-of-voatz/))
- Źródła: [usenix.org — The Ballot is Busted Before the Blockchain: A Security Analysis of Voatz](https://www.usenix.org/system/files/sec20-specter.pdf), [internetpolicy.mit.edu — FAQ on the Security Analysis of Voatz](https://internetpolicy.mit.edu/faq-on-the-security-analysis-of-voatz/), [csail-dev-2025.csail.mit.edu — MIT researchers identify security vulnerabilities](https://www.csail-dev-2025.csail.mit.edu/news/mit-researchers-identify-security-vulnerabilities-voting-app)

### 45. Kaspersky Polys

**Kraj:** Rosja · **Język implementacji:** nieznany publicznie

Komercyjny SaaS do głosowań na blockchainie oferowany przez Kaspersky, wykorzystywany częściowo przy pilotażowych konsultacjach w Moskwie (2019); główne głosowanie miejskie w Moskwie w 2019/2020 r. korzystało jednak z odrębnego, własnego systemu miasta (patrz pozycja 43), a nie z samego Polys.

- Niski koszt administrowania: ❓ — brak zweryfikowanego źródła w tej sesji.
- Uniwersalność: ❓ — brak zweryfikowanego źródła w tej sesji.
- Dowolna ordynacja wyborcza: ❓ — brak zweryfikowanego źródła w tej sesji.
- Weryfikowalność i otwartość: ❌ — system własnościowy, kod niepubliczny.
- Jeden głos na uprawnionego + odwoływanie uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Tajność głosu: ❓ — deklarowana przez dostawcę (szyfrowanie homomorficzne na blockchainie), ale niezależnie niezweryfikowana w tej sesji.
- Weryfikacja tożsamości i delegacja uprawnień: ❓ — brak zweryfikowanego źródła w tej sesji.
- Niezależność cyfrowa: ❌ — zamknięty, komercyjny produkt jednego dostawcy.
- Źródła: brak zweryfikowanego linku w tej sesji — ocena na podstawie ogólnodostępnej wiedzy o produkcie Kaspersky Polys; zob. też kontekst moskiewskiego pilotażu w pozycji 43.
