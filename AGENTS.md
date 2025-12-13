# Repository Guidelines

## Project Structure & Module Organization
- Crates in this repo are Rust libraries: `wp-connector-api/`, `wp-ctrl-api/`, `wp-data-model/`, `wp-enrich-api/`, `wp-parse-api/`.
- Each crate keeps code in `src/` with unit tests co-located via `#[cfg(test)]`. Some crates depend on siblings in the top-level `warp-flow-system` workspace (e.g., `../../wp-data-model`).
- No binaries here; these crates define traits, models, and helpers consumed by other services.

## Build, Test, and Development Commands
- From a crate directory (recommended when working only on that crate):
  - `cargo build` — compile the library
  - `cargo test` — run unit tests
  - `cargo doc --open` — build docs locally
- From the workspace root (warp-flow-system), target a crate by package name:
  - `cargo build -p wp-parse-api`
  - `cargo test  -p wp-data-model`
- Lint/format before committing:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -D warnings`

## Coding Style & Naming Conventions
- Rust 2021 edition; use `rustfmt` defaults (4-space indent, max width defaults).
- Types/traits: `CamelCase`; functions/modules: `snake_case`; constants: `SCREAMING_SNAKE_CASE`.
- Prefer returning the workspace error type over panicking; document public APIs with `///` and crate/module docs with `//!`.

## Testing Guidelines
- Unit tests live next to code (`mod tests { ... }`); name tests `test_*` for clarity.
- Async tests use `#[tokio::test]` where applicable.
- Optional fuzzing in `wp-data-model/fuzz/` with `cargo fuzz run <target>` (requires `cargo-fuzz`).

## Commit & Pull Request Guidelines
- Use Conventional Commits style; scope with the crate when helpful:
  - Example: `feat(wp-parse-api): add RawData parser for bytes input`
- PRs should include: a clear description, linked issues, tests for behavior changes, and any relevant screenshots/logs.
- Before opening a PR, ensure `cargo fmt`, `cargo clippy`, and `cargo test` pass for the affected crates (or targeted `-p` in the workspace).

## Security & Configuration Tips
- Do not commit secrets or sample keys; treat all inputs as untrusted.
- Avoid `unwrap`/`expect` in library code; return `Result` with meaningful errors.
- Keep cross-crate paths aligned with the workspace; prefer workspace versions/deps when available.
