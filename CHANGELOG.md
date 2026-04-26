# 2026-04-26 (2cc8d24)

## Added

- Added `client-ui` LLM environment guardrails for JavaScript, React, CSS, and container-based operations.
- Added playable board interactions that call the `client-engine` domain behaviors from the UI.
- Added keyboard-accessible board position controls with ARIA labels.
- Added live gameplay status messaging for turns, selections, domain errors, phase changes, and victory.
- Added visual states for occupied, active, selected, and locked board positions.

## Changed

- Updated `App` to preserve a single WASM `Grid` instance and route gameplay through `placeToken`, `moveToken`, `changeGridPhase`, and `victoryAchieved`.
- Refactored the board component into documented geometry helpers and hook-based rendering.
- Updated the header reset action to recreate the domain game state.
- Moved board presentation into CSS custom properties, media queries, and container queries.
- Updated `client-ui` ignore rules to exclude `.vite` cache output.

## Removed

- Removed board-only visual behavior that was not connected to the domain engine.

# 2026-04-26 (11ca94f)

## Added

- Added a WebGL-powered `NebulaBackground` React component for the `client-ui` aspect.
- Added an animated background layer behind the existing Achi header and board layout.
- Added CSS fallback nebula styling for environments where WebGL is unavailable.

## Changed

- Refined the board rendering with placement nodes, a subtle glow filter, and responsive SVG sizing.
- Updated the `client-ui` application shell so content renders above the nebula canvas.
- Updated default player labels used by the client UI.
- Updated Vite configuration to use `defineConfig`.
- Updated `client-ui` ignore rules to exclude generated `dist` output.

## Removed

- Removed the incompatible `vite-plugin-top-level-await` transform from the client UI build.
