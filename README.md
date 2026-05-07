# Axiom Achi

Axiom Achi is a specification-driven implementation of the Achi game domain.
The repository is organized around a single canonical domain specification and
one or more generated or maintained aspects that turn that specification into
code, documentation, and tests.

The current implemented aspect is a Rust/WebAssembly client engine. It exposes
the specification-defined Achi behaviors as a reusable rules engine that can be
called from Rust or JavaScript.

## Stakeholder Overview

This project is intended to keep game rules, implementation, and validation in
sync. The domain source of truth is [specs/achi.yml](specs/achi.yml);
aspect outputs must explain, implement, or verify that specification without
inventing additional domain behavior.

The specification currently defines:

- Two player sides: red and black.
- Player labels with validation.
- A nine-position grid.
- Four tokens per player.
- Two board phases: placement and movement.
- Eight victory alignments.
- Placement and movement records.
- Behavior-level exceptions for invalid turns, occupied positions, unset
  tokens, non-adjacent movement, same-position movement, wrong phase changes,
  and using the same player as both adversaries.

The implemented game behaviors are:

- `definePlayer`: creates a player with a side, label, and four unset tokens.
- `startGame`: creates a placement-phase grid for distinct red and black
  players.
- `placeToken`: places a player token onto an unoccupied position and advances
  the active player.
- `moveToken`: moves a token to an unoccupied adjacent position and advances
  the active player.
- `victoryAchieved`: evaluates whether a player has a victory alignment after
  all tokens are positioned.
- `changeGridPhase`: changes the grid from placement to movement after all
  tokens are positioned.

## Repository Layout

```text
.
├── guardrails.yml
├── LICENSE.md
├── README.md
├── specs/
│   └── achi.yml
└── aspects/
    └── client-engine/
        ├── guardrails.yml
        ├── Cargo.toml
        ├── src/
        │   ├── lib.rs
        │   └── utils.rs
        └── tests/
            ├── behavior.rs
            ├── web.rs
            └── features/
                └── achi.feature
```

Important files:

- [guardrails.yml](guardrails.yml): repository-level aspect and strictness configuration.
- [specs/achi.yml](specs/achi.yml): canonical domain model,
  behavior, and exception definition.
- [aspects/client-engine/guardrails.yml](aspects/client-engine/guardrails.yml): local
  guardrails for the Rust/WASM client engine.
- [aspects/client-engine/src/lib.rs](aspects/client-engine/src/lib.rs): Rust
  implementation of the domain behaviors.
- [aspects/client-engine/tests/features/achi.feature](aspects/client-engine/tests/features/achi.feature):
  Cucumber behavior coverage for the specification.

## Aspect Guardrails

The root `guardrails.yml` declares that the specification is the only canonical
source of domain truth. In practical terms:

- Code must align with the specification and local aspect rules.
- Tests must verify declared behavior and exceptions.
- Documentation must explain the domain, not reinterpret it.
- Aspect outputs must not add undeclared fields, constraints, or behaviors.

The `client-engine` aspect requires:

- Rust crate packaging with Cargo.
- Rust minimum level: `1.95`.
- Clippy linting.
- Warnings treated as errors.
- Behavior-style tests in `./tests`.
- Cucumber as the behavior testing framework.
- WebAssembly packaging with `wasm-pack`.
- Build output in `./build`.

Generated or local build output such as `aspects/client-engine/target` and
`aspects/client-engine/build` is ignored by the aspect configuration.

## Developer Environment

Install the following tools:

- Rust toolchain `1.95` or newer.
- Cargo, rustfmt, and Clippy.
- The `wasm32-unknown-unknown` Rust target.
- `wasm-pack`.

Recommended setup with `rustup`:

```sh
rustup toolchain install 1.95
rustup component add rustfmt clippy --toolchain 1.95
rustup target add wasm32-unknown-unknown --toolchain 1.95
cargo install wasm-pack
```

If you prefer to use your default Rust toolchain, make sure it is at least
`1.95`:

```sh
rustc --version
cargo --version
wasm-pack --version
```

## Build And Test

Run client-engine commands from the aspect directory:

```sh
cd aspects/client-engine
```

Format the code:

```sh
cargo fmt
```

Run the Rust and Cucumber behavior tests:

```sh
cargo test
```

Run Clippy with warnings treated as errors:

```sh
cargo clippy --all-targets -- -D warnings
```

Build the WebAssembly package:

```sh
wasm-pack build --out-dir build
```

The package output is written to:

```text
aspects/client-engine/build
```

## Testing Strategy

The client engine uses Cucumber feature scenarios to validate behavior against
the specification. The current feature coverage verifies player creation, game
startup, token placement, movement, phase changes, victory checks, and declared
domain exceptions.

The WebAssembly test file provides smoke coverage for exported bindings. Native
behavior coverage is in the Cucumber harness.

## Working With The Specification

When changing domain behavior, update the canonical specification first:

```text
specs/achi.yml
```

Then update the affected aspects:

1. Update implementation code in `aspects/client-engine/src`.
2. Update behavior scenarios in `aspects/client-engine/tests/features`.
3. Update step definitions in `aspects/client-engine/tests/behavior.rs` when
   new scenario language is introduced.
4. Run formatting, tests, Clippy, and the WebAssembly build.

Do not treat generated build output as source. Cargo and wasm-pack can recreate
`target` and `build` artifacts.

## License

This repository is licensed under the terms in [LICENSE.md](LICENSE.md).
