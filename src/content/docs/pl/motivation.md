---
title: Motywacja i wymagania
description: Dlaczego projekt Suffragio stworzył nowoczesny system do głosowania elektronicznego oraz jakie cele i wymagania mu przyświecały.
---

Suffragio to propozycja nowoczesnego systemu wyborczego. Poniżej opisujemy motywację, jaka stała za jego powstaniem, oraz cele i wymagania, które kształtowały jego projekt.

## Wymagania

### Niski koszt organizacji wyborów

Członek Państwowej Komisji Wyborczej (lub dowolnej innej organizacji) powinien mieć możliwość zaprojektowania, opublikowania i zweryfikowania wyborów oraz ich wyników bez ponoszenia znaczących kosztów.

### Uniwersalność

System powinien być w stanie obsłużyć organizację dowolnych wyborów — w szczególności krajowych, ale nie tylko. Dotyczy to różnego rodzaju pytań (tak/nie, wybór jednokrotny, wybór wielokrotny, skala 1–10 itd.), a także nietrywialnych kart wyborczych (referenda, partyjne listy wyborcze, głosowanie na osoby, różna treść kart dla tych samych wyborów w różnych okręgach wyborczych).

### Obsługa dowolnej ordynacji wyborczej

System prawny danego kraju ani rodzaj wyborów nie powinny być argumentem za tworzeniem kolejnego, odrębnego systemu. Wynik — kto zostaje prezydentem lub kto zasiądzie w parlamencie — powinien dać się wyliczyć z zebranych głosów według dowolnie obowiązującej ordynacji.

### Weryfikowalność i otwartość

Każdy obywatel powinien być w stanie samodzielnie i lokalnie zweryfikować wynik wyborów. Oznacza to pełny dostęp do wszystkich oddanych głosów oraz dostęp do kodu źródłowego wszystkich elementów systemu wyborczego.

### Jedna karta na uprawnionego wyborcę

Każda osoba uprawniona do głosowania może pobrać kartę do głosowania tylko raz i wyłącznie dla swojego okręgu wyborczego.

### Tajność głosu

Nikt — w szczególności rząd — nie powinien być w stanie dowiedzieć się, jak zagłosowała dana osoba.

### Weryfikacja tożsamości

Organizator wyborów powinien mieć dostęp do narzędzi weryfikacji tożsamości i uprawnień do głosowania. Powinna istnieć możliwość weryfikacji zarówno elektronicznej (np. z użyciem podpisu zaufanego lub aplikacji mObywatel), jak i fizycznej — na przykład przez pracownika urzędu gminy, powiatu, ambasady czy placówki konsularnej.

### Odbieranie uprawnień wyborczych

Jeśli dana osoba utraciła prawa wyborcze (np. z powodu prawomocnego skazania za przestępstwo, którego sankcją jest pozbawienie praw wyborczych, niepełnosprawności intelektualnej lub śmierci), system powinien odmówić wydania jej karty wyborczej.

### Autentykacja i autoryzacja

System powinien umożliwiać integrację z wieloma rodzajami poświadczeń tożsamości oraz oferować rozbudowany model uprawnień pozwalający na delegowanie ich pomiędzy instytucjami — np. możliwość przyznawania lub odbierania uprawnień do: fizycznej weryfikacji tożsamości, organizacji wyborów czy edycji treści pytań na kartach wyborczych.

### Ogłaszanie i wyszukiwanie (discovery & broadcast)

Powinien istnieć mechanizm ogłaszania i wyszukiwania dostępnych głosowań oraz węzłów sieci.

## Pełna audytowalność

### Nienaruszalność oprogramowania

Każdy obywatel weryfikujący wynik wyborów musi mieć pewność, że program, do którego ma dostęp, jest dokładnie tym samym programem, którego rząd używał w trakcie wyborów, i że nie został on zmodyfikowany po ich rozpoczęciu.

### Integralność i otwarte urny wyborcze

Każdy obywatel ma dostęp do wszystkich kart wyborczych. Raz oddany głos nie może zostać nigdy zmodyfikowany ani usunięty.

### Jawność list wyborczych

Lista osób uprawnionych do głosowania oraz ich przynależność do okręgów wyborczych musi być ogólnodostępna. Otwartą kwestią pozostaje, czy jawna powinna być również informacja o tym, kto pobrał kartę wyborczą (w odróżnieniu od tego, jak zagłosował).

### Niezależność cyfrowa

Oprogramowanie powinno opierać się na otwartych protokołach i udostępniać pełną dokumentację API. Specyfikacja musi być na tyle kompletna, by niezależne organizacje mogły tworzyć odrębne, wzajemnie kompatybilne implementacje, nie mając dostępu do swojego kodu źródłowego.

Implementacja referencyjna powinna być udostępniona na wolnej licencji typu copyleft — zapewniającej wolność użycia, modyfikacji i dystrybucji, przy jednoczesnym wymogu zachowania tych samych swobód dla kolejnych odbiorców oprogramowania.

Specyfikacja nie może w żaden sposób wymuszać użycia konkretnych zamkniętych standardów, produktów konkretnych firm ani dostępu do usług nieobjętych tymi samymi lub wyższymi rygorami otwartości co sam system e-votingu. Oprogramowanie nie może być powiązane z konkretną infrastrukturą — każdy powinien móc uruchomić je na własnej maszynie. Implementacja referencyjna musi zawierać pełną instrukcję kompilacji.

## Cele opcjonalne

- **Zapobieganie sprzedawaniu głosów.**
- **Zapobieganie wymuszaniu** oddania określonego głosu pod groźbą przemocy.
