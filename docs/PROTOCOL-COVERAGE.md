# Protocol coverage matrix

Every check has a stable ID. `Level` decides how a violation is reported:

| Level | Meaning | Reported as |
| --- | --- | --- |
| `normative` | The spec text says MUST / defines the field or rule | `FAIL` |
| `reference` | Spec is silent; the official TS reference server behaves this way | `WARN` |
| `recommended` | Spec SHOULD, or an operational recommendation | `WARN` |
| `profile` | Expectation the user opted into through a profile file | `FAIL` |

Spec = x402 V2 spec at upstream `911709c` (see `RESEARCH.md`).
"x402-conformance" = atsp-protocol/x402-conformance `46f52f8`, read from its
`docs/COVERAGE.md` only. "Same" means the same property is checked there, so
this project's version is a Rust-native re-implementation for embedding and
offline use, not new coverage. "Partial" means related, narrower or different.
"None found" means nothing matching was found in what was read.
Official SDK/E2E tests were not reviewed (`RESEARCH.md` section 5).

Status: `planned` or `implemented`. Only `implemented` rows are claimed.

| ID | Feature | Spec reference | Level | x402-conformance | Status |
| --- | --- | --- | --- | --- | --- |
| HTTP-001 | Unpaid request returns 402 | http.md Payment Required Signaling | normative | Same (P1) | planned |
| HTTP-002 | `PAYMENT-REQUIRED` header present | http.md Payment Required Signaling | normative | Same (P2) | planned |
| HTTP-003 | `PAYMENT-REQUIRED` header size <= 8 KiB | none (operational) | recommended | Same (P6) | planned |
| HDR-001 | Header is standard padded base64 | http.md; RFC 4648 s4 | normative | Partial (P2) | planned |
| HDR-002 | Decoded bytes are a UTF-8 JSON object | spec 5.1.1 | normative | Partial (P2) | planned |
| SCH-001 | `x402Version` is 2 | spec 5.1.2 | normative | Same (P3) | planned |
| SCH-002 | `resource.url` present; optional field limits | spec 5.1.2 ResourceInfo | normative | Partial (P3) | planned |
| SCH-003 | `accepts` is a non-empty array | spec 5.1.2 | normative | Same (P3) | planned |
| SCH-004 | Option has required fields with correct JSON types | spec 5.1.2 | normative | Same (P4) | planned |
| SCH-005 | `network` is CAIP-2 | spec 5.1.2, 11.1 | normative | Same (P4) | planned |
| SCH-006 | `amount` is an atomic-unit digit string | spec 5.1.2 | normative | Same (P4) | planned |
| SCH-007 | `maxTimeoutSeconds` is a positive integer | spec 5.1.2 | normative | Same (P4) | planned |
| SCH-008 | `extra` is an object; reserved keys typed | spec 5.1.2, 6.1 | normative | Partial (P7) | planned |
| SCH-009 | Extensions have `info` and `schema` | spec 5.1.2 | normative | None found | planned |
| SCH-010 | EVM `exact` (eip3009): `extra.name`, `extra.version` | schemes/exact/scheme_exact_evm.md | normative | Same (P5) | planned |
| OPT-001 | No exact duplicate options in `accepts` | none (spec silent) | recommended | None found | planned |
| OPT-002 | No ambiguous options (same scheme/network/asset/payTo, different amount) | none (reference picks first match) | recommended | None found | planned |
| OPT-003 | `extra.paymentFlow` value is a known flow | spec 6.1 | recommended | Partial (P7) | planned |
| OPT-010 | Every advertised option, echoed exactly, is accepted (first, middle, last) | spec 2, 5.2.2 | normative | Partial (R11) | planned |
| OPT-011 | `accepted` with unknown network is rejected | spec 5.2.2 | normative | Partial (R11b) | planned |
| OPT-012 | `accepted` with unknown asset is rejected | spec 5.2.2 | normative | Partial (R11b) | planned |
| OPT-013 | `accepted` with a different scheme is rejected | spec 5.2.2 | normative | None found | planned |
| OPT-014 | `accepted` with a different amount is rejected | spec 5.2.2 | normative | Partial (R11a) | planned |
| OPT-015 | `accepted` with a different `payTo` is rejected | spec 5.2.2 | normative | Partial (R11b) | planned |
| OPT-016 | `accepted` with a different `maxTimeoutSeconds` is rejected | reference `findMatchingRequirements` | reference | None found | planned |
| OPT-017 | `accepted` with a changed server-declared `extra` value is rejected | reference `findMatchingRequirements` | reference | None found | planned |
| OPT-018 | `accepted` with an additional `extra` key is accepted | reference `paymentRequirementsMatchAccepted` | reference | None found | planned |
| FAC-001 | `GET /supported` answers 200 with a JSON object | spec 7.3 | normative | Same (S0) | planned |
| FAC-002 | `kinds`, `extensions`, `signers` present with correct types | spec 7.3.1 | normative | Same (S1) | planned |
| FAC-003 | Every kind has version, scheme, CAIP-2 network | spec 7.3.1 | normative | Same (S1, S2) | planned |
| FAC-004 | `signers` maps CAIP-2 patterns to string arrays | spec 7.3 | normative | Partial (S4) | planned |
| FAC-005 | At least one V2 kind is advertised | spec 7.3 | recommended | Partial (S1) | planned |
| PAY-001 | Unpaid request never reaches the facilitator | none | recommended | None found | planned |
| PAY-002 | A valid payment is served | spec 2 | normative | Same (R1) | planned |
| PAY-003 | Facilitator call order matches the resolved payment flow | spec 6.1 | normative | Partial (R19, library only) | planned |
| PAY-004 | Failure of the pre-resource check blocks the resource | spec 6.1 invariant | normative | Partial (R3-R9) | planned |
| PAY-005 | Failed post-resource settle is not served as success | http.md Error Handling | normative | Partial (R15) | planned |
| PAY-006 | Facilitator 5xx / invalid JSON / hang blocks the resource (fail closed) | none | recommended | Partial (R3-R9) | planned |
| PAY-007 | `PAYMENT-RESPONSE` on success is a valid `SettlementResponse` | spec 5.3 | normative | None found | planned |
| PAY-008 | Server verifies against its own advertised requirements | spec 5.2.2, 7.1 | normative | Partial (R11a, F10) | planned |
| CORS-001 | 402 carries `Access-Control-Allow-Origin` for the caller | http.md Browser clients | recommended | None found | planned |
| CORS-002 | `Access-Control-Expose-Headers` names both response headers | http.md Browser clients | recommended | None found | planned |
| CORS-003 | Preflight is answered without payment | http.md Browser clients | recommended | None found | planned |
| CORS-004 | Preflight allows `PAYMENT-SIGNATURE` and `Content-Type` | http.md Browser clients | recommended | None found | planned |
| CORS-005 | Preflight sends a non-trivial `Access-Control-Max-Age` | http.md Browser clients | recommended | None found | planned |
| CORS-006 | `Vary: Origin` when the origin is reflected | http.md Browser clients | recommended | None found | planned |
| CACHE-001 | The 402 is not marked shared-cacheable | http.md Browser clients | recommended | None found (R17 is the paid 200) | planned |
| KITE-001 | Profile file loads and is internally valid | n/a | profile | None found | planned |
| KITE-002 | An advertised option targets a profile network | n/a | profile | None found | planned |
| KITE-003 | Asset equals the profile's asset for that network | n/a | profile | None found | planned |
| KITE-004 | Scheme is one the profile expects | n/a | profile | None found | planned |
| KITE-005 | `extra.name` / `extra.version` equal the profile's EIP-712 domain | n/a | profile | None found | planned |
| KITE-006 | Amount is plausible for the asset's decimals | n/a | recommended | None found | planned |
| KITE-007 | Facilitator advertises the profile's version/scheme/network | spec 7.3 | profile | None found | planned |
| KITE-008 | RPC `eth_chainId` equals the profile network (opt-in) | n/a | profile | None found | planned |
| KITE-009 | Asset address has contract code on the RPC (opt-in) | n/a | profile | None found | planned |

## Deliberately not covered (use x402-conformance)

Replay and duplicate-settlement protection, concurrency and multi-instance
tests, HEAD handling, paid-response cache leaks, malformed-header status
mapping, `settlement_pending` reconciliation, `upto` and `batch-settlement`,
MCP and A2A. This project links to that tool instead of copying its checks.

## Known limits

- Active probes (`OPT-01x`, `PAY-*`) only run against a server that has been
  configured to use the tool's loopback mock facilitator.
- `escrow` is exercised only as "two settles around the resource"; the
  mechanism-specific `step` distinction is not modelled.
- Scheme-declared dynamic `extra` fields cannot be discovered black-box; the
  profile can list them (`dynamic_extra_fields`).
