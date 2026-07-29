---
title: Protokół v1 (normatywny)
description: Normatywne decyzje protokołu do implementacji backendu zgodnego z Suffragio — kryptografia, karty, auth, maszyna stanów, transport i audyt.
---

Ta strona jest **normatywnym** uzupełnieniem [Architektury systemu](/suffragio-spec/pl/architecture/) oraz [Specyfikacji API gRPC](/suffragio-spec/pl/api-reference/). W razie sprzeczności ze starszym tekstem **dla v1 wygrywa ta strona**.

Implementatorzy (ludzie i agenci AI) MUSZĄ traktować poniższe reguły jako wymagania, nie luźne sugestie.

## Podsumowanie decyzji

| Obszar | Decyzja v1 |
| --- | --- |
| Suite ślepego podpisu | Wersjonowany; domyślnie `BLIND_SIG_RSA_FDH_3072_SHA256` |
| Co jest podpisywane | **Cała** wypełniona karta (kanoniczny CBOR) |
| Klucze BSA | Lista w konfiguracji wyborów (`key_id`, materiał, ważność); submit niesie `key_id` |
| Eligibility token | Losowy ID + stan po stronie Registration & Eligibility (RegSvc) |
| Paragon głosu | **Brak** `receipt_hash`; wyborca trzyma lokalną kopię karty i szuka jej w publicznym logu |
| Szablon karty | Mały własny DSL (JSON), nie surowy JSON Schema |
| Kodowanie wypełnionej karty | Deterministyczny CBOR |
| Identyfikatory wyborów | Stabilne pola `id` w DSL; tally tylko po `id` |
| Ordynacja | **Skrypt Lua** (bez wbudowanych formuł); inline lub ref z katalogu; zawsze `content_hash` |
| Odkrywanie formuł | Osobny serwis gRPC `FormulaCatalog` |
| Nieważne karty | Vote Queue **odrzuca** przy submit (nawet przy poprawnym sig BSA) |
| Auth adminów | Pluggable port AuthZ; domyślnie OIDC JWT → stringi akcji; nie zastępuje IdP |
| Akcje AuthZ | Dowolne stringi (konwencje w docs) |
| Cykl wyborów | Normatywna maszyna stanów |
| Revoke praw | Tylko roll / nowe tokeny; nie wyciąga głosów z logu; utrata w trakcie → skutek od **kolejnych** wyborów |
| Log głosów | Multi-writer, append-only **hash chain**, eventual consistency |
| Czas wyników | Wolno czekać godzinami na sync; oficjalne dopiero po podpisach m-of-n PKW/komisji |
| Transport | Freenet **obowiązkowy** dla `RequestBlindSignature` i `SubmitVote` |
| Dowód tożsamości wyborcy | Asercja od klienta **albo** sesja po stronie serwera (per adapter) |
| Publikacja wyników | Wyniki + head logu + `content_hash` Lua + podpisy m-of-n |
| Oficjalny czas | Podpisane `CloseVotingWindow` (organizator/PKW), nie NTP peerów |
| `voter_id` | Stabilny w ramach jednych wyborów; nielinkowalny między wyborami z zewnątrz |
| Retry RPC | `idempotency-key` (UUID) w metadata gRPC |
| Jednorazowość tokenu | Atomowy `ConsumeEligibilityToken` tylko w RegSvc (bez księgi zużyć w BSA) |
| Eventy | `WatchEvents` z kursorem **oraz** snapshoty |
| Publiczny `received_at` | Flaga per wybory; domyślnie **off** przy wyborach publicznych |

## Kryptografia

### Rejestr algorytmów

Mechanizmy są **wersjonowane**. Każde wybory pinują akceptowane suite.

| Identyfikator | Rola | Status v1 |
| --- | --- | --- |
| `BLIND_SIG_RSA_FDH_3072_SHA256` | Ślepy podpis (RSA-FDH, 3072 bit, SHA-256) | **Wymagany domyślny** |
| Kolejne ID | Dodatkowe suite | Opcjonalne; bez zmiany semantyki starych ID |

### Co podpisuje BSA

1. Klient buduje **kompletną** wypełnioną kartę jako deterministyczny CBOR.  
2. Zaślepia suite-specific encoding tych bajtów.  
3. BSA podpisuje wartość **zaślepioną**; po odślepieniu `Verify(klucz_BSA, ballot_cbor, signature)` MUSI przejść.  
4. `SubmitVote` niesie te same bajty `ballot`, `signature` oraz `key_id`.

BSA NIE WOLNO poznać odślepionej karty. Podpis wiąże **całą** treść karty.

Separacja dziedziny MUSI obejmować co najmniej `election_id` i suite ID.

### Lista kluczy BSA

W `ElectionConfig`: powtarzalne `BsaPublicKey` (`key_id`, `suite_id`, `public_key`, `not_before`, `not_after`). Po `PUBLISHED` — immutability z wyjątkiem dopinania nowego `key_id` (rotacja).

### EligibilityToken

- Losowy opaque ID; semantyka w stanie RegSvc.  
- Zużycie wyłącznie przez atomowy `ConsumeEligibilityToken`.  
- Odpowiedź do BSA **bez** `voter_id`.

### Brak wiążącego paragonu

`SubmitVote` NIE ZWRACA kryptograficznego receiptu dla osób trzecich. Wyborca trzyma lokalnie kartę+sig i po publikacji oficjalnego logu sprawdza obecność identycznego wpisu.

## Karty

### DSL szablonu

Mały DSL JSON (`suffragio-ballot-dsl/1`) z obowiązkowymi stabilnymi `id` pytań i opcji. Etykiety tylko do UI.

### Wypełniona karta

Deterministyczny CBOR; te bajty są podpisywane i trafiają do logu.

### Walidacja w Queue

Po weryfikacji podpisu Queue WALIDUJE kartę względem DSL; błąd → reject (bez append). Poprawa treści wymaga nowego tokena i nowego ślepego podpisu — klient POWINIEN walidować przed BSA.

## Ordynacje (Lua)

Brak wbudowanych FPTP/D’Hondt/STV w core. Skrypt Lua: inline lub `FormulaCatalog`; zawsze `content_hash`. Frontend może oferować gotowce (m.in. PL). Piaskownica Lua bez sieci/FS, deterministyczna, z limitami.

Serwis **`FormulaCatalog`**: publish/get/list.

## Autoryzacja (organizatorzy)

Port `Authorize(proof, action, resource)`. Domyślnie OIDC JWT. Akcje = dowolne stringi. Wyborcy = osobny port IdentityProvider (mObywatel itd.).

Metadata: `idempotency-key` na mutacjach.

## Maszyna stanów

`DRAFT → READY → PUBLISHED → VOTING → CLOSED → TALLIED → RESULTS_PUBLISHED`  
Niedozwolone przejścia = błąd. Od `PUBLISHED` zamrożenie DSL, hash skryptu, okręgów, okna; klucze BSA append-only.

## Rejestracja, revoke, voter_id

`VerifyIdentity`: asercja **lub** `auth_session_ref`.  
`voter_id`: opaque, stabilny w elekcji, inny między elekcjami.  
Revoke: roll / blokada nowych tokenów; bez kasowania głosów z logu.  
`ConsumeEligibilityToken`: publiczne RPC, tylko rola BSA, atomowe, bez `voter_id` w odpowiedzi; jedyne źródło single-use.

## Log głosów

Multi-writer, hash chain, eventual consistency. Oficjalny zbiór = commitment w pakiecie podpisanym m-of-n. Sygnały sync pomocnicze.  
`received_at` publiczny: flaga konfiguracji (domyślnie off w wyborach publicznych).

## Wyniki

Close = podpisane przez komisję. Publish = wyniki + head logu + hash Lua + podpisy m-of-n.

## Transport

| Ścieżka | Reguła v1 |
| --- | --- |
| `RequestBlindSignature`, `SubmitVote` | **Freenet** (wybory publiczne) |
| Reszta (admin, VerifyIdentity, tally, audyt stream) | MAY zwykły gRPC |

## Eventy

`WatchEvents` z kursorem **oraz** snapshot stanu — wybór konsumenta.

## Ścieżka implementacyjna

```text
Voter --VerifyIdentity--> RegSvc
Voter --RequestBlindSignature--> BSA (Freenet)
BSA  --ConsumeEligibilityToken--> RegSvc
Voter --SubmitVote--> Queue (Freenet)
Komisja: sync → Close → Compute (Lua) → Publish (m-of-n)
Audytor: weryfikacja pakietu + przeliczenie
```

Pełne brzmienie normatywne w języku angielskim (źródło dla implementacji obok tej strony): [Protocol v1](/suffragio-spec/protocol-v1/).
