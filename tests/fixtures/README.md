# Protocol 30 header fixtures

`signed_ms.xdr` and `empty_tx_set_ms.xdr` are synthetic
`LedgerHeaderHistoryEntry` values assembled directly from the CAP-88 wire
layout, rather than encoded by the library under test. They are not signed
network ledgers: hashes, node ID and signature are placeholders.

Both use ledger 42, protocol 30, `closeTime = 1700000000` seconds and
`closeTimeMs = 1700000000123`. The extension discriminants are 3 and 4.
The empty-set proposal carries previous protocol version 29.

Wire order: zero entry hash; ledger version; zero previous hash; SCP value
(zero tx-set hash, seconds, empty upgrades, extension); zero result and bucket
hashes; ledger 42; zero coins/fee pool/inflation sequence/ID pool; base fee 100;
base reserve 5000000; max tx-set size 1000; four zero skip hashes; header ext 0;
history-entry ext 0. The extension contains milliseconds followed by the
signature; arm 4 inserts zero proposed tx-set/previous hashes and version 29
before the signature. The signature contains an Ed25519 node ID (type 0,
zero key) and an empty variable-length signature.

Reference: https://github.com/stellar/stellar-protocol/blob/master/core/cap-0088.md
