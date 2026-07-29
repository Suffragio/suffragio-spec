---
title: Porównanie sieci anonimizujących
description: Porównanie sieci Tor, I2P, mixnetów, Hyphanet i Freenetu wobec dwóch skonfliktowanych wymagań e-votingu — anonimowego oddania głosu oraz trwałej, publicznie śledzonej listy głosów — i uzasadnienie, dlaczego Suffragio dzieli te zadania między dwa mechanizmy.
---

Suffragio potrzebuje sieci anonimizującej do dwóch zadań, które wyglądają podobnie, ale ciągną w przeciwnych kierunkach. Ta strona porównuje główne kandydatury i wyjaśnia, która z nich (lub jakie ich połączenie) pasuje do którego zadania.

## Dwa wymagania, pozorna sprzeczność

1. **Anonimowe oddanie głosu.** Głosujący musi mieć możliwość oddania głosu tak, by nikt — łącznie z operatorami infrastruktury — nie mógł powiązać żądania sieciowego z jego tożsamością.
2. **Publiczne, trwałe, niemal na żywo śledzenie.** Każdy musi móc pobrać pełną listę oddanych głosów, śledzić jej wzrost (z akceptowalnym opóźnieniem) i samodzielnie policzyć wynik. Lista musi być **append-only** — żaden głos nie może zostać zmodyfikowany ani usunięty po zapisaniu.

Te wymagania ciągną w przeciwne strony: anonimowość faworyzuje efemeryczny, trudny do skorelowania ruch sieciowy bez trwałego zapisu *kto, co i kiedy wysłał*; wiarygodny publiczny wynik faworyzuje trwały, replikowany, odporny na manipulacje rekord, który wielu ludzi może odczytać i zweryfikować. Pojedynczy mechanizm rzadko optymalizuje oba te cele jednocześnie, dlatego warto zapytać, czy Suffragio powinno użyć **jednej sieci do obu zadań, czy je rozdzielić**.

## Kandydaci

### Tor

Najlepiej przebadana sieć anonimizująca o niskim opóźnieniu, z milionami użytkowników dziennie i tysiącami wolontariackich przekaźników. Ruch jest kierowany przez stały, dwukierunkowy obwód o 3 skokach; usługa ukryta dodaje kolejne 3 skoki po stronie odbiorcy. Ogromny zbiór anonimowości i sprawdzone w boju oprogramowanie to mocne strony Tora, ale jest to **czysty transport** — nie posiada pojęcia trwałego, replikowanego, publicznie przeszukiwalnego magazynu danych. Jeśli przekaźnik obsługujący usługę ukrytą przestanie działać, usługa znika. Tor jest też podatny na ataki korelacji ruchu ze strony przeciwnika obserwującego oba końce obwodu ("globalny pasywny przeciwnik"), choć wymaga to znacznych zasobów.

### I2P

Sieć nakładkowa peer-to-peer oparta na przełączaniu pakietów, zaprojektowana specjalnie pod usługi ukryte wewnątrz sieci. Każdy uczestnik przekazuje ruch innych (model bardziej "demokratyczny" niż podział klient/przekaźnik w Torze) i wykorzystuje jednokierunkowe tunele (zwykle 6 przychodzących + 6 wychodzących), więc pojedynczy skompromitowany węzeł widzi tylko jeden kierunek przepływu. Daje to I2P nieco silniejszą nierozróżnialność połączeń niż Tor dla ruchu typu usługi ukrytej, kosztem znacznie mniejszego zbioru anonimowości (dziesiątki tysięcy routerów wobec milionów w Torze) i mniejszej niezależnej weryfikacji bezpieczeństwa. Podobnie jak Tor, I2P jest warstwą transportową — trwały, publicznie odczytywalny, append-only magazyn danych nie jest częścią jej projektu.

### Mixnety (np. Nym, Loopix)

Mixnety dodają ruch maskujący i opóźnienie mieszania na każdym skoku, specjalnie po to, by opierać się atakom korelacji czasowej, na które podatne są zarówno Tor, jak i I2P wobec globalnego pasywnego przeciwnika. Czyni to je najsilniejszą opcją anonimowości dla etapu *oddania głosu*. Koszt to opóźnienie (pakiety są celowo opóźniane i dopełniane) oraz — w przypadku sieci takich jak Nym — warstwa ekonomiczna/tokenowa potrzebna do zachęcenia operatorów węzłów. Mixnety są też nowsze i mniej sprawdzone w boju niż Tor, i — podobnie jak Tor i I2P — są czystym transportem bez wbudowanego trwałego, publicznego rejestru.

### Hyphanet (dawniej Freenet, oryginalna sieć w Javie)

Oryginalny projekt Freenet — przemianowany na **Hyphanet**, by uniknąć mylenia go z niezwiązanym z nim przepisaniem w Ruście opisanym niżej — to rozproszony magazyn danych do anonimowego publikowania i przeglądania treści ("freesites"). Zawartość jest dzielona na zaszyfrowane fragmenty rozproszone po węzłach zgodnie z routingiem opartym na kluczach typu "small-world", więc żaden pojedynczy węzeł nie posiada ani nie zna zawartości całego pliku, a żaden operator nie może zostać pociągnięty do odpowiedzialności za to, co przechowuje jego węzeł. Popularna zawartość jest zachowywana, a niepopularna automatycznie usuwana, by zrobić miejsce na nową, a opóźnienie routingu/pobierania liczone jest w minutach, nie milisekundach. Hyphanet nie ma pojęcia subskrypcji aktualizacji na żywo dla danej treści — czytelnik musi ponownie pobrać klucz, by sprawdzić, czy się zmienił — co czyni go słabym kandydatem do listy, którą trzeba obserwować rosnącą niemal na żywo. Jego aktywny rdzeń liczy się w tysiącach węzłów, znacznie mniej niż w Torze, a jego warstwa routingu przeszła znacznie mniej niezależnej kryptograficznej weryfikacji gwarancji anonimowości niż Tor.

### Freenet (niezwiązana implementacja w Ruście, na której opiera się Suffragio)

Mimo wspólnej nazwy (i częściowo wspólnego rodowodu routingu) z Hyphanet, aktywnie rozwijany **Freenet** w Ruście (`freenet-core`) to inna sieć o innym podstawowym prymitywie: rozproszony, adresowalny treścią magazyn klucz-wartość zbudowany na **kontraktach** WebAssembly, a nie nakładka do publikowania statycznych plików. Żądania są nadal kierowane przez peer-to-peerową nakładkę typu "small-world" ukrywającą pochodzenie sieciowe żądającego, ale **stan kontraktu jest replikowany i trwale przechowywany** przez węzły znajdujące się blisko jego lokalizacji na pierścieniu, jest odczytywalny przez każdego, kto potrafi do niego dotrzeć, a węzły mogą **subskrybować** aktualizacje niemal w czasie rzeczywistym, gdy stan się zmienia. To dokładnie kształt listy głosów typu append-only, publicznie audytowalnej, aktualizowanej na żywo: predykat poprawności kontraktu może odrzucić każdą aktualizację, która nie jest prawidłowo podpisanym nowym głosem, dzięki czemu węzły nie mogą modyfikować ani usuwać istniejących wpisów, a jedynie dopisywać nowe.

Kompromis jest podobny jak w przypadku Hyphanet: sieć jest młoda, jej aktywny rdzeń najprawdopodobniej wciąż liczy się w tysiącach węzłów, a nie milionach jak w Torze, a jej warstwa routingu przeszła znacznie mniej niezależnej kryptograficznej weryfikacji gwarancji anonimowości niż Tor.

## Porównanie

| Właściwość | Tor | I2P | Mixnet (Nym/Loopix) | Hyphanet | Freenet (Rust) |
| --- | --- | --- | --- | --- | --- |
| Główna rola | Anonimowy transport | Anonimowy transport (usługi wewnętrzne) | Anonimowy transport | Anonimowe statyczne publikowanie/magazyn | Rozproszony magazyn + transport |
| Zbiór anonimowości | Miliony użytkowników, 7000+ przekaźników | ~55 000 routerów | Mały, rosnący | Tysiące węzłów | Tysiące węzłów (młoda sieć) |
| Odporność na korelację czasową/ruchu (globalny przeciwnik) | Słaba | Słaba do umiarkowanej | Silna (z założenia) | Słaba do umiarkowanej | Słaba do umiarkowanej |
| Opóźnienie | Niskie (~200–500 ms) | Niskie do umiarkowanego (1–3 s) | Wyższe (celowe opóźnienie mieszania) | Wysokie (minuty) | Wyższe niż Tor/I2P; brak publicznych benchmarków |
| Trwały, publicznie odczytywalny magazyn | Nie | Nie | Nie | Tak (fragmenty statycznej treści) | Tak (stan kontraktu) |
| Subskrypcja na żywo / aktualizacje push | Nie (tylko odpytywanie) | Nie | Nie | Nie (tylko odpytywanie) | Tak (natywne `SUBSCRIBE`) |
| Gwarancja append-only | Nie dotyczy | Nie dotyczy | Nie dotyczy | Nie (treść jest usuwana wg popularności) | Tak, wymuszana przez predykat poprawności kontraktu |
| Dojrzałość / niezależna weryfikacja | Bardzo wysoka | Umiarkowana | Niska do umiarkowanej | Umiarkowana | Niska (młoda sieć) |

Żaden wiersz nie jest najlepszy wszędzie: Tor wygrywa rozmiarem zbioru anonimowości i dojrzałością jako czysty transport; mixnety wygrywają odpornością na korelację ruchu; Freenet (Rust) jest jedynym kandydatem, który natywnie zapewnia trwały, append-only, subskrybowalny publiczny rekord — czego statyczny model publikowania Hyphanet nie oferuje.

## Dlaczego nie wybrać po prostu "najlepszej" sieci anonimizującej do wszystkiego?

Tor, I2P i mixnety świetnie ukrywają *kto wysłał wiadomość*, ale żadna z nich niczego trwale nie przechowuje. W żadnej z tych sieci nie ma pojęcia "kanonicznej, stale rosnącej, publicznie pobieralnej listy wszystkich dotychczas oddanych głosów" — to trzeba zbudować dodatkowo, zwykle uruchamiając serwer, który loguje to, co widział, co ponownie wprowadza pojedynczy punkt zaufania i awarii. To niweczy drugie wymaganie: wiarygodny, odporny na manipulacje, odporny na cenzurę publiczny wynik.

Hyphanet przechowuje treść trwale i anonimowo, ale został zaprojektowany pod statyczne, rankingowane popularnością publikowanie, a nie żywy, stale rosnący rekord typu append-only — nie ma mechanizmu subskrypcji, więc śledzenie go oznacza wielokrotne ponowne pobieranie klucza i porównywanie wyniku, a nic nie chroni przed zniknięciem starych, usuniętych fragmentów pod presją miejsca na dysku.

Freenet (Rust) elegancko rozwiązuje połowę zadania dotyczącą *śledzenia*, ponieważ kontrakty są dokładnie tym: zdecentralizowanym, replikowanym, append-only, aktualizowanym na żywo publicznym rejestrem bez pojedynczego operatora. Ale jego gwarancje anonimowości na warstwie routingu są słabiej przebadane, a zbiór anonimowości najprawdopodobniej mniejszy niż w Torze, co ma większe znaczenie dla etapu *oddawania głosu*, gdzie jednorazowe żądanie głosującego o podpis ślepy lub złożenie głosu jest najbardziej wrażliwym zdarzeniem sieciowym w całym protokole.

## Rekomendacja: dwa mechanizmy, nie jeden

Biorąc pod uwagę powyższe porównanie, Suffragio **nie powinno** zakładać, że jedna sieć anonimizująca jest właściwym narzędziem do obu zadań:

- **Warstwa oddawania głosu — postaw na najsilniejszy dostępny anonimowy transport.** Do żądania podpisu ślepego i złożenia wypełnionego głosu należy priorytetyzować odporność na ataki korelacji ruchu oraz duży zbiór anonimowości. Zarówno Tor (z ogromną, dobrze zaudytowaną siecią przekaźników), jak i mixnet (dla silniejszej odporności na globalnego pasywnego przeciwnika, jeśli dodatkowe opóźnienie jest akceptowalne w oknie głosowania) są uzasadnionym wyborem i są lepiej zbadane pod tym kątem niż sama warstwa routingu Freenetu (Rust). Hyphanet też się tu nie sprawdza — jest zaprojektowany pod magazynowanie, nie pod interaktywną komunikację żądanie/odpowiedź o niskim opóźnieniu.
- **Warstwa śledzenia — użyj modelu kontraktów Freenetu (Rust).** Publiczna, append-only lista głosów oraz wyniki końcowe powinny znajdować się w kontrakcie Freenetu: jego trwałość, adresowanie treścią i natywny mechanizm subskrypcji dają każdemu obywatelowi sposób na obserwowanie na żywo rosnącego wyniku i weryfikację, że żaden wpis nie został nigdy zmieniony ani usunięty, bez polegania na pojedynczym serwerze. Hyphanet nie oferuje połowy tej funkcjonalności związanej z subskrypcją na żywo, a jego mechanizm usuwania wg popularności jest wprost sprzeczny z gwarancją append-only.

Konkretnie oznacza to kierowanie wywołań `RequestBlindSignature` i `SubmitVote` przez Tora (lub mixnet) jako zewnętrzny transport, podczas gdy **celem** tych wywołań — Blind Signature Authority i Vote Broadcast Queue — nadal jest stan kontraktu Freenetu (Rust) zapewniający trwały, publicznie audytowalny magazyn danych. Obie sieci nie wykluczają się wzajemnie: własny routing żądań Freenetu może również przechodzić przez Tora dla dodatkowej warstwy ukrywania pochodzenia na ścieżce oddawania głosu, kosztem dodatkowego opóźnienia, które jest akceptowalne dla czynności wykonywanej raz na wybory.

Taki podział mapuje się bezpośrednio na dwa konkurujące ze sobą wymagania: **budżet anonimowości trafia tam, gdzie zachodzi najbardziej wrażliwe zdarzenie (oddanie głosu)**, a **budżet trwałości/aktualności trafia tam, gdzie publiczny rekord musi być wiarygodny i niemożliwy do po cichu zmodyfikowania (śledzenie)**.

## Zobacz też

- [Anonimowość głosu](/suffragio-spec/pl/vote-anonymity/) — jak podpisy ślepe i anonimowość sieciowa łączą się, by chronić tajność głosu.
- [Dlaczego nie blockchain?](/suffragio-spec/pl/why-not-blockchain/) — dlaczego publiczny rejestr blockchain nie rozwiązuje tego samego konfliktu między uprawnieniem a anonimowością.
- [Architektura systemu](/suffragio-spec/pl/architecture/) — pełna architektura systemu, w tym obecny projekt transportu opartego wyłącznie na Freenecie (Rust), który ocenia ta strona.
