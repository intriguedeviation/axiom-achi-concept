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
