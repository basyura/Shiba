# Repository Guidelines

## 計画

- 修正を始める前に計画をマークダウンファイルで .plans フォルダ配下に生成してください。
- 計画のファイル名は連番とし、1つ目を 001 始まりとして修正にあった適切なファイル名としてください。
- 計画のフォーマットは `001-windows-makefile-conditional.md` を参照すること。
- 具体的なファイル編集をする前に、修正案を提示すること。

## Project Structure & Module Organization
- `src/`: Rust core (Markdown parsing, app logic, platform integration) with submodules like `src/markdown/` and `src/wry/`.
- `ui/`: React + TypeScript UI (`ui/components/`, `ui/style.css`, `ui/index.html`).
- `src/assets/`: Bundled UI assets and app resources, including `default_config.yml` and generated JS/CSS/HTML.
- `assets/`: Icons and platform packaging assets (macOS app, Windows MSI, Debian, etc.).
- `docs/`: End-user documentation and installation guides.
- `scripts/`, `bench/`, `fuzz/`: Build helpers, benchmarks, and fuzzing targets.

## Build, Test, and Development Commands
- `npm install`: Install frontend/tooling dependencies.
- `make build`: Build the debug binary and UI bundle.
- `make release`: Build the release binary with minified UI assets.
- `bash ./scripts/gen_macos_app.bash`: Generate `Shiba.app` after both arch builds exist.
- macOS app build note: `make Shiba.app` expects both `x86_64-apple-darwin` and `aarch64-apple-darwin` builds. If `cargo` comes from Homebrew, set `RUSTC` to the rustup toolchain so cross-arch builds find the stdlibs (example below).
- `npm run watch`: Watch TS, bundling, and Rust checks for active development.
- `npm run lint`: Run Rust and UI linters/formatters (clippy, rustfmt, tsc, prettier, eslint, stylelint).
- `cargo test` (or `npm test`): Run Rust tests.

Example for universal macOS build:
```sh
RUSTC="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc" \
  "$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo" build \
  --release --target x86_64-apple-darwin
```

## Coding Style & Naming Conventions
- Rust: Format with `cargo fmt` and fix warnings with `cargo clippy`.
- TypeScript/React: Enforced by `eslint`, `prettier`, `stylelint`, and `tsc --noEmit`.
- Naming: Rust modules in `snake_case`, React components in `PascalCase` under `ui/components/`.

## Testing Guidelines
- Add Rust unit/integration tests alongside modules in `src/` or in a `tests/` directory if needed.
- Benchmarks live in `bench/benches`, fuzz targets in `fuzz/`.
- Prefer running `cargo test` before PRs; include focused tests when touching core parsing or platform logic.

## Commit & Pull Request Guidelines
- Recent commits often follow Conventional Commits like `fix(config): ...`, though some are simple `update ...` messages. Prefer `type(scope): subject` with a short body for multi-file changes.
- PRs should include a concise description, test results, and screenshots/GIFs for UI changes.
- Link relevant issues or docs when behavior changes or config updates are involved.

## Configuration & Packaging Notes
- Default configuration lives at `src/assets/default_config.yml`.
- Packaging targets include macOS app/DMG, Windows MSI, and Debian packages via `make` targets.
