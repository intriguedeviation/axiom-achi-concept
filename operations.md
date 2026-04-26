# client-engine: Operations Reference
# Axiom Protocol - client-engine aspect
# Achi Domain — Rust/WASM Implementation

## Prerequisites

The only requirement for all roles is [Docker Desktop](https://www.docker.com/products/docker-desktop/).
No Rust, no Cargo, no development toolchain required.

---

## Stakeholders

You are here to verify that the system behaves as agreed. Nothing more is required of you technically.

### Start the environment

```bash
docker compose up -d --build
```

Run this once. The environment will stay running until you stop it.

### Verify the domain behaves correctly

```bash
docker exec achi-client-engine cargo test
```

All tests derive directly from the domain specification. A passing suite means
the implementation matches what was agreed. A failing test means something
does not — and that is important information, not a problem you caused.

### Stop the environment

```bash
docker compose down
```

### What you are seeing

Test output will reference behaviors by name — `placeToken`, `moveToken`,
`victoryAchieved`, and so on. These correspond directly to the behaviors
declared in the domain specification. If a behavior you expected to see
is absent from the test output, that is worth raising.

---

## QA & Business Analysts

You are here to validate that declared behaviors and their exception conditions
are correctly covered and that the test cases match the agreed domain.

### Start the environment

```bash
docker compose up -d --build
```

### Run the full behavioral suite

```bash
docker exec achi-client-engine cargo test
```

### Run tests for a specific behavior

```bash
docker exec achi-client-engine cargo test <behavior_name>
```

Examples:
```bash
docker exec achi-client-engine cargo test place_token
docker exec achi-client-engine cargo test move_token
docker exec achi-client-engine cargo test victory_achieved
```

### View continuous output

The environment watches for changes and runs automatically. To follow the output:

```bash
docker logs -f achi-client-engine
```

### What to look for

Each declared behavior in the domain specification should have corresponding
test coverage for:

- The happy path (preconditions met, postconditions verified)
- Each named exception condition

If a declared exception has no corresponding test, that is a coverage gap
worth raising. If a test exists for something not declared in the specification,
that is a domain integrity concern worth raising.

---

## SDETs

You are here to ensure behavioral coverage is complete, traceable to the
specification, and executable in any environment consistently.

### Start the environment

```bash
docker compose up -d --build
```

### Full suite

```bash
docker exec achi-client-engine cargo test
```

### Clippy — linting with warnings as errors

```bash
docker exec achi-client-engine cargo clippy -- -D warnings
```

### Specific behavior

```bash
docker exec achi-client-engine cargo test <behavior_name>
```

### WASM build verification

```bash
docker exec achi-client-engine wasm-pack build --target web --out-dir /app/build
```

### Coverage expectations

Every behavior declared in the domain specification maps to a Gherkin feature
in `./aspects/testing/features`. Every Gherkin feature maps to a Cucumber
scenario in `./tests`. Coverage is considered complete when:

- Every declared behavior has a passing scenario for its happy path
- Every declared exception condition has a passing scenario for its failure path
- No scenario exists for behavior or exceptions not declared in the specification

Invented test coverage — scenarios not traceable to a specification declaration
— is a domain integrity violation under the Axiom Protocol.

### Continuous watch

The environment runs Clippy and the full test suite automatically on source
changes. To observe:

```bash
docker logs -f achi-client-engine
```

### Interactive shell (diagnostic only)

```bash
docker exec -it achi-client-engine bash
```

---

## AI Partner

Operating within the `client-engine` aspect under the Axiom Protocol.
The domain specification at `./specs/specification.yml` is the only
canonical source of domain truth. The following operations are available
within the container context.

### Operational sequence

Per the `client-engine` guardrail, operations follow: **behavior → fulfillment → verification**

1. Reason from the declared behavioral contract in the specification
2. Produce the implementation that satisfies the contract within declared constraints
3. Verify correctness against the behavioral test suite

### Available operations

#### Lint
```bash
docker exec achi-client-engine cargo clippy -- -D warnings
```

#### Build
```bash
docker exec achi-client-engine cargo build
```

#### Build — release
```bash
docker exec achi-client-engine cargo build --release
```

#### Test — full suite
```bash
docker exec achi-client-engine cargo test
```

#### Test — specific behavior
```bash
docker exec achi-client-engine cargo test <behavior_name>
```

#### WASM — development build
```bash
docker exec achi-client-engine wasm-pack build --target web --out-dir /app/build
```

#### WASM — release build
```bash
docker exec achi-client-engine wasm-pack build --target web --out-dir /app/build --release
```

### Conformance reminders

- The specification is the source of truth for structure and behavior
- Generated code must not contradict or extend specification semantics
- Warnings are errors — Clippy must pass clean before fulfillment is complete
- No domain logic may be invented outside what the specification declares
- If a specification ambiguity is encountered, surface it — do not resolve it unilaterally

---

## CI/CD Pipeline

The container environment is identical to the local environment by design.
No pipeline-specific configuration is required beyond the following.

### Pipeline sequence

```bash
# 1. Build the environment
docker compose up -d --build

# 2. Lint — fail pipeline on any warning
docker exec achi-client-engine cargo clippy -- -D warnings

# 3. Behavioral suite — fail pipeline on any failure
docker exec achi-client-engine cargo test

# 4. WASM package — fail pipeline on build failure
docker exec achi-client-engine wasm-pack build --target web --out-dir /app/build --release

# 5. Teardown
docker compose down
```

### Pipeline guarantees

A passing pipeline means:
- The implementation is free of Clippy warnings
- All declared behaviors pass their behavioral tests
- The WASM package builds successfully for the target environment

A failing pipeline means one of those conditions is not met.
The failure is the signal. The specification is the arbiter.