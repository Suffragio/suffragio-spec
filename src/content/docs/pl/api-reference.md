---
title: Specyfikacja API gRPC
description: Pełna specyfikacja protokołu gRPC każdego serwisu Suffragio — komendy, zapytania i strumienie zdarzeń, na podstawie kanonicznych definicji .proto.
---

Ta strona dokumentuje protokół gRPC każdego serwisu opisanego w [Architekturze systemu](/suffragio-spec/pl/architecture/). Odzwierciedla kanoniczne, gotowe do implementacji pliki `.proto` w [`proto/suffragio/v1/`](https://github.com/Suffragio/suffragio-spec/tree/main/proto/suffragio/v1) — pliki `.proto` są źródłem prawdy dla numerów pól i kształtów komunikatów, a ta strona jest czytelną referencją. **Normatywne zachowanie** (kryptografia, maszyna stanów, Freenet, ordynacje Lua, auth) jest w [Protokole v1](/suffragio-spec/pl/protocol-v1/). Zobacz [`proto/README.md`](https://github.com/Suffragio/suffragio-spec/blob/main/proto/README.md) w sprawie lintu i generowania kodu przez [buf](https://buf.build).

Każdy serwis ma ten sam kształt:

- **Komendy** — unarne RPC mutujące stan.
- **Zapytania** — unarne lub server-streaming RPC tylko do odczytu.
- **Snapshoty** — unarne RPC zwracające stan do odbudowy plus `EventCursor` do dogrania.
- **Zdarzenia** — jedno `WatchEvents` (server-streaming) z komunikatem w `oneof` (każde zdarzenie ma `cursor`), żeby węzeł lub audytor mógł subskrybować i samodzielnie odtworzyć stan.

### Konwencje przekrojowe

| Temat | Reguła |
| --- | --- |
| Pakiet | Wszystkie komunikaty w `suffragio.v1`. Typy wspólne w `common.proto`. |
| Idempotencja | Mutacje MUSZĄ przyjmować metadata gRPC `idempotency-key: <uuid>`. Ponowienie zwraca oryginalny wynik. |
| AuthZ | RPC organizatora/urzędnika przez pluggable port AuthZ (domyślnie OIDC JWT → stringi akcji). Zob. [Protokół v1](/suffragio-spec/pl/protocol-v1/). |
| Transport | W wyborach publicznych `RequestBlindSignature` i `SubmitVote` **MUSZĄ** iść Freenetem. Pozostałe RPC MAY używać zwykłego gRPC. |
| Wznowienie Watch | `WatchEventsRequest` = opcjonalne `election_id` + `after_cursor`. Puste `election_id` = wszystkie wybory (gdzie ma sens). |

## Typy wspólne (`common.proto`)

| Typ | Pola | Uwagi |
| --- | --- | --- |
| `ElectionId` | `value: string` | Globalnie unikalne id wyborów |
| `ConstituencyId` | `value: string` | Okręg w ramach wyborów |
| `EligibilityToken` | `value: string` | Losowy opaque token; semantyka w stanie RegSvc |
| `CryptoSuiteId` | `value: string` | np. `BLIND_SIG_RSA_FDH_3072_SHA256` |
| `Constituency` | `id`, `name` | |
| `VotingWindow` | `starts_at`, `ends_at` | znaczniki czasu |
| `BsaPublicKey` | `key_id`, `suite_id`, `public_key`, `not_before`, `not_after` | Publiczne klucze weryfikacji BSA |
| `BallotTemplate` | `constituency_id`, `dsl_version`, `document_json` | DSL karty Suffragio (JSON), nie surowy JSON Schema |
| `FormulaScriptRef` | `content_hash`, opcjonalnie `inline_script`, opcjonalnie `catalog_script_id` | Ordynacja Lua; hash jest autorytatywny |
| `ElectionState` | enum | `DRAFT`, `READY`, `PUBLISHED`, `VOTING`, `CLOSED`, `TALLIED`, `RESULTS_PUBLISHED` |
| `EventCursor` | `value: string` | Token wznowienia strumienia zdarzeń |
| `WatchEventsRequest` | `election_id`, `after_cursor` | Wspólny filtr Watch |

## ElectionRegistry

Źródło prawdy konfiguracji wyborów. `proto/suffragio/v1/election_registry.proto`.

```proto
service ElectionRegistry {
  rpc CreateElection(CreateElectionRequest) returns (CreateElectionResponse);
  rpc DefineBallotTemplate(DefineBallotTemplateRequest) returns (DefineBallotTemplateResponse);
  rpc SetFormulaScript(SetFormulaScriptRequest) returns (SetFormulaScriptResponse);
  rpc AddBsaPublicKey(AddBsaPublicKeyRequest) returns (AddBsaPublicKeyResponse);
  rpc ScheduleElection(ScheduleElectionRequest) returns (ScheduleElectionResponse);
  rpc SetPublicTimestamps(SetPublicTimestampsRequest) returns (SetPublicTimestampsResponse);
  rpc TransitionElectionState(TransitionElectionStateRequest) returns (TransitionElectionStateResponse);
  rpc PublishElection(PublishElectionRequest) returns (PublishElectionResponse);
  rpc GetElection(GetElectionRequest) returns (GetElectionResponse);
  rpc ListElections(ListElectionsRequest) returns (ListElectionsResponse);
  rpc GetElectionSnapshot(GetElectionSnapshotRequest) returns (GetElectionSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream ElectionRegistryEvent);
}
```

### Komendy

| RPC | Request | Response |
| --- | --- | --- |
| `CreateElection` | `title: string`, `constituencies: Constituency[]` | `election_id: ElectionId` |
| `DefineBallotTemplate` | `election_id`, `template: BallotTemplate` | *(puste)* |
| `SetFormulaScript` | `election_id`, `script: FormulaScriptRef` | *(puste)* |
| `AddBsaPublicKey` | `election_id`, `key: BsaPublicKey` | *(puste)* |
| `ScheduleElection` | `election_id`, `voting_window: VotingWindow` | *(puste)* |
| `SetPublicTimestamps` | `election_id`, `publish_received_at: bool` | *(puste)* — gdy `true`, `StreamVotes` może zawierać `received_at`. Domyślnie dla publicznych wyborów politycznych **MUSI** być `false` ([Protokół v1](/suffragio-spec/pl/protocol-v1/)). |
| `TransitionElectionState` | `election_id`, `to_state: ElectionState` | `state: ElectionState` |
| `PublishElection` | `election_id` | *(puste)* |

### Zapytania

| RPC | Request | Response |
| --- | --- | --- |
| `GetElection` | `election_id` | `election: ElectionConfig` |
| `ListElections` | `page_size: int32`, `page_token: string` | `elections: ElectionConfig[]`, `next_page_token: string` |
| `GetElectionSnapshot` | `election_id` | `election: ElectionConfig`, `cursor: EventCursor`, `captured_at` |

`ElectionConfig` = `election_id`, `title`, `constituencies[]`, `ballot_templates[]`, `formula_script`, `bsa_public_keys[]`, `voting_window`, `state`, `published: bool`, `publish_received_at: bool`.

### Zdarzenia (`WatchEvents`)

Puste `election_id` = wszystkie wybory; wznawianie przez `after_cursor`.

| Zdarzenie | Pola | Kiedy emitowane |
| --- | --- | --- |
| `ElectionCreated` | `election_id`, `title`, `occurred_at` | sukces `CreateElection` |
| `BallotTemplateDefined` | `election_id`, `template`, `occurred_at` | sukces `DefineBallotTemplate` |
| `FormulaScriptSet` | `election_id`, `content_hash`, `catalog_script_id`, `occurred_at` | sukces `SetFormulaScript` |
| `BsaPublicKeyAdded` | `election_id`, `key`, `occurred_at` | sukces `AddBsaPublicKey` |
| `ElectionScheduled` | `election_id`, `voting_window`, `occurred_at` | sukces `ScheduleElection` |
| `ElectionStateTransitioned` | `election_id`, `from_state`, `to_state`, `occurred_at` | sukces `TransitionElectionState` |
| `ElectionPublished` | `election_id`, `occurred_at` | sukces `PublishElection` |

Każde `ElectionRegistryEvent` ma też `cursor: EventCursor`.

## RegistrationEligibility

Weryfikuje tożsamość/okręg, utrzymuje listy, wydaje i atomowo zużywa jednorazowe `EligibilityToken`. Nie rozmawia bezpośrednio z zewnętrznym systemem tożsamości — zob. [integracja tożsamości](/suffragio-spec/pl/architecture/#external-identity-integration-anti-corruption-layer). `proto/suffragio/v1/registration_eligibility.proto`.

```proto
service RegistrationEligibility {
  rpc RegisterVoterRoll(RegisterVoterRollRequest) returns (RegisterVoterRollResponse);
  rpc VerifyIdentity(VerifyIdentityRequest) returns (VerifyIdentityResponse);
  rpc RevokeVotingRights(RevokeVotingRightsRequest) returns (RevokeVotingRightsResponse);
  rpc ConsumeEligibilityToken(ConsumeEligibilityTokenRequest) returns (ConsumeEligibilityTokenResponse);
  rpc GetVoterStatus(GetVoterStatusRequest) returns (GetVoterStatusResponse);
  rpc GetRegistrationSnapshot(GetRegistrationSnapshotRequest) returns (GetRegistrationSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream RegistrationEligibilityEvent);
}
```

### Komendy

| RPC | Request | Response |
| --- | --- | --- |
| `RegisterVoterRoll` | `election_id`, `voters: VoterEntry[]` (`voter_id`, `constituency_id`) | `registered_count: int32` |
| `VerifyIdentity` | `election_id`, opcjonalnie `proof: IdentityProof` (`adapter`, `assertion`), opcjonalnie `auth_session_ref: string` | `token: EligibilityToken`, `constituency_id`, `expires_at` |
| `RevokeVotingRights` | `election_id`, `voter_id`, `reason: string` | *(puste)* |
| `ConsumeEligibilityToken` | `election_id`, `token: EligibilityToken` | `constituency_id`, `expires_at` — **nigdy** `voter_id` |

Uwagi:

- `IdentityProof.adapter` wskazuje adapter, np. `"gov-registry"`, `"mobywatel-eidas"`, `"ldap"`, `"oidc"`. Dokładnie jedno z `proof` lub `auth_session_ref` POWINNO być ustawione (zależnie od adaptera). `auth_session_ref` obejmuje flow po stronie serwera (redirect OIDC, broker mObywatel, sesja urzędnika).
- `voter_id` jest opaque, **stabilny w ramach jednych wyborów** — nigdy surowy PESEL. Między wyborami nielinkowalny dla obserwatora z zewnątrz ([Protokół v1](/suffragio-spec/pl/protocol-v1/)).
- `RevokeVotingRights` usuwa z listy / blokuje **nowe** tokeny. **Nie** unieważnia kart już podpisanych ślepo ani już w logu głosów.
- `ConsumeEligibilityToken` jest **tylko dla BSA** (AuthZ). To jedyna atomowa księga single-use; BSA NIE WOLNO prowadzić równoległego store’a zużyć. Równoległe consume → co najwyżej jeden sukces.

### Zapytania

| RPC | Request | Response |
| --- | --- | --- |
| `GetVoterStatus` | `election_id`, `voter_id` | `registered`, `eligible`, `revoked: bool`, `constituency_id` |
| `GetRegistrationSnapshot` | `election_id` | `cursor`, `captured_at`, `registered_count`, `revoked_count`, `tokens_issued_count`, `tokens_consumed_count` (tylko agregaty — bez dumpa `voter_id`) |

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Kiedy emitowane |
| --- | --- | --- |
| `VoterRegistered` | `election_id`, `constituency_id`, `count: int32`, `occurred_at` | sukces `RegisterVoterRoll` |
| `VoterEligibilityVerified` | `election_id`, `constituency_id`, `occurred_at` | sukces `VerifyIdentity` — **bez `voter_id`**, żeby nie korelować weryfikacji z oddaniem głosu |
| `VoterRightsRevoked` | `election_id`, `occurred_at` | sukces `RevokeVotingRights` |
| `EligibilityTokenConsumed` | `election_id`, `constituency_id`, `occurred_at` | sukces `ConsumeEligibilityToken` — bez wartości tokenu i bez `voter_id` |

Każde `RegistrationEligibilityEvent` ma też `cursor: EventCursor`.

## BlindSignatureAuthority

Zużywa `EligibilityToken` (przez RegSvc) i wystawia ślepy podpis na **całej wypełnionej karcie**, której nie może odczytać. Zob. [wydawanie karty](/suffragio-spec/pl/architecture/#blind-signature-ballot-issuance) oraz [Protokół v1](/suffragio-spec/pl/protocol-v1/). `proto/suffragio/v1/blind_signature.proto`.

W wyborach publicznych ścieżka wyborcy do tej usługi **MUSI** iść Freenetem.

```proto
service BlindSignatureAuthority {
  rpc RequestBlindSignature(RequestBlindSignatureRequest) returns (RequestBlindSignatureResponse);
  rpc GetBlindSignatureSnapshot(GetBlindSignatureSnapshotRequest) returns (GetBlindSignatureSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream BlindSignatureEvent);
}
```

### Komendy

| RPC | Request | Response |
| --- | --- | --- |
| `RequestBlindSignature` | `election_id`, `token: EligibilityToken`, `blinded_ballot: bytes`, `key_id: string`, `suite_id: CryptoSuiteId` | `blind_signature: bytes`, `key_id: string` |

Uwagi:

- `blinded_ballot` to suite-specific zaślepiony encoding **deterministycznego CBOR** wypełnionej karty (domyślny suite `BLIND_SIG_RSA_FDH_3072_SHA256`).
- `key_id` wybiera opublikowany `BsaPublicKey` z konfiguracji wyborów do późniejszej weryfikacji odślepionego podpisu.
- Przed podpisem BSA MUSI wywołać `RegistrationEligibility.ConsumeEligibilityToken`. Przy błędzie — nie podpisywać.

### Zapytania

| RPC | Request | Response |
| --- | --- | --- |
| `GetBlindSignatureSnapshot` | `election_id` | `cursor`, `captured_at`, `signatures_issued_count: uint64` |

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Kiedy emitowane |
| --- | --- | --- |
| `BlindSignatureIssued` | `election_id`, `occurred_at` | sukces `RequestBlindSignature` — bez tokenu, treści karty, podpisu ani tożsamości wyborcy |

Każde `BlindSignatureEvent` ma też `cursor: EventCursor`.

## VoteBroadcastQueue

Publiczny, tylko-dopisywalny, multi-writer log głosów z łańcuchem haszy (eventual consistency). `proto/suffragio/v1/vote_queue.proto`.

W wyborach publicznych `SubmitVote` **MUSI** iść Freenetem. **Nie ma** `receipt_hash` (odporność na przymus).

```proto
service VoteBroadcastQueue {
  rpc SubmitVote(SubmitVoteRequest) returns (SubmitVoteResponse);
  rpc StreamVotes(StreamVotesRequest) returns (stream SignedVote);
  rpc GetLogHead(GetLogHeadRequest) returns (GetLogHeadResponse);
  rpc ReportLogHead(ReportLogHeadRequest) returns (ReportLogHeadResponse);
  rpc GetVoteQueueSnapshot(GetVoteQueueSnapshotRequest) returns (GetVoteQueueSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream VoteQueueEvent);
}
```

### Komendy

| RPC | Request | Response |
| --- | --- | --- |
| `SubmitVote` | `election_id`, `constituency_id`, `ballot: bytes`, `signature: bytes`, `key_id`, `suite_id`, opcjonalnie `prev_hash` | `sequence: uint64`, `entry_hash: bytes`, `prev_hash: bytes` |
| `ReportLogHead` | `election_id`, `reporter_node_id`, `head_hash`, `sequence` | *(puste)* — pomocniczy sygnał sync; **niewiążący** |

Uwagi:

- `ballot` to deterministyczny CBOR wypełnionej karty — **te same bajty**, które obejmuje odślepiony `signature` BSA.
- Queue MUSI zweryfikować podpis BSA (`key_id` względem `bsa_public_keys` wyborów) **oraz** walidować kartę względem DSL okręgu. Zła struktura → **odrzut** (bez append), nawet przy poprawnym podpisie.
- Łańcuch haszy: `entry_hash = H(prev_hash || canonical_entry)` (Protokół v1). Opcjonalne `prev_hash` w requestcie jest doradcze przy multi-writer.
- Celowo **brak** `receipt_hash`. Wyborca trzyma lokalną kopię karty+podpisu i później szuka jej w oficjalnym logu.
- Wiążąca finalność zbioru głosów to pakiet wyników komisji z podpisami m-of-n, nie pojedynczy ACK peera.

### Zapytania

| RPC | Request | Response |
| --- | --- | --- |
| `StreamVotes` | `election_id`, `after_sequence: uint64`, `follow: bool` | strumień `SignedVote` |
| `GetLogHead` | `election_id` | `head_hash`, `sequence`, `observed_at` |
| `GetVoteQueueSnapshot` | `election_id` | `cursor`, `head_hash`, `sequence`, `captured_at` |

`SignedVote` = `election_id`, `constituency_id`, `ballot`, `signature`, `key_id`, `suite_id`, `sequence`, `entry_hash`, `prev_hash`, opcjonalnie `received_at`.

`received_at` jest obecne tylko gdy flaga wyborów `publish_received_at` jest `true`.

`StreamVotes` odtwarza log od `after_sequence` (0 = od początku) i przy `follow = true` streamuje nowe głosy na żywo — tak Tally Engine i audytorzy konsumują kolejkę.

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Kiedy emitowane |
| --- | --- | --- |
| `VoteCast` | `election_id`, `sequence`, `entry_hash`, `occurred_at` | sukces `SubmitVote` |
| `LogHeadReported` | `election_id`, `reporter_node_id`, `head_hash`, `sequence`, `occurred_at` | sukces `ReportLogHead` |

Każde `VoteQueueEvent` ma też `cursor: EventCursor`.

## TallyEngine

Uruchamia przypięty skrypt **Lua** na logu głosów po autoryzowanym zamknięciu i publikuje oficjalny pakiet wyników z podpisami m-of-n. Oficjalny czas zamknięcia to podpisane `CloseVotingWindow`, nie NTP peerów. `proto/suffragio/v1/tally.proto`.

```proto
service TallyEngine {
  rpc CloseVotingWindow(CloseVotingWindowRequest) returns (CloseVotingWindowResponse);
  rpc ComputeResults(ComputeResultsRequest) returns (ComputeResultsResponse);
  rpc PublishResults(PublishResultsRequest) returns (PublishResultsResponse);
  rpc GetResults(GetResultsRequest) returns (GetResultsResponse);
  rpc GetOfficialResultsPackage(GetOfficialResultsPackageRequest) returns (GetOfficialResultsPackageResponse);
  rpc GetTallySnapshot(GetTallySnapshotRequest) returns (GetTallySnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream TallyEvent);
}
```

### Komendy

| RPC | Request | Response |
| --- | --- | --- |
| `CloseVotingWindow` | `election_id`, `authorizing_signatures: bytes[]` | *(puste)* |
| `ComputeResults` | `election_id`, `log_head_hash: bytes` | `results: ElectionResults` |
| `PublishResults` | `election_id`, `package: OfficialResultsPackage` | *(puste)* |

Uwagi:

- `authorizing_signatures` to odłączone podpisy kluczy komisji autoryzujące zamknięcie (polityka m-of-n).
- `ComputeResults` uruchamia przypięty skrypt Lua (`FormulaScriptRef.content_hash`) na zbiorze głosów wskazanym przez `log_head_hash`.
- `OfficialResultsPackage` (normatywna zawartość):
  - `results: ElectionResults`
  - `log_head_hash: bytes`
  - `formula_content_hash: bytes`
  - opcjonalnie `formula_catalog_script_id: string`
  - `signatures: CommissionSignature[]` (`key_id`, `signature`, `signed_at`) — podpisy **m-of-n** komisji/PKW na kanonicznym payloadzie pakietu

`ElectionResults` = `election_id`, `constituency_results: ConstituencyResult[]`, `total_votes_counted`, `invalid_rejected_at_submit`, `computed_at`, `formula_content_hash`, `log_head_hash`.

`ConstituencyResult` = `constituency_id`, `tally_by_choice: map<string, int64>` — klucze to wyłącznie **stabilne id z DSL** (nie etykiety UI).

Brak wbudowanego enuma ordynacji; w v1 tylko Lua.

### Zapytania

| RPC | Request | Response |
| --- | --- | --- |
| `GetResults` | `election_id` | `results: ElectionResults`, `published: bool` |
| `GetOfficialResultsPackage` | `election_id` | `package: OfficialResultsPackage` |
| `GetTallySnapshot` | `election_id` | `cursor`, `closed`, `published`, `log_head_hash`, `captured_at` |

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Kiedy emitowane |
| --- | --- | --- |
| `VotingWindowClosed` | `election_id`, `occurred_at`, `log_head_hash_hint` | sukces `CloseVotingWindow` |
| `ResultsPublished` | `election_id`, `occurred_at`, `results_payload_hash`, `log_head_hash`, `formula_content_hash` | sukces `PublishResults` |

Każde `TallyEvent` ma też `cursor: EventCursor`.

## FormulaCatalog

Przechowuje i udostępnia wielokrotnego użytku skrypty Lua do zliczania. Wybory pinują skrypt przez `content_hash` (inline i/lub `catalog_script_id`). Gotowce frontendu (np. Sejm/Senat/prezydent/referendum PL) to treść katalogu, nie silniki w core. `proto/suffragio/v1/formula_catalog.proto`.

```proto
service FormulaCatalog {
  rpc PublishScript(PublishScriptRequest) returns (PublishScriptResponse);
  rpc GetScript(GetScriptRequest) returns (GetScriptResponse);
  rpc ListScripts(ListScriptsRequest) returns (ListScriptsResponse);
  rpc GetFormulaCatalogSnapshot(GetFormulaCatalogSnapshotRequest) returns (GetFormulaCatalogSnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream FormulaCatalogEvent);
}
```

### Komendy

| RPC | Request | Response |
| --- | --- | --- |
| `PublishScript` | `title`, `description`, `tags: string[]`, `script: bytes` (źródło Lua) | `script_id: string`, `content_hash: bytes` |

### Zapytania

| RPC | Request | Response |
| --- | --- | --- |
| `GetScript` | `script_id`, opcjonalnie `expected_content_hash` | `metadata: FormulaScriptMetadata`, `script: bytes` |
| `ListScripts` | `query`, `page_size`, `page_token`, opcjonalnie `tag` | `scripts: FormulaScriptMetadata[]`, `next_page_token` |
| `GetFormulaCatalogSnapshot` | *(puste)* | `cursor`, `scripts: FormulaScriptMetadata[]`, `captured_at` |

`FormulaScriptMetadata` = `script_id`, `title`, `description`, `tags[]` (np. `"pl-sejm"`, `"pl-president"`, `"referendum"`), `content_hash`, `published_at`.

Jeśli ustawiono `expected_content_hash` w `GetScript`, serwer MUSI zwrócić not-found, gdy bajty się nie zgadzają.

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Kiedy emitowane |
| --- | --- | --- |
| `ScriptPublished` | `metadata`, `occurred_at` | sukces `PublishScript` |

Każde `FormulaCatalogEvent` ma też `cursor: EventCursor`.

## Discovery

Ogłoszenia węzłów na overlayu P2P oraz publiczny Election Catalog. `proto/suffragio/v1/discovery.proto`.

Suffragio nie jest jedną globalną siecią. Jak swarmy BitTorrent z różnymi trackerami, różni organizatorzy mogą prowadzić wybory na fizycznie osobnych sieciach P2P, każda koordynowana przez własny węzeł **trackera**. `TrackerRef` (klucz Freenet) identyfikuje sieć — stąd request/response i zdarzenia niosą tracker, żeby klient wiedział, do której sieci dołączyć.

```proto
message TrackerRef {
  string freenet_key = 1;
}

service Discovery {
  rpc AnnounceNode(AnnounceNodeRequest) returns (AnnounceNodeResponse);
  rpc DiscoverElections(DiscoverElectionsRequest) returns (DiscoverElectionsResponse);
  rpc GetDiscoverySnapshot(GetDiscoverySnapshotRequest) returns (GetDiscoverySnapshotResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream DiscoveryEvent);
}
```

### Komendy

| RPC | Request | Response |
| --- | --- | --- |
| `AnnounceNode` | `node: NodeInfo` (`node_id`, `roles: NodeRole[]`, `freenet_key`, `tracker: TrackerRef`) | `tracker: TrackerRef` — potwierdza, która sieć przyjęła ogłoszenie |

`NodeRole` obejmuje: `ELECTION_REGISTRY`, `REGISTRATION_ELIGIBILITY`, `BLIND_SIGNATURE_AUTHORITY`, `VOTE_BROADCAST_QUEUE`, `TALLY_ENGINE`, `CATALOG_MIRROR`, `FORMULA_CATALOG`.

### Zapytania

| RPC | Request | Response |
| --- | --- | --- |
| `DiscoverElections` | `query: string`, `page_size: int32`, `page_token: string`, `tracker: TrackerRef` (opcjonalnie — zawężenie do jednej sieci) | `elections: ElectionSummary[]`, `next_page_token: string`, `tracker: TrackerRef` — sieć, która odpowiedziała |
| `GetDiscoverySnapshot` | *(puste)* | `cursor`, `nodes: NodeInfo[]`, `captured_at` |

`ElectionSummary` = `election_id`, `title`, `constituencies[]`, `voting_window`, `published: bool`, `tracker: TrackerRef`, `state: ElectionState` — model odczytu [Election Catalog](/suffragio-spec/pl/architecture/#system-components). Dwa wybory w tej samej odpowiedzi mogą wskazywać różne trackery.

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Kiedy emitowane |
| --- | --- | --- |
| `NodeAnnounced` | `node: NodeInfo`, `occurred_at`, `tracker: TrackerRef` | sukces `AnnounceNode` |

Każde `DiscoveryEvent` ma też `cursor: EventCursor`.
