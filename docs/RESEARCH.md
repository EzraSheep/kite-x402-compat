# Research notes (Phase 0)

Baseline for every statement below. Nothing here is claimed beyond what was
actually read; anything not read is listed under "Not reviewed".

| Source | Revision read | Date of revision |
| --- | --- | --- |
| x402-foundation/x402 (spec, TS reference server) | `911709c32e62641f7bd4df345b2029ebe84968c8` (HEAD when read) | 2026-11-18 |
| atsp-protocol/x402-conformance | `46f52f8b412ba7992d934fd9a30b0a43f974907c` | 2026-11-04 |
| tianzeshi-study/kite-x402-axum | `32b096377b9418b2cfb2f0ec45f5e5bff1bbbe2b` | 2026-11-18 |
| BrKDDD/kite-x402-hono | `5e2adeb97ffb0d3a2d5b1c3f70f84831a316fff4` | 2026-11-16 |

## 1. What the x402 V2 specification fixes (and what it leaves open)

Read in full: `specs/x402-specification-v2.md`, `specs/transports-v2/http.md`;
read in part: `specs/schemes/exact/scheme_exact_evm.md` (the `extra` rules).

Fixed by the spec (this project tests these as normative):

- HTTP transport: `402` + base64 JSON in `PAYMENT-REQUIRED`; client sends
  `PAYMENT-SIGNATURE`; server returns `PAYMENT-RESPONSE`.
- `PaymentRequired`: `x402Version` (2), `resource.url`, non-empty `accepts`.
- `PaymentRequirements`: `scheme`, `network` (CAIP-2), `amount` (atomic units,
  string), `asset`, `payTo`, `maxTimeoutSeconds`, optional `extra`.
- `extra.paymentFlow` / `extra.assetTransferMethod` are reserved keys;
  flows are `authorization` (verify -> resource -> settle), `upfront`
  (settle -> resource), `escrow` (settle -> resource -> settle); at least one
  check must run before the resource executes (section 6.1).
- Facilitator: `POST /verify` is read-only, `POST /settle`, `GET /supported`
  (`kinds`, `extensions`, `signers` all required).
- HTTP error mapping: payment required / payment failed = 402, invalid
  payment = 400, server error = 500.
- HTTP "Browser clients": none of the three headers is CORS-safelisted; a
  browser-reachable server SHOULD expose/allow them (all SHOULD, not MUST).

Left open by the spec (so this tool must not invent a normative rule):

- **How the server matches `PaymentPayload.accepted` to its own `accepts`.**
  The spec only says `accepted` "indicat[es] the payment method chosen". The
  TS reference server (`x402ResourceServer.ts`, `findMatchingRequirements`)
  does this: the first advertised option whose core fields (`scheme`,
  `network`, `amount`, `asset`, `payTo`, `maxTimeoutSeconds`) deep-equal the
  accepted ones, and whose server-declared `extra` keys are a *subset* of
  `accepted.extra` (clients may add keys; scheme-declared dynamic keys are
  ignored). This project encodes exactly that as its oracle and labels the
  resulting checks "reference behaviour", not "spec MUST".
- Header size limits, the meaning of `maxTimeoutSeconds` bounds, and how a
  server should treat duplicate or ambiguous `accepts` entries.

## 2. Existing tools

### x402-conformance (atsp-protocol), Apache-2.0

Mature: 84 catalogued checks (41 normative / 28 security / 15 resilience),
Python and Node CLIs, GitHub Action, doubles for facilitator and scheme, fault
injection, evidence from 15 public facilitators. Read: README, `docs/COVERAGE.md`,
`docs/SPEC.md` index. Not read: its source code line by line.

Already covered there, so **not re-implemented here**: `GET /supported` shape
(S0-S4), 402 shape (P1-P7), settle/verify faults and `settlement_pending`
(F*, R3-R9, R18), replay / duplicate settlement (R2, R12, R13, R20*), HEAD
paywall (R16a), paid-response cache leak (R17), malformed header -> 400
(R24), upfront/escrow flow gating (R19), `accepted` bound to the route's own
requirements and recipient/asset/network mismatch (R11a, R11b), upto and
batch-settlement, MCP and A2A.

Searched and **found nothing** in its docs and sources at the revision above
(`grep -i cors`, `grep -i kite` over py/ts/md; the only `kite` hit is the word
"Buildkite"):

- CORS / browser-client checks from the HTTP transport spec.
- Anything Kite-specific (networks `eip155:2366` / `eip155:2368`, their
  assets, EIP-712 domain, decimals).
- Static analysis of the `accepts` *list itself* (duplicates, ambiguous
  options, unknown flow values). Its matching checks cover a single mismatch
  against one route, not the per-field matrix against a multi-option route.
- A Rust-native library (it ships Python, TypeScript and Lean models).

"Found nothing" means nothing in what was searched, not proof of absence in
code paths not read. `docs/PROTOCOL-COVERAGE.md` keeps this per feature.

### Official repository

Contains SDKs, examples and E2E tests. Only the spec files and the TS
`x402ResourceServer.ts` were read. SDK test suites and `e2e/` were **not
reviewed**; nothing here claims anything about their coverage.

## 3. Kite ecosystem facts used by the Kite profile

Two community reference projects agree with each other, neither is official
Kite documentation, and nothing was verified on chain:

| | Mainnet | Testnet |
| --- | --- | --- |
| Network (CAIP-2) | `eip155:2366` | `eip155:2368` |
| Asset (as configured by both projects) | Bridged USDC `0x7aB6f3ed87C42eF0aDb67Ed95090f8bF5240149e` | pieUSD `0x38129cf4CE5E183eFF248F42A7D345Bb1B47621A` |
| Decimals | 6 | **18** |
| EIP-712 domain name / version | `Bridged USDC (Kite AI)` / `2` | `pieUSD` / `1` |

Sources: `kite-x402-axum` `kite-x402-axum/src/kite.rs` and `kite-x402-hono`
`src/networks.ts`. The facilitator both projects default to is
`https://facilitator.pieverse.io/v2`. The profile file therefore carries a
`source` field and `verified_on_chain = false`; the tool can optionally check
chain id and contract code over JSON-RPC (read-only).

The 6 vs 18 decimal difference is why the Kite profile includes an amount
plausibility check: a price written for 6 decimals is dust on an 18-decimal
asset.

## 4. Consequences for scope

1. Do not rebuild what x402-conformance covers; link to it as the baseline and
   document the overlap in `PROTOCOL-COVERAGE.md`.
2. Differentiators: `accepts`-list analysis, a per-field option-matching
   probe with the reference-implementation oracle, CORS checks, the Kite
   profile, an offline `inspect` command, a Rust library others can embed.
3. Independence: the tool talks HTTP only; it never imports `kite-x402-axum`.
   The Kite profile values are *configuration data with provenance*, not
   copied logic.

## 5. Not reviewed (explicitly)

kite-x402-explorer, kite-agent-zk-attestation, kite_counter_dapp; the official
repo's `e2e/` and SDK tests; the `upto`, `batch-settlement` and extension
specs; x402-conformance source code. If any of these changes the scope, this
file must be updated first.
