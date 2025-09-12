# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Vue 3 + Vite frontend (entry `main.ts`, root `App.vue`).
- `public/`: Static assets served by Vite.
- `src-tauri/`: Rust + Tauri backend.
  - `src/`: `lib.rs` (commands, setup), `main.rs` (entry), modules `apis/` and `database/`.
  - `tauri.conf.json`: Dev/build hooks (uses Deno tasks) and dev URL `http://127.0.0.1:8080`.
  - `Cargo.toml`, `build.rs`.
- `db/`: Runtime-created SQLite files (git-ignored).

## Build, Test, and Development Commands
- `cargo tauri dev`: Run the desktop app in dev (spawns Vite via `deno task dev`).
- `cargo tauri build`: Build production binaries/installer.
- `deno task dev`: Frontend-only dev server on port 8080.
- `deno task build`: Build frontend to `dist/` (used by Tauri build).

## Coding Style & Naming Conventions
- Rust: Format with `cargo fmt`; lint with `cargo clippy` (prefer zero warnings).
  - `snake_case` for functions/variables/modules; `PascalCase` for types; `SCREAMING_SNAKE_CASE` for constants.
- Vue/TS: 2-space indent; `PascalCase` component files; `camelCase` variables/functions; kebab-case directories.
- Keep Tauri command names explicit and register them in `tauri::generate_handler![...]` in `src-tauri/src/lib.rs`.

## Testing Guidelines
- Currently no formal tests.
- Rust: Add unit tests with `#[cfg(test)]` inside modules or integration tests under `src-tauri/tests/`; run with `cargo test`.
- Frontend: If adding tests, prefer Vitest; place `*.spec.ts` next to source in `src/`.

## Commit & Pull Request Guidelines
- Use Conventional Commits (seen in history): `feat: ...`, `fix: ...`, `chore: ...`.
- PRs: clear description, linked issues, screenshots for UI changes, and brief test notes. Keep diffs focused and update docs when behavior changes.
- Branches: `feature/<short-topic>` or `fix/<issue-id>`.

## Security & Configuration Tips
- Prereqs (Windows): WebView2 Runtime, MSVC Build Tools, Rust (stable), Tauri CLI, Deno.
- Database files in `db/` are generated; do not commit. Add new migrations via the `database` module and register them in `lib.rs`.
- Dev server binds `127.0.0.1:8080`; if you change ports, update both `vite.config.ts` and `tauri.conf.json`.
