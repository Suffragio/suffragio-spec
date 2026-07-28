---
title: Dlaczego nie blockchain?
description: Dlaczego Suffragio nie jest zbudowane na blockchainie — fundamentalny konflikt między uprawnieniem do głosowania a anonimowością, rozważane sposoby jego obejścia i dlaczego zawodzą.
---

Na pierwszy rzut oka technologia blockchain wydaje się idealnym kandydatem do implementacji e-votingu. Ta strona wyjaśnia, dlaczego Suffragio jej mimo to nie używa.

## Zaleta

Blockchain gwarantuje nienaruszalność kontraktów i pełną audytowalność przebiegu procesu wyborczego. Mniej oczywistą, ale równie ważną zaletą jest dostępność rozproszonej infrastruktury, której działania nie można łatwo zakłócić i której instytucje rządowe nie muszą budować na własny koszt.

## Fundamentalny konflikt

Niestety żadna z dostępnych implementacji blockchain nie jest w stanie rozwiązać podstawowego konfliktu leżącego u podstaw ogólnokrajowego systemu wyborczego: musi on jednocześnie gwarantować, że

- **tylko osoby uprawnione do głosowania otrzymają kartę wyborczą**, oraz
- **nikt nie jest w stanie sprawdzić, kto i jak oddał głos** (anonimowość).

Publiczny, transparentny rejestr jest bardzo dobry w zapewnianiu pierwszej właściwości i bardzo słaby w zapewnianiu drugiej — każda transakcja jest z założenia możliwa do powiązania z jej nadawcą.

## Próby rozwiązania problemu

### Wirówki (np. Tornado Cash)

Wirówki zapewniają bardzo wysoki poziom anonimowości. Alternatywna implementacja tego wzorca w postaci smart contractu może zapewnić pełną anonimowość, ale generuje nowy problem: po skorzystaniu z wirówki nie jesteśmy w stanie zagwarantować, że instytucje rządowe nie wygenerują własnych głosów i nie "dosypią ich do urn wyborczych" razem z prawdziwymi.

### Oracles

Użycie wyroczni (oracles) w ramach smart contractu pozwala zapewnić pełną anonimowość *oraz* potwierdzić, że tylko jedna osoba odebrała daną kartę wyborczą. Niestety w przypadku e-votingu środek ciężkości przesuwa się tak mocno w stronę wyroczni, że tracimy kluczowe atuty blockchaina, zachowując jednocześnie jego wady. Wyrocznia musiałaby być kontrolowana przez instytucje rządowe w takim samym stopniu jak w tradycyjnym, scentralizowanym systemie wyborczym — co odbiera sens sięganiu po blockchain w pierwszej kolejności.

## Dodatkowe czynniki

Nie są one same w sobie decydujące, ale warto je również rozważyć:

- **Ekologia.** Wykonywanie smart contractów jest po prostu zachłanne energetycznie. Analogiczne rozwiązania można często zaimplementować o wiele taniej i wygenerować przy tym o wiele mniejszy ślad węglowy.
- **Trudność w oszacowaniu kosztów infrastruktury.** Teoretycznie można napisać smart contracty, które nie wymagają opłat gas przy każdej transakcji. W praktyce jednak, jeśli chcemy mieć pewność, że różne operacje zostaną wykonane w czasie akceptowalnym dla przeciętnego użytkownika, koszt organizacji ogólnokrajowych wyborów może okazać się nieproporcjonalnie wysoki — nie zawsze, ale sama nieprzewidywalność tego kosztu jest już problemem.
- **Wysokie wymagania systemowe dla audytorów.** Ktoś, kto chce zaudytować pojedyncze wybory, jest zmuszony uruchomić pełny węzeł (np. Ethereum) i pobrać długą historię transakcji w większości niezwiązanych z interesującymi go wyborami. Projektując system do e-votingu, chcieliśmy zapewnić możliwość weryfikacji przeciętnemu obywatelowi dysponującemu zwykłym domowym laptopem — tak, by każdy mógł przejść przez cały proces weryfikacji w domowym zaciszu, bez kupowania absurdalnej ilości pamięci dyskowej czy operacyjnej i bez skomplikowanej konfiguracji infrastruktury.
