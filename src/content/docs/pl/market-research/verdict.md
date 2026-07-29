---
title: Werdykt
description: Wniosek z badania rynku — żaden z zbadanych systemów nie spełnia wszystkich wymagań Suffragio i dlaczego.
---

Ta strona syntetyzuje [Wyniki](/suffragio-spec/pl/market-research/findings/) i [Macierz porównawczą](/suffragio-spec/pl/market-research/matrix/) w bezpośrednią odpowiedź na trzy pytania: który z istniejących systemów jest najbliżej spełnienia wymagań Suffragio, jak daleko mu do „ideału” oraz dlaczego ta różnica uzasadnia budowę nowego systemu zamiast adaptacji istniejącego.

## Żaden system nie spełnia wszystkich 8 wymagań

Spośród wszystkich 45 zbadanych systemów żaden nie zbliża się do „same ✅” na tle ośmiu wymagań Suffragio (pełne nazwy — patrz [Motywacja i wymagania](/suffragio-spec/pl/motivation/)). Widoczne są dwa strukturalne klastry:

- **Open-source'owe / akademickie narzędzia kryptograficzne** (rodzina Helios, Belenios, Zeus) — mocne pod względem weryfikowalności, tajności głosu i niezależności cyfrowej, ale słabe w uniwersalności, elastyczności ordynacji wyborczej oraz weryfikacji tożsamości/delegacji uprawnień.
- **Systemy wdrożone przez państwa** (estoński IVXV, Swiss Post e-voting, genewski CHVote) — mocne pod względem uniwersalności, weryfikacji tożsamości i realnej skali produkcyjnej, ale słabe lub porażające w niezależności cyfrowej, a niekiedy także w gwarancjach weryfikowalności czy tajności (np. podatność po stronie klienta w Estonii, błąd kryptograficzny Scytl/Swiss Post z 2019 r. wykryty i naprawiony przed wdrożeniem).

## Najlepszy gotowy kandydat

**Belenios** (lub blisko spokrewnione Helios i Zeus) to najlepszy uniwersalny kandydat, jeśli budowa nowego systemu nie wchodzi w grę:

| Wymaganie | Belenios |
|---|---|
| Niski koszt administrowania | ✅ darmowy, samodzielnie hostowalny |
| Uniwersalność | ⚠️ obsługuje kilka typów kart, ale nie pełną złożoność wieloobwodową |
| Dowolna ordynacja wyborcza | ⚠️ ograniczona do typów głosowania wspieranych przez oprogramowanie |
| Weryfikowalność i otwartość | ✅ licencja AGPLv3, formalnie zweryfikowane bezpieczeństwo protokołu |
| Jeden głos na uprawnionego + odwoływanie uprawnień | ⚠️ lista wyborców konfigurowana ręcznie, brak mechanizmu odwoływania |
| Tajność głosu | ✅ szyfrowanie homomorficzne z rozproszonym zaufaniem (trustees) |
| Weryfikacja tożsamości i delegacja uprawnień | ❌ tylko logowanie przez e-mail / konto Google / CAS Inria |
| Niezależność cyfrowa | ✅ samodzielnie hostowalny, brak zależności od dostawcy |

Warto też wspomnieć o **TrustTheVote / OSET Institute (ElectOS)** (pełna ocena w `@/home/witek/Projekty/suffragio/src/content/docs/pl/market-research/findings.md` — sekcja poświęcona pozycji 18) — jedynym zbadanym systemie z **zerem jednoznacznych ❌**, ponieważ jest to pełny pakiet administracji wyborami (rejestracja wyborców, tabulacja, firmware urządzeń do głosowania), a nie wąski protokół kryptograficzny. Jest płytki, a nie doskonały na większości osi (przeważają ⚠️), ale sama ta szerokość pokrycia całego stosu administracji wyborami jest znacząca.

Dla realnego, rządowego wdrożenia produkcyjnego szczególnie wyróżnia się **Swiss Post e-voting** — najbardziej rygorystycznie zaprojektowany system w tym badaniu: opublikowane komponenty kryptograficzne, coroczne publiczne testy penetracyjne z nagrodami do 250 000 CHF oraz weryfikowalność zarówno indywidualna, jak i uniwersalna. Ale zawodzi w niezależności cyfrowej (jeden dostawca posiada wyłączne prawa do kodu niezbędnego do niezależnego rozwoju systemu), a koszt administrowania jest bardzo wysoki.

## Jak daleko Beleniosowi do „ideału”

- **Brak warstwy tożsamości/delegacji.** Uwierzytelnianie odbywa się przez e-mail, konto Google lub CAS Inria — nie przez silne e-ID — i nie istnieje pojęcie delegowanych lub płynnych (liquid) uprawnień do głosowania.
- **Brak odwoływania uprawnień.** Listy wyborców to statyczne, ręcznie skonfigurowane migawki; nie ma mechanizmu odebrania uprawnień wyborcy w trakcie wyborów.
- **Wąskie wsparcie dla uniwersalności i ordynacji wyborczej.** Belenios obsługuje typy głosowania zaimplementowane przez autorów (wybór jedno-/wielokrotny, niektóre warianty referendalne), a nie dowolny silnik ordynacji wyborczej — brak natywnego wsparcia dla STV, podziału mandatów wg list partyjnych itp.
- **Luka w modelu zaufania.** Tabulacja homomorficzna z rozproszonymi trustees daje silną kryptograficzną tajność treści głosu, ale nic nie mówi o integralności uprawnień wyborców, która pozostaje zewnętrznie zaufana (lista dostarczona przez organizatora).

## Uzasadnienie budowy nowego systemu

Powyższe luki nie są powierzchownymi brakującymi funkcjami, które dałoby się „dokleić” do Beleniosa (ani do żadnego innego pojedynczego zbadanego systemu) — wynikają z fundamentalnych wyborów architektonicznych:

- **Tożsamość, delegacja i odwoływanie uprawnień** wymagają warstwy uprawnień/poświadczeń *pod* warstwą kryptograficzną głosowania. Belenios, podobnie jak Helios i Zeus, nigdy nie został do tego zaprojektowany — zakłada zewnętrznie dostarczoną, statyczną, zaufaną listę wyborców.
- **Dowolna ordynacja wyborcza** wymaga potraktowania tabulacji jako podłączalnego, ogólnego komponentu. Belenios (i pokrewne systemy) zamiast tego zaszywa mały, ustalony zestaw wspieranej semantyki kart do głosowania w tym samym kodzie co kryptografię.
- **Niezależność cyfrowa połączona z silną gwarancją tożsamości** nie jest właściwie adresowana przez żaden zbadany system. Systemy osiągające niezależność (samodzielnie hostowane, open source, bez dostawcy) robią to kosztem gwarancji tożsamości (Belenios, Helios, Zeus). Systemy osiągające silną gwarancję tożsamości (Estonia, Szwajcaria) robią to poprzez ścisłe związanie z infrastrukturą PKI/e-ID jednego państwa — co jest dokładnie tym, co wyklucza niezależność cyfrową.

Żaden zbadany system nie łączy jednocześnie: (a) generycznego, podłączalnego wsparcia dla dowolnej ordynacji wyborczej, (b) warstwy tożsamości, delegacji i odwoływania uprawnień oraz (c) pełnej kryptograficznej weryfikowalności i tajności, pozostając przy tym samodzielnie hostowalnym i wolnym od zależności od jednego dostawcy lub jednego państwa. Ta kombinacja to dokładnie luka, którą mają wypełnić wymagania Suffragio — pełny zestaw wymagań i uzasadnienie w [Motywacji i wymaganiach](/suffragio-spec/pl/motivation/).
