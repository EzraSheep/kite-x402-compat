Kite x402 Compatibility Test Kit

Product Specification & Coding Agent Development Instructions

Project: kite-x402-compat
Language: Rust
License: Apache-2.0
Protocol Target: x402 V2 / HTTP Transport
Initial Release: V0.1
Project Type: CLI + Reusable Rust Testing Library

1. Project Overview

1.1 Background

x402 is an open payment protocol that enables HTTP resources to require payment before returning protected content.

The ecosystem already has implementations in different languages and frameworks, including:

Rust / Axum

TypeScript / Hono

TypeScript / Express

Python

Go

However, different implementations may have inconsistent behavior, incomplete protocol support, or subtle compatibility issues.

Examples include:

Incorrect HTTP 402 responses.

Invalid PAYMENT-REQUIRED encoding.

Incorrect PaymentRequirements parsing.

Multiple payment options not being handled correctly.

Incorrect matching between PaymentPayload.accepted and advertised requirements.

Incorrect payment lifecycle ordering.

Inconsistent settlement error handling.

Missing or incorrect PAYMENT-RESPONSE headers.

Differences between framework implementations.

The official x402 repository provides reference implementations, protocol specifications, SDK tests, and integration examples.

Third-party tools such as atsp-protocol/x402-conformance already cover substantial portions of resource-server and facilitator testing.

This project must not simply duplicate those existing tools.

1.2 Project Goal

Build an independent Rust-based x402 protocol compatibility testing framework.

The framework should allow developers to test an x402 implementation through its public interfaces without requiring access to its internal source code.

The primary use case is:

Given an HTTP endpoint, automatically inspect its x402 implementation, execute applicable protocol tests, identify compatibility problems, and generate a structured report.

The project should eventually support testing:

x402 Resource Servers.

x402 Facilitators.

x402 Client implementations.

Different x402 framework adapters.

Kite ecosystem-specific implementations.

1.3 Core Principles

Protocol first

The official x402 specification is the source of truth.

Never invent protocol fields or change protocol semantics for implementation convenience.

Implementation independence

Tests must not depend on the internal implementation of the target service.

Use public HTTP interfaces and documented protocol behavior.

Safe by default

The default inspection mode must not sign transactions, spend funds, or submit blockchain transactions.

Active payment tests must use an explicitly configured mock facilitator or a dedicated test environment.

Reproducibility

Every test must have:

A stable test identifier.

A clear description.

Preconditions.

Input data.

Expected behavior.

Actual behavior.

A machine-readable result.

Extensibility

New payment schemes, networks, protocol versions, and framework adapters must be addable without redesigning the entire project.

2. Research Phase: Mandatory Before Implementation

Before writing production code, inspect the current official x402 repository and existing testing tools.

2.1 Official References

Read the following resources:

https://github.com/x402-foundation/x402

https://github.com/x402-foundation/x402/blob/main/specs/x402-specification-v2.md

https://github.com/x402-foundation/x402/tree/main/specs/transports-v2

https://github.com/x402-foundation/x402/tree/main/specs/schemes

https://github.com/x402-foundation/x402/tree/main/specs/extensions

https://github.com/x402-foundation/x402/tree/main/e2e

https://github.com/x402-foundation/x402/tree/main/typescript

Pay particular attention to:

PaymentRequired

PaymentRequirements

PaymentPayload

SettlementResponse

Facilitator API

Payment flows

Extensions

HTTP transport

Error handling

Replay protection

Do not assume that the current V2 specification will remain unchanged.

Record the exact upstream commit or release used as the baseline.

2.2 Existing Testing Tools

Inspect:

https://github.com/atsp-protocol/x402-conformance

Study:

Existing test coverage.

Test case identifiers.

Facilitator mock implementation.

Resource server probes.

Fault injection.

CLI interface.

JSON reporting.

GitHub Actions integration.

Identify which capabilities are already implemented and which can be reused as external tools or compatibility references.

Do not copy its implementation.

Also inspect the official SDK test suites and determine which protocol test cases are already covered by upstream tests.

2.3 Research Deliverables

Before implementation, create:

docs/RESEARCH.md

docs/PROTOCOL-COVERAGE.md

docs/ARCHITECTURE.md

RESEARCH.md must describe the existing ecosystem and competing tools.

PROTOCOL-COVERAGE.md must contain a test coverage matrix with:

Feature.

Official specification reference.

Existing upstream coverage.

Existing third-party coverage.

Planned implementation.

Priority.

Known limitations.

Do not claim that a feature is missing from existing tools without verifying their current source code.

The implementation scope must be adjusted based on the research findings.

3. Protocol Scope

3.1 Initial Protocol Support

The first release targets:

x402 V2.

HTTP transport.

Resource Server inspection.

Facilitator inspection.

Deterministic mock-based payment lifecycle tests.

V1 support is not required for the initial release.

However, the architecture must not prevent future V1 support.

Unsupported versions must be reported explicitly rather than silently interpreted as V2.

3.2 HTTP Headers

Implement correct parsing and validation for:

Header

Direction

Purpose

PAYMENT-REQUIRED

Server → Client

Payment requirements

PAYMENT-SIGNATURE

Client → Server

Payment payload

PAYMENT-RESPONSE

Server → Client

Settlement response

The V2 HTTP transport uses Base64-encoded JSON for these protocol objects.

The implementation must:

Decode Base64 safely.

Parse JSON.

Validate structure.

Distinguish malformed Base64 from malformed JSON.

Handle missing headers.

Handle empty headers.

Handle invalid UTF-8.

Handle oversized headers.

Preserve useful error information.

Never assume that a successful JSON parse means the protocol object is valid.

3.3 Core Data Structures

Implement strongly typed Rust representations for:

ResourceInfo

PaymentRequired

PaymentRequirements

PaymentPayload

SettlementResponse

Extensions

Use Serde.

Unknown fields should be handled according to the official protocol's forward-compatibility requirements.

Do not globally reject every unknown field.

Validation must distinguish required fields, optional fields, and scheme-specific fields.

3.4 Payment Requirements

Validate the following fields:

scheme

network

amount

asset

payTo

maxTimeoutSeconds

extra

Check:

Required fields.

Correct JSON types.

Valid CAIP-2 network identifiers.

Amount representation.

Positive and reasonable timeout values.

Scheme-specific required fields.

Duplicate or conflicting options where applicable.

Do not assume every network uses EVM addresses.

Do not implement chain-specific address validation as a universal rule.

3.5 Payment Payload

Validate:

x402Version

resource

accepted

payload

extensions

The accepted field represents one selected PaymentRequirements option.

The testing framework must verify that the selected option is compatible with the server's advertised accepts list.

Scheme-specific payload validation must be extensible.

4. Core Feature A: Resource Server Inspector

Command:

kite-x402-compat resource <URL>

Example:

kite-x402-compat resource https://example.com/api/data

This command performs read-only inspection.

4.1 Initial Request

Send an unpaid HTTP request to the specified endpoint.

Support configurable HTTP methods, with GET as the default.

Inspect:

HTTP status.

Response headers.

Response body.

Response duration.

Redirect behavior.

Expected behavior for a protected resource is an HTTP 402 response with a valid V2 PAYMENT-REQUIRED header.

4.2 Payment Challenge Validation

Validate:

HTTP status is 402.

PAYMENT-REQUIRED exists.

The header contains valid Base64.

Decoded JSON is valid.

x402Version is supported.

resource exists.

accepts exists and is non-empty.

Every advertised option contains required fields.

Network identifiers use valid CAIP-2 syntax.

Amounts use atomic token units.

Scheme-specific requirements are present.

Return individual test results.

4.3 Multiple Payment Options

This is one of the primary project differentiators.

The framework must support endpoints advertising multiple payment options.

For example:

USDC on Kite Testnet.

USDT on another supported network.

Different prices for different assets.

Different payment schemes.

Different payment flow configurations.

Test:

Multiple accepts entries.

Different networks.

Different assets.

Different amounts.

Duplicate options.

Ambiguous options.

Invalid entries.

Empty accepts.

Options with different scheme configurations.

Do not assume that the first accepts entry is always selected.

The framework must distinguish between:

A structurally valid payment option.

A payment option supported by the implementation.

A payment option that can actually be verified and settled.

4.4 Payment Option Matching Tests

Implement a dedicated matching test suite.

Verify that a server correctly handles the client's selected PaymentRequirements.

Test cases must include:

Exact matching option.

Matching option at the beginning of accepts.

Matching option at the end of accepts.

Matching option in the middle of accepts.

Unknown network.

Unknown asset.

Incorrect scheme.

Incorrect amount.

Incorrect payTo.

Incorrect timeout.

Modified extra fields.

Multiple potentially ambiguous options.

Duplicate advertised options.

The comparison rules must follow the official x402 protocol and scheme specifications.

Do not invent a universal matching algorithm based on arbitrary JSON equality.

In particular, do not normalize or ignore fields unless the protocol defines that behavior.

Amounts must be compared using exact integer or decimal-string semantics as appropriate. Never use floating-point arithmetic for token amounts.

4.5 Report

Each check must produce:

Test ID.

Test name.

Status.

Expected value.

Actual value.

Explanation.

Specification reference.

5. Core Feature B: Facilitator Inspector

Command:

kite-x402-compat facilitator <URL>

Inspect the facilitator's public API.

5.1 GET /supported

Check:

HTTP response status.

JSON parsing.

Supported protocol versions.

Supported schemes.

Supported networks.

Extensions.

Signers.

Network identifier format.

Compare advertised capabilities against the selected test profile.

Do not assume every facilitator must support every network or scheme.

An unsupported capability should be reported as unsupported, not automatically as a protocol failure.

5.2 POST /verify

The framework should support verification testing using synthetic fixtures and a mock facilitator.

Validate:

Request schema.

Response schema.

Successful verification.

Failed verification.

Invalid payload.

Unsupported network.

Unsupported scheme.

Timeout.

Malformed response.

Verify that /verify is read-only.

Do not send arbitrary invalid payloads to production facilitators unless the user explicitly enables that test.

5.3 POST /settle

Test:

Successful settlement response.

Failed settlement response.

Insufficient funds.

Invalid transaction.

Timeout.

Malformed JSON.

Server error.

settlement_pending.

Missing transaction information where required.

The tool must not submit real transactions by default.

6. Core Feature C: Payment Lifecycle Testing

This feature is intended to test actual Resource Server behavior using a controlled mock facilitator.

The default x402 authorization flow is:

Verify → Resource → Settle → Respond

However, the protocol also defines upfront and escrow payment flows.

The test framework must be payment-flow-aware.

6.1 Authorization Flow

Verify that:

An unpaid request returns HTTP 402.

A valid payment payload is accepted.

Facilitator verification occurs before resource execution.

The resource handler executes only after successful verification.

Settlement occurs after successful resource execution.

Successful settlement produces an appropriate response.

Settlement failure is not reported as successful payment.

6.2 Upfront Flow

Verify that the server performs the required settlement before resource execution.

Do not require a separate /verify call when the selected flow does not specify one.

6.3 Escrow Flow

Verify the documented sequence:

Initial settlement.

Resource execution.

Final settlement.

The expected behavior must follow the relevant scheme and mechanism specifications.

6.4 Failure Injection

Create a configurable mock facilitator that can simulate:

Verification rejection.

Settlement rejection.

Connection timeout.

Connection reset.

HTTP 500.

HTTP 502.

HTTP 503.

Invalid JSON.

Truncated response.

Delayed response.

settlement_pending.

Repeated requests.

Duplicate payment identifiers.

Record the number and order of facilitator calls.

Verify that the protected resource is not executed when the required pre-resource payment check fails.

For each payment flow, test its specific expected ordering.

6.5 Duplicate Payment Tests

Where the relevant scheme and implementation provide replay protection or idempotency guarantees, test:

Repeated identical payment payloads.

Reused payment identifiers.

Concurrent requests.

Multiple application instances where supported.

Do not impose implementation-specific database requirements on the protocol.

7. Core Feature D: HTTP Interoperability

Test HTTP behavior independently from payment scheme logic.

Include:

GET.

POST.

HEAD.

OPTIONS.

HTTP redirects.

Content-Type handling.

Header casing.

Missing headers.

Duplicate headers where observable.

Large headers.

Invalid Base64.

Malformed JSON.

7.1 CORS

Add browser-oriented CORS checks.

Inspect whether:

PAYMENT-REQUIRED is exposed.

PAYMENT-RESPONSE is exposed.

PAYMENT-SIGNATURE is allowed.

OPTIONS preflight works without payment.

Existing CORS headers are preserved.

Vary: Origin is handled appropriately.

Follow the official HTTP transport specification.

7.2 Cache Safety

Inspect whether paid responses may be cached in ways that expose protected content to unpaid clients.

Report potentially unsafe cache behavior.

Distinguish security recommendations from mandatory protocol violations.

8. Core Feature E: Kite Network Profile

Provide a dedicated Kite ecosystem test profile.

The profile must be configurable and must use authoritative network identifiers and token information.

Do not hardcode unverified network or token addresses.

The initial profile should support:

Kite Testnet.

Kite-supported payment assets.

Kite-supported schemes.

Relevant facilitator capabilities.

The tool should be able to verify whether an advertised payment option corresponds to a configured Kite network and asset.

Support external configuration:

profiles/kite-testnet.toml

Example configuration fields:

Profile name.

Network identifier.

Supported schemes.

Token identifiers.

Token decimals.

Facilitator endpoint.

Optional RPC endpoint.

Do not make an RPC endpoint mandatory for read-only protocol checks.

The Kite profile must clearly distinguish:

Protocol validity.

Network support.

Asset configuration validity.

Actual chain connectivity.

A valid x402 response does not automatically prove that a token or payment network is operational.

9. Architecture

Use a modular Rust workspace.

Suggested initial structure:

kite-x402-compat/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── CHANGELOG.md
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
├── crates/
│   ├── x402-core/
│   │   ├── src/
│   │   └── tests/
│   ├── x402-http/
│   │   ├── src/
│   │   └── tests/
│   ├── x402-testkit/
│   │   ├── src/
│   │   └── tests/
│   ├── x402-report/
│   │   ├── src/
│   │   └── tests/
│   └── x402-cli/
│       ├── src/
│       └── tests/
├── fixtures/
│   ├── valid/
│   ├── invalid/
│   ├── multiple-options/
│   └── facilitator/
├── profiles/
│   └── kite-testnet.toml
├── docs/
│   ├── RESEARCH.md
│   ├── PROTOCOL-COVERAGE.md
│   ├── ARCHITECTURE.md
│   ├── TESTING.md
│   └── DEVELOPMENT.md
└── examples/

The final structure may be adjusted based on actual complexity.

Avoid unnecessary crates and overengineering.

9.1 Crate Responsibilities

x402-core

Contains:

Protocol data structures.

Protocol version types.

Schema validation.

Payment requirement validation.

Payment matching.

Scheme abstractions.

Must not depend on HTTP clients or CLI code.

x402-http

Contains:

HTTP header parsing.

Base64 encoding and decoding.

HTTP request and response handling.

Resource Server inspection.

Facilitator HTTP client.

x402-testkit

Contains:

Test case definitions.

Test execution.

Mock facilitator.

Fault injection.

Payment lifecycle tests.

Test fixtures.

x402-report

Contains:

Test result structures.

Summary generation.

JSON output.

Human-readable terminal output.

x402-cli

Contains:

CLI argument parsing.

Configuration loading.

Command orchestration.

Exit codes.

10. CLI Design

Use Clap.

Proposed commands:

resource

Inspect a protected HTTP resource.

kite-x402-compat resource <URL>

Options:

--method

--timeout

--output

--profile

--format

--verbose

facilitator

Inspect a facilitator.

kite-x402-compat facilitator <URL>

inspect

Decode and inspect a saved x402 protocol object.

kite-x402-compat inspect payment-required.json

Support inspecting:

PaymentRequired JSON.

PaymentPayload JSON.

SettlementResponse JSON.

Base64-encoded header values.

probe

Execute controlled payment lifecycle tests.

kite-x402-compat probe --resource <URL> --facilitator <URL>

This command must require explicit opt-in and a controlled test environment.

self-test

Run internal test fixtures and mock facilitator tests.

kite-x402-compat self-test

report

Convert saved test results into another output format.

kite-x402-compat report results.json --format html

HTML output is optional for the initial release.

11. Test Case Architecture

All tests must be data-driven where practical.

A test case should contain:

ID.

Name.

Category.

Description.

Protocol version.

Preconditions.

Input fixture.

Expected result.

Severity.

Specification reference.

Suggested categories:

HTTP transport.

Header encoding.

Schema validation.

Payment requirements.

Multiple payment options.

Payment payload.

Facilitator.

Payment lifecycle.

Security.

Kite network.

Example test IDs:

HTTP-001: Unpaid request returns 402.

HTTP-002: PAYMENT-REQUIRED exists.

HDR-001: Valid Base64 decoding.

HDR-002: Invalid Base64 rejection.

SCH-001: PaymentRequired required fields.

SCH-002: Empty accepts rejection.

OPT-001: Multiple options accepted.

OPT-002: Correct option matching.

OPT-003: Unknown option rejected.

OPT-004: Incorrect amount rejected.

FAC-001: Supported endpoint schema.

PAY-001: Verify failure blocks resource execution.

PAY-002: Settlement failure is not reported as success.

PAY-003: Correct payment flow ordering.

CORS-001: Required headers exposed.

KITE-001: Kite network profile validation.

These are initial identifiers, not an exhaustive list.

12. Test Results and Reporting

Support the following statuses:

PASS

FAIL

SKIP

WARN

ERROR

Their meanings:

PASS: The test passed.

FAIL: The observed behavior violates the applicable requirement.

SKIP: The test could not be performed because its prerequisites were unavailable.

WARN: Potential issue or recommendation that is not a confirmed protocol violation.

ERROR: The test could not complete because of an infrastructure or execution error.

Do not confuse unsupported capabilities with protocol failures.

12.1 JSON Report

Generate a stable JSON report.

Example:

{
"tool": {
"name": "kite-x402-compat",
"version": "0.1.0"
},
"target": {
"url": "https://example.com/api/data",
"type": "resource"
},
"protocol": {
"version": 2,
"transport": "http"
},
"summary": {
"total": 10,
"passed": 8,
"failed": 1,
"warnings": 1,
"skipped": 0,
"errors": 0
},
"results": [
{
"id": "HTTP-001",
"name": "Unpaid request returns HTTP 402",
"status": "PASS",
"expected": 402,
"actual": 402,
"message": "Expected payment challenge received."
}
]
}

The real report must contain timestamps and sufficient metadata for reproducibility.

Avoid storing payment secrets, signatures, private keys, or sensitive authorization payloads in reports.

12.2 Exit Codes

Define stable exit codes:

0: All applicable tests passed.

1: One or more tests failed.

2: Configuration or usage error.

3: Test execution or infrastructure error.

Warnings alone should not cause failure by default.

Allow an optional strict mode.

13. Security Requirements

Security is particularly important because the tool interacts with arbitrary HTTP endpoints.

Implement:

Connection and request timeouts.

Response size limits.

Header size limits.

Safe Base64 decoding.

JSON nesting and size limits where practical.

No automatic transaction signing.

No private key storage.

No automatic mainnet payment.

No credential leakage in logs.

Safe handling of malicious server responses.

Prevent SSRF risks:

Reject localhost and private-network targets by default for remote inspection.

Resolve and validate DNS addresses.

Revalidate destinations after redirects.

Reject redirects to forbidden addresses.

Provide an explicit local-testing mode for localhost.

Do not allow active probes against arbitrary third-party facilitators without explicit authorization.

Do not automatically follow redirects without applying destination validation.

Mock facilitator mode must bind to loopback by default.

14. Technology Stack

Use Rust stable.

Recommended libraries:

tokio: asynchronous runtime.

reqwest: HTTP client.

serde: serialization.

serde_json: JSON handling.

clap: CLI.

anyhow: application error handling.

thiserror: library errors.

base64: Base64 encoding.

tracing: structured logging.

tracing-subscriber: logging output.

toml: profile configuration.

url: URL parsing.

tempfile: test temporary files.

Use additional dependencies only when justified.

Check current stable crate versions and MSRV compatibility before selecting dependencies.

Avoid introducing blockchain SDKs into the core crate unless a specific feature requires them.

15. Development Phases

Implement incrementally.

Do not attempt to build every feature in one large implementation.

Phase 0: Research and Design

Deliver:

Research report.

Existing tool comparison.

Protocol coverage matrix.

Architecture document.

Initial test plan.

Acceptance:

Existing overlapping functionality is identified.

Unique project scope is clearly established.

Official specification references are recorded.

No unverified claims about protocol behavior.

Phase 1: Protocol Core

Implement:

Core V2 data structures.

HTTP header codecs.

PaymentRequired validation.

PaymentPayload validation.

SettlementResponse validation.

CAIP-2 syntax validation.

Multiple payment option representation.

Payment option matching tests.

Acceptance:

Valid fixtures pass.

Invalid fixtures fail correctly.

Multiple options are supported.

Amount comparisons do not use floating-point arithmetic.

Unit tests cover boundary cases.

Phase 2: Resource Inspector

Implement:

HTTP client.

Unpaid request.

HTTP 402 inspection.

PAYMENT-REQUIRED decoding.

Schema validation.

Multiple options analysis.

Human-readable report.

JSON report.

Acceptance:

Can inspect a local x402 service.

Can inspect a remote service.

Correctly identifies malformed challenges.

Does not perform payment.

Phase 3: Facilitator Inspector

Implement:

GET /supported.

Capability validation.

Scheme and network checks.

Facilitator response validation.

Acceptance:

Can inspect a local mock facilitator.

Can inspect a public facilitator in read-only mode.

Correctly distinguishes unsupported features from invalid responses.

Phase 4: Mock Facilitator and Lifecycle Testing

Implement:

Mock facilitator.

Configurable verification outcomes.

Configurable settlement outcomes.

Fault injection.

Request sequence recording.

Authorization flow testing.

Extensible upfront and escrow testing.

Acceptance:

Full lifecycle can be tested without blockchain transactions.

Failed verification blocks protected resource execution.

Settlement failures are handled correctly.

Flow-specific ordering is tested.

Phase 5: Kite Testnet Profile

Implement:

Kite profile configuration.

Network identifier validation.

Asset configuration validation.

Facilitator compatibility checks.

Documentation.

Acceptance:

Kite profile can be loaded.

Invalid network or asset configuration is detected.

No unverified addresses are hardcoded.

Phase 6: Release Preparation

Implement:

Documentation.

Examples.

GitHub Actions.

CLI installation instructions.

Versioning.

Changelog.

License.

Release build.

Acceptance:

A clean environment can build and run the project.

All tests pass.

Documentation is independently reproducible.

A complete demo is available.

16. Testing Requirements

Every new feature must have corresponding tests.

Required test categories:

Unit tests

Serialization.

Deserialization.

Header encoding.

Header decoding.

Schema validation.

Payment matching.

Error handling.

Configuration parsing.

Boundary tests

Empty fields.

Missing fields.

Invalid types.

Large headers.

Invalid Base64.

Invalid JSON.

Duplicate payment options.

Very large amounts.

Amount precision.

Unsupported networks.

Integration tests

Local HTTP resource server.

Mock facilitator.

Payment challenge.

Verification.

Resource execution.

Settlement.

Error responses.

Regression tests

Every discovered bug should receive a regression test.

Compatibility tests

Where practical, use official x402 fixtures and examples.

Do not copy third-party test implementation without checking its license.

Tests must run offline whenever possible.

17. Code Quality and CI

Before every commit or PR:

Run:

cargo fmt --all -- --check

cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo test --workspace --all-features

cargo build --workspace --release

New functionality and improvements must have corresponding tests.

CI must run:

Formatting checks.

Clippy.

Unit tests.

Integration tests.

Release build.

Use GitHub Actions.

Keep dependencies minimal and avoid unnecessary dependency upgrades.

Do not introduce unrelated code changes.

18. Documentation Requirements

The repository must contain:

README.md

Include:

Project introduction.

Problem statement.

Feature list.

Supported protocol versions.

Supported transports.

Installation.

Usage.

CLI examples.

Sample reports.

Security limitations.

Roadmap.

docs/ARCHITECTURE.md

Explain:

Crate responsibilities.

Module structure.

Data flow.

Test execution.

Extension points.

docs/PROTOCOL-COVERAGE.md

Document protocol coverage and upstream references.

docs/TESTING.md

Explain:

Test architecture.

Fixtures.

Mock facilitator.

Integration tests.

How to add new tests.

docs/DEVELOPMENT.md

Explain:

Rust toolchain.

Build commands.

Test commands.

Contribution guidelines.

CHANGELOG.md

Record version changes.

Include an Apache-2.0 LICENSE.

19. Definition of Done

The initial release is complete only when all of the following are satisfied:

A Rust CLI can be installed and executed.

It can inspect an x402 V2 HTTP resource.

It can decode and validate PAYMENT-REQUIRED.

It can analyze multiple payment options.

It can test payment option matching.

It can inspect a facilitator's /supported endpoint.

It can execute a mock payment lifecycle test.

It can simulate verification and settlement failures.

It can generate JSON reports.

It has automated unit and integration tests.

It has complete Markdown documentation.

It builds and runs in a clean environment.

It does not require real wallets or mainnet transactions for its default test suite.

The project must demonstrate real compatibility-testing functionality, not merely provide a collection of static JSON validators.

20. Coding Agent Execution Instructions

You are the lead Rust engineer responsible for delivering this project.

Follow these instructions strictly.

Step 1: Inspect Before Coding

Inspect the current working directory.

Determine whether an existing repository or implementation is present.

Do not overwrite existing user files or unrelated code.

Read the official x402 specifications and inspect existing test tools before designing the implementation.

Step 2: Research and Report

Create the research documents described in Phase 0.

Identify existing functionality that would be duplicated.

If the proposed feature is already comprehensively implemented by another tool, do not blindly reproduce it.

Instead, identify a concrete, independently useful improvement.

Step 3: Produce the Technical Design

Before implementation, document:

Project architecture.

Crate boundaries.

Protocol data structures.

Test case model.

CLI design.

Security model.

Initial acceptance criteria.

Step 4: Implement Incrementally

Implement Phase 1 first.

After each phase:

Run formatting.

Run Clippy.

Run tests.

Fix regressions.

Update documentation.

Do not mark incomplete functionality as implemented.

Step 5: Validate Independently

Test the resulting tool against:

A bundled mock resource server.

A mock facilitator.

At least one real x402 V2 implementation.

A Kite-compatible test environment when available.

Use the existing x402-conformance project as an external comparison or baseline where practical.

Document any differences.

Step 6: Maintain Engineering Quality

Use clear Rust abstractions.

Avoid unnecessary generic frameworks.

Avoid duplicated validation logic.

Use typed errors.

Keep protocol semantics separate from transport and CLI concerns.

Do not silently ignore failed tests.

Do not claim compatibility with networks or schemes that have not been tested.

Step 7: Final Delivery Report

At completion, provide:

Implemented features.

Architecture summary.

Project directory structure.

CLI usage examples.

Test results.

Build results.

Protocol coverage.

Known limitations.

Remaining TODOs.

Suggested next development phase.

Clearly distinguish completed functionality from planned functionality.

The goal is a small, reliable, extensible Rust project that provides real value to the Kite x402 ecosystem.

Do not optimize for the amount of generated code. Optimize for protocol correctness, reproducibility, maintainability, and demonstrable functionality.