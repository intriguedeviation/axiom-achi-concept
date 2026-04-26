# 2026-04-26 (fc70ad9)

## Added

- Added a React/Vite `client-ui` aspect with application entry points, header, board rendering, stylesheet, package metadata, and Vite configuration.
- Added integration from the client UI to the generated `client-engine` WebAssembly package.
- Added generated `client-engine` WebAssembly package outputs under `aspects/client-engine/build`.
- Added Cucumber behavior coverage for the client engine under `aspects/client-engine/tests`.
- Added Docker Compose support for running the client UI alongside the client engine.
- Added an nginx proxy configuration in `docker/proxy/app.conf`.
- Added root stakeholder and developer setup documentation in `README.md`.

## Changed

- Updated `client-engine` WebAssembly bindings to current `wasm-bindgen` and `wasm-bindgen-test` versions.
- Reworked the Rust client engine from template code into a specification-aligned Achi domain engine.
- Updated `compose.yml` to mount the `aspects/client-engine` source paths and include `client-ui` and proxy services.
- Regenerated lockfiles and package outputs for the Rust and JavaScript aspects.

## Removed

- Removed placeholder-only client-engine behavior in favor of the implemented specification-driven engine.
