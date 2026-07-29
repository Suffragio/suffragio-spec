---
title: Anonimowość głosu
description: W jaki sposób Suffragio kryptograficznie i architektonicznie chroni tajność pojedynczych głosów, zachowując jednocześnie pełną audytowalność wyborów.
---

Suffragio musi jednocześnie spełniać dwa wyglądające na sprzeczne wymagania: każdy oddany głos musi pochodzić od uprawnionego wyborcy, a nikt — ani rząd, ani organizator wyborów, ani operator infrastruktury — nie może dowiedzieć się, jak zagłosowała konkretna osoba. Ta strona wyjaśnia mechanizmy, które realizują drugie z tych założeń.

## Istota problemu

Naiwny system e-votingu przechowywałby gdzieś zapis w stylu *„Anna zagłosowała na opcję B”*. Pozwoliłoby to urzędnikom, hakerom lub przyszłym rządom odtworzyć wybory poszczególnych osób. Aby temu zapobiec, Suffragio rozdziela **uprawnienie do głosowania** od **samego głosu** na każdej warstwie systemu.

Celem jest uzyskanie trzech rodzajów niepowiązywalności:

1. **Niepowiązywalność tożsamości z kartą.** Organy potwierdzające prawo do głosu nigdy nie widzą treści karty wyborczej.
2. **Niepowiązywalność wydania karty z jej oddaniem.** Moment i fakt uzyskania podpisanego blankietu nie mogą być skorelowane z momentem i faktem przesłania wypełnionego głosu.
3. **Niepowiązywalność pochodzenia sieciowego z głosem.** Sieciowa ścieżka użyta do przesłania głosu nie może zdradzać tożsamości wyborcy.

## Podpisy ślepe — uprawnienie bez ujawniania treści

Pierwszy rozdział zapewniają **podpisy ślepe** (*blind signatures*), schemat kryptograficzny wprowadzony przez Davida Chauma.

W zwykłym podpisie cyfrowym podpisujący czyta wiadomość, podpisuje ją, a sam podpis potwierdza zarówno autora, jak i treść. W podpisie ślepym podpisujący weryfikuje jedynie *zamazany* token i wystawia podpis na tej zamazanej wartości. Odbiorca może później usunąć czynnik zamazujący i uzyskać podpis na prawdziwej wiadomości — podpisujący natomiast nigdy jej nie widzi.

W Suffragio wygląda to następująco:

1. Wyborca uwierzytelnia się w **Registration & Eligibility Service** i otrzymuje jednorazowy `EligibilityToken`. Ten krok jest powiązany z tożsamością: usługa zapisuje, że token został wydany konkretnemu uprawnionemu wyborcy w konkretnym okręgu.
2. Lokalny klient wyborcy generuje nowy identyfikator karty wyborczej i zamazuje go losową wartością znaną tylko klientowi.
3. Klient wysyła zamazany identyfikator razem z `EligibilityToken` do **Blind Signature Authority** (BSA).
4. BSA sprawdza, czy token jest ważny i nieużywany, zużywa go i podpisuje zamazany identyfikator. Ponieważ identyfikator jest zamazany, BSA nie widzi treści karty wyborczej ani nie jest w stanie później rozpoznać, która karta odpowiada temu podpisowi.
5. Klient lokalnie usuwa czynnik zamazujący. Efektem jest ważny podpis BSA na prawdziwym identyfikatorze karty, przy czym samo BSA nie posiada zapisu tego prawdziwego identyfikatora.

Wyborca dysponuje teraz poświadczeniem, które dowodzi *„BSA poświadczył jeden głos jednego uprawnionego wyborcy”*, nie ujawniając przy tym ani wyborcy, ani karty.

## Rozdzielenie procesowe i czasowe

Nawet przy podpisach ślepych obserwator widzący jednocześnie żądanie podpisu i oddany głos mógłby próbować je skorelować po czasie. Suffragio utrudnia to poprzez projekt:

- Wyborca może poprosić o podpis ślepy w dowolnym momencie okna wyborczego, niekoniecznie tuż przed oddaniem głosu.
- Żądanie może być trasowane przez anonimizującą sieć (patrz niżej), więc BSA widzi jedynie pochodzenie z Freenet, a nie adres IP ani sesję dostawcy tożsamości.
- `EligibilityToken` jest zużywany w momencie podpisywania, więc ta sama tożsamość nie może poprosić o drugi podpis. Dzięki temu na każdego uprawnionego wyborcę przypada dokładnie jedna podpisana karta, ale BSA nie wie, którą.

## Anonimowy transport — Freenet

Podpisy ślepe ukrywają *co* jest podpisywane, ale same z siebie nie ukrywają *kto* kontaktuje się z infrastrukturą wyborczą. Suffragio działa na **[Freenet](https://freenet.org)** — sieci P2P, w której każde żądanie jest trasowane przez innych uczestników:

- Klient wyborcy nawiązuje bezpośrednie połączenie TCP ani z BSA, ani z Vote Broadcast Queue, ani z żadną inną usługą.
- Każde żądanie przechodzi przez ścieżkę małego świata węzłów Freenet, co sprawia, że usługi nie widzą jego sieciowego pochodzenia.
- Żądania są obsługiwane przez replikowany stan kontraktów; nie ma pojedynczego serwera, którego logi mogłyby zdeanonimizować wyborców.

Oznacza to, że nawet jeśli atakujący kontroluje część węzłów, skorelowanie żądania sieciowego z konkretnym wyborcą wymaga przejęcia znaczącej części sieci.

## Publiczna urna jest anonimowa

Po wypełnieniu karty wyborca przesyła gotowy głos do **Vote Broadcast Queue**. Kolejka jest publiczna i przyrostowa: każdy może pobrać wszystkie oddane głosy i samodzielnie zweryfikować podpisy oraz końcowy wynik. Zawiera ona jednak tylko:

- podpisany identyfikator karty,
- dokonane przez wyborcę wybory,
- kryptograficzny dowód, że kartę podpisało BSA.

Nie ma tam identyfikatora wyborcy, ciasteczka sesji, adresu IP ani znacznika czasu powiązanego z rzeczywistą tożsamością. Podpis dowodzi uprawnienia do głosu; treść karty pozostaje anonimowa.

## Co widzi, a czego nie widzi każdy z aktorów

| Aktor | Widzi | Nie widzi |
| --- | --- | --- |
| Registration & Eligibility Service | Kto jest uprawniony, komu wydano token | Treści kart ani ich identyfikatorów |
| Blind Signature Authority | Że zużyto ważny token i podpisano zamazaną wartość | Prawdziwego identyfikatora karty, dokonanych wyborów ani tożsamości wyborcy |
| Vote Broadcast Queue / audytorzy | Wszystkie oddane głosy i ich podpisy | Kto oddał który głos |
| Operatorzy sieci / węzły Freenet | Szyfrowany ruch trasowany przez sieć | Który głos należy do którego wyborcy |
| Organizator wyborów | Wyniki zbiorcze | Poszczególnych głosów powiązanych z tożsamościami |

## Odporność na przymus (opcjonalne wzmocnienie)

W wymaganiach *zapobieganie sprzedawaniu głosów i przymusowi* jest wymienione jako cel opcjonalny. Podstawowy projekt już częściowo temu przeciwdziała: ponieważ żaden pokwitowanie nie łączy wyborcy z kartą, wyborca nie może udowodnić prześladowcy, jak zagłosował. W przyszłości można dodać dedykowane mechanizmy odporności na przymus bez zmiany rdzeniowego schematu podpisów ślepych.

## Podsumowanie

Suffragio chroni tajność głosu dzięki połączeniu rozwiązań kryptograficznych i architektonicznych:

- **Podpisy ślepe** rozdzielają organ weryfikujący uprawnienia od organu podpisującego kartę.
- **Jednorazowe tokeny uprawnień** zapewniają jedną kartę na wyborcę, nie ujawniając przy tym treści głosu.
- **Sieć peer-to-peer Freenet** ukrywa sieciowe pochodzenie wyborcy przed usługami systemu.
- **Anonimowa, przyrostowa urna** sprawia, że każdy głos jest weryfikowalny, ale nieśledzialny.

Dzięki tym właściwościom system może udowodnić, że wynik wyborów został obliczony z dokładnie tego zbioru ważnych głosów uprawnionych wyborców — nie ujawniając nigdy, kto na kogo głosował.
