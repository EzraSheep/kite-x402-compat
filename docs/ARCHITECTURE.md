# Architecture

`kite-x402-compat` tests an x402 implementation through its public HTTP
interface only. It never links against, imports or reads the internals of the
implementation under test.

## Crates

```
crates/
  x402-core     wire types, header codec, CAIP-2, atomic amounts,
                validators, option analysis, option matching oracle,
                check catalogue.      No HTTP, no async, no CLI.
  x402-report   Status / TestResult / Report, summary, JSON, text and
                Markdown rendering, redaction, exit codes.
  x402-http     guarded HTTP client (timeouts, size caps, SSRF guard),
                resource inspector, CORS checks, facilitator inspector,
                Kite profile checks (incl. optional JSON-RPC).
  x402-testkit  scriptable mock facilitator, configurable reference resource
                server with injectable faults, option-matching and lifecycle
                probes, self-test matrix.
  x402-cli      clap front end, exit codes.
```

Dependency direction: `core <- report <- http <- testkit <- cli`
(`http` and `testkit` also use `core`). `core` has no I/O dependencies.

## Check model

`x402-core::catalog` lists every check once: ID, name, category, level, spec
reference (see `PROTOCOL-COVERAGE.md`). Validators in `core` return
`Finding { check, severity, path, message }`; they know nothing about HTTP.
A runner turns the catalogue plus findings into one `TestResult` per check:

- no finding -> `PASS`;
- findings -> `FAIL` for `normative` / `profile` checks, `WARN` for
  `reference` / `recommended`; a finding can itself only lower severity;
- prerequisites missing -> `SKIP`; the tool could not finish -> `ERROR`.

Unsupported capabilities (a facilitator that does not offer a network) are
`SKIP` or `WARN`, never `FAIL`.

## Data flow

```
resource <url>  -> guarded GET (unpaid) -> 402? -> decode header -> validate
                   -> accepts analysis -> [--cors: preflight + 402 headers]
                   -> [--profile: Kite checks] -> Report
facilitator <url> -> guarded GET /supported -> validate -> [--profile] -> Report
probe             -> mock facilitator (loopback) <- target resource server
                     unpaid GET -> advertised options
                     OPT-01x: mutated `accepted` per field
                     PAY-*:   valid / rejected / failing facilitator scripts,
                              ordered call log asserted -> Report
self-test         -> reference server (control + faulty builds) probed by the
                     same code; every check must PASS on the control and the
                     targeted check must FAIL on its faulty build
inspect <file>    -> decode + validate a saved object, no network
```

## Security model

- Default commands are read-only: one unpaid request (plus an OPTIONS
  preflight with `--cors`). Nothing is signed; there is no key handling code.
- Targets resolving to loopback / private / link-local / CGNAT / reserved
  addresses are refused unless `--allow-local`. Resolution is pinned per hop
  and redirects are followed manually and re-validated.
- Response bodies and headers are size-capped; JSON nesting is capped by
  `serde_json`; base64 decoding is bounded before allocation.
- Active probes need `--yes-run-active-probes`, bind the mock facilitator to
  loopback by default, and send only synthetic payloads.
- Reports never contain `PAYMENT-SIGNATURE` values; payload secrets are
  redacted by key name.

## Extension points

- New scheme-specific rules: add a validator keyed by `(scheme, network
  namespace)` in `x402-core::validate` and a catalogue row.
- New protocol version: `core::types` is V2-only on purpose; unsupported
  versions are reported (`SCH-001`), never reinterpreted.
- New framework adapter: none needed (black-box); add a fault to the reference
  server to prove a new check has teeth.
