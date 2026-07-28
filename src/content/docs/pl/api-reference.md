---
title: Specyfikacja API gRPC
description: Pełna specyfikacja protokołu gRPC dla wszystkich serwisów Suffragio — komendy, zapytania i strumienie zdarzeń, wygenerowane na podstawie kanonicznych definicji .proto.
---

Ta strona dokumentuje protokół gRPC dla każdego serwisu opisanego w [Architekturze systemu](/suffragio-spec/pl/architecture/). Odzwierciedla kanoniczne, gotowe do implementacji pliki `.proto` w [`proto/suffragio/v1/`](https://github.com/Suffragio/suffragio-spec/tree/main/proto/suffragio/v1) — traktuj pliki `.proto` jako źródło prawdy, a tę stronę jako czytelne dla człowieka odniesienie. Zobacz [`proto/README.md`](https://github.com/Suffragio/suffragio-spec/blob/main/proto/README.md), aby dowiedzieć się, jak je lintować i generować z nich kod za pomocą [buf](https://buf.build).

Każdy serwis ma tę samą strukturę:

- **Komendy** — unarne wywołania RPC modyfikujące stan.
- **Zapytania** — unarne lub strumieniowe wywołania RPC odczytujące stan bez jego modyfikacji.
- **Zdarzenia** — pojedyncze strumieniowe wywołanie RPC `WatchEvents` zwracające zdarzenie typu `oneof`, dzięki czemu każdy węzeł lub audytor może subskrybować tylko zdarzenia danego serwisu i samodzielnie odtworzyć jego stan.

Wszystkie komunikaty są zdefiniowane w pakiecie `suffragio.v1`. Wspólne typy (`ElectionId`, `ConstituencyId`, `EligibilityToken`, `ElectoralFormula`, `Constituency`, `VotingWindow`, `BallotTemplate`) znajdują się w `common.proto` i są używane przez wszystkie serwisy poniżej.

## ElectionRegistry

Źródło prawdy dla niezmiennej konfiguracji wyborów. `proto/suffragio/v1/election_registry.proto`.

```proto
service ElectionRegistry {
  rpc CreateElection(CreateElectionRequest) returns (CreateElectionResponse);
  rpc DefineBallotTemplate(DefineBallotTemplateRequest) returns (DefineBallotTemplateResponse);
  rpc SetElectoralFormula(SetElectoralFormulaRequest) returns (SetElectoralFormulaResponse);
  rpc ScheduleElection(ScheduleElectionRequest) returns (ScheduleElectionResponse);
  rpc PublishElection(PublishElectionRequest) returns (PublishElectionResponse);
  rpc GetElection(GetElectionRequest) returns (GetElectionResponse);
  rpc ListElections(ListElectionsRequest) returns (ListElectionsResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream ElectionRegistryEvent);
}
```

### Komendy

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `CreateElection` | `title: string`, `constituencies: Constituency[]` | `election_id: ElectionId` |
| `DefineBallotTemplate` | `election_id`, `template: BallotTemplate` | *(pusta)* |
| `SetElectoralFormula` | `election_id`, `formula: ElectoralFormula`, `params: map<string,string>` | *(pusta)* |
| `ScheduleElection` | `election_id`, `voting_window: VotingWindow` | *(pusta)* |
| `PublishElection` | `election_id` | *(pusta)* |

### Zapytania

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `GetElection` | `election_id` | `election: ElectionConfig` |
| `ListElections` | `page_size: int32`, `page_token: string` | `elections: ElectionConfig[]`, `next_page_token: string` |

`ElectionConfig` = `election_id`, `title`, `constituencies[]`, `ballot_templates[]`, `formula`, `voting_window`, `published: bool`.

### Zdarzenia (`WatchEvents`)

Subskrybuj z pustym `election_id`, aby otrzymywać zdarzenia dla wszystkich wyborów, lub z konkretnym, aby filtrować.

| Zdarzenie | Pola | Emitowane gdy |
| --- | --- | --- |
| `ElectionCreated` | `election_id`, `title`, `occurred_at` | `CreateElection` się powiedzie |
| `BallotTemplateDefined` | `election_id`, `template`, `occurred_at` | `DefineBallotTemplate` się powiedzie |
| `ElectoralFormulaSet` | `election_id`, `formula`, `occurred_at` | `SetElectoralFormula` się powiedzie |
| `ElectionScheduled` | `election_id`, `voting_window`, `occurred_at` | `ScheduleElection` się powiedzie |
| `ElectionPublished` | `election_id`, `occurred_at` | `PublishElection` się powiedzie |

## RegistrationEligibility

Waliduje tożsamość i okręg wyborcy oraz wydaje jednorazowe `EligibilityToken`. Nigdy nie rozmawia bezpośrednio z zewnętrznym systemem tożsamości — zobacz [Integrację zewnętrznej tożsamości](/suffragio-spec/pl/architecture/#integracja-zewnętrznej-tożsamości-warstwa-antykorupcyjna). `proto/suffragio/v1/registration_eligibility.proto`.

```proto
service RegistrationEligibility {
  rpc RegisterVoterRoll(RegisterVoterRollRequest) returns (RegisterVoterRollResponse);
  rpc VerifyIdentity(VerifyIdentityRequest) returns (VerifyIdentityResponse);
  rpc RevokeVotingRights(RevokeVotingRightsRequest) returns (RevokeVotingRightsResponse);
  rpc GetVoterStatus(GetVoterStatusRequest) returns (GetVoterStatusResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream RegistrationEligibilityEvent);
}
```

### Komendy

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `RegisterVoterRoll` | `election_id`, `voters: VoterEntry[]` (`voter_id`, `constituency_id`) | `registered_count: int32` |
| `VerifyIdentity` | `election_id`, `proof: IdentityProof` (`adapter: string`, `assertion: bytes`) | `token: EligibilityToken`, `constituency_id`, `expires_at` |
| `RevokeVotingRights` | `election_id`, `voter_id`, `reason: string` | *(pusta)* |

`IdentityProof.adapter` identyfikuje, który [adapter zewnętrznej tożsamości](/suffragio-spec/pl/architecture/#integracja-zewnętrznej-tożsamości-warstwa-antykorupcyjna) wystawił potwierdzenie, np. `"gov-registry"`, `"mobywatel-eidas"`, `"ldap"`, `"oidc"`.

### Zapytania

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `GetVoterStatus` | `election_id`, `voter_id` | `registered`, `eligible`, `revoked: bool`, `constituency_id` |

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Emitowane gdy |
| --- | --- | --- |
| `VoterRegistered` | `election_id`, `constituency_id`, `count: int32`, `occurred_at` | `RegisterVoterRoll` się powiedzie |
| `VoterEligibilityVerified` | `election_id`, `constituency_id`, `occurred_at` | `VerifyIdentity` się powiedzie — **bez `voter_id`**, aby uniknąć powiązania weryfikacji tożsamości z oddaniem głosu |
| `VoterRightsRevoked` | `election_id`, `occurred_at` | `RevokeVotingRights` się powiedzie |

## BlindSignatureAuthority

Konsumuje `EligibilityToken` i wydaje ślepy podpis nad kartą, której nie może odczytać. Zobacz [Wydawanie kart z użyciem ślepego podpisu](/suffragio-spec/pl/architecture/#wydawanie-kart-do-głosowania-z-użyciem-ślepego-podpisu). `proto/suffragio/v1/blind_signature.proto`.

```proto
service BlindSignatureAuthority {
  rpc RequestBlindSignature(RequestBlindSignatureRequest) returns (RequestBlindSignatureResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream BlindSignatureEvent);
}
```

### Komendy

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `RequestBlindSignature` | `election_id`, `token: EligibilityToken`, `blinded_ballot: bytes` | `blind_signature: bytes` |

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Emitowane gdy |
| --- | --- | --- |
| `BlindSignatureIssued` | `election_id`, `occurred_at` | `RequestBlindSignature` się powiedzie — bez tokenu, treści karty czy podpisu |

## VoteBroadcastQueue

Publiczny, tylko-dopisywalny, replikowany dziennik wszystkich oddanych głosów. `proto/suffragio/v1/vote_queue.proto`.

```proto
service VoteBroadcastQueue {
  rpc SubmitVote(SubmitVoteRequest) returns (SubmitVoteResponse);
  rpc StreamVotes(StreamVotesRequest) returns (stream SignedVote);
  rpc WatchEvents(WatchEventsRequest) returns (stream VoteQueueEvent);
}
```

### Komendy

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `SubmitVote` | `election_id`, `constituency_id`, `ballot: bytes`, `signature: bytes` | `sequence: uint64`, `receipt_hash: bytes` |

### Zapytania

| RPC | Żądanie | Odpowiedź (strumień) |
| --- | --- | --- |
| `StreamVotes` | `election_id`, `after_sequence: uint64`, `follow: bool` | `SignedVote` (`election_id`, `constituency_id`, `ballot`, `signature`, `sequence`, `received_at`) |

`StreamVotes` odtwarza cały dziennik od `after_sequence`, a jeśli `follow` jest `true`, kontynuuje strumieniowanie nowych głosów na żywo — tak Silnik liczenia głosów i audytorzy konsumują kolejkę.

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Emitowane gdy |
| --- | --- | --- |
| `VoteCast` | `election_id`, `sequence: uint64`, `occurred_at` | `SubmitVote` się powiedzie |

## TallyEngine

Stosuje skonfigurowaną ordynację wyborczą do dziennika głosów po zamknięciu okna głosowania. `proto/suffragio/v1/tally.proto`.

```proto
service TallyEngine {
  rpc CloseVotingWindow(CloseVotingWindowRequest) returns (CloseVotingWindowResponse);
  rpc ComputeResults(ComputeResultsRequest) returns (ComputeResultsResponse);
  rpc PublishResults(PublishResultsRequest) returns (PublishResultsResponse);
  rpc GetResults(GetResultsRequest) returns (GetResultsResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream TallyEvent);
}
```

### Komendy

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `CloseVotingWindow` | `election_id` | *(pusta)* |
| `ComputeResults` | `election_id` | `results: ElectionResults` |
| `PublishResults` | `election_id` | *(pusta)* |

`ElectionResults` = `election_id`, `formula`, `constituency_results: ConstituencyResult[]`, `total_votes_counted: uint64`, `computed_at`. `ConstituencyResult` = `constituency_id`, `tally_by_choice: map<string,int64>`.

### Zapytania

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `GetResults` | `election_id` | `results: ElectionResults`, `published: bool` |

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Emitowane gdy |
| --- | --- | --- |
| `VotingWindowClosed` | `election_id`, `occurred_at` | `CloseVotingWindow` się powiedzie |
| `ResultsPublished` | `election_id`, `occurred_at` | `PublishResults` się powiedzie |

## Discovery

Ogłaszanie węzłów w sieci nakładkowej P2P oraz publiczny Katalog wyborów. `proto/suffragio/v1/discovery.proto`.

Suffragio nie jest jedną globalną siecią. Podobnie jak roje BitTorrent koordynowane przez różne trackery, różni organizatorzy mogą prowadzić swoje wybory na fizycznie odrębnych sieciach P2P, każda koordynowana przez własny węzeł-**tracker**. `TrackerRef` (po prostu domena I2P) identyfikuje, do jakiej sieci należą dane — dlatego **każde** żądanie/odpowiedź i zdarzenie poniżej niesie ten identyfikator, dzięki czemu klient wie, w jakiej sieci faktycznie znajduje się węzeł lub wybory oraz która sieć odpowiedziała na jego zapytanie.

```proto
message TrackerRef {
  string i2p_domain = 1;
}

service Discovery {
  rpc AnnounceNode(AnnounceNodeRequest) returns (AnnounceNodeResponse);
  rpc DiscoverElections(DiscoverElectionsRequest) returns (DiscoverElectionsResponse);
  rpc WatchEvents(WatchEventsRequest) returns (stream DiscoveryEvent);
}
```

### Komendy

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `AnnounceNode` | `node: NodeInfo` (`node_id`, `roles: NodeRole[]`, `i2p_address`, `freenet_key`, `tracker: TrackerRef`) | `tracker: TrackerRef` — potwierdza, która sieć przyjęła ogłoszenie |

`NodeRole` wylicza: `ELECTION_REGISTRY`, `REGISTRATION_ELIGIBILITY`, `BLIND_SIGNATURE_AUTHORITY`, `VOTE_BROADCAST_QUEUE`, `TALLY_ENGINE`, `CATALOG_MIRROR`.

### Zapytania

| RPC | Żądanie | Odpowiedź |
| --- | --- | --- |
| `DiscoverElections` | `query: string`, `page_size: int32`, `page_token: string`, `tracker: TrackerRef` (opcjonalnie — ograniczenie do jednej sieci) | `elections: ElectionSummary[]`, `next_page_token: string`, `tracker: TrackerRef` — sieć, która odpowiedziała |

`ElectionSummary` = `election_id`, `title`, `constituencies[]`, `voting_window`, `published: bool`, `tracker: TrackerRef` — to jest model odczytu stojący za [Katalogiem wyborów](/suffragio-spec/pl/architecture/#komponenty-systemu). Dwa wybory w tej samej odpowiedzi mogą wskazywać na dwa różne trackery, bo usługi każdych wyborów mogą działać na innej sieci.

### Zdarzenia (`WatchEvents`)

| Zdarzenie | Pola | Emitowane gdy |
| --- | --- | --- |
| `NodeAnnounced` | `node: NodeInfo`, `occurred_at`, `tracker: TrackerRef` | `AnnounceNode` się powiedzie |
