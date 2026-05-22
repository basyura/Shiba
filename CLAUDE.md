# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Planning Workflow

Before making any edits, create a Japanese-language plan file in `.plans/` with sequential numbering (e.g., `001-fix-something.md`). Present the plan and wait for confirmation before editing files.

- Do not create a new plan file unless asked.
- If no plan file is specified for the session, ask which one to use rather than picking one automatically.

## Build Commands

```sh
npm install            # Install frontend/tooling dependencies
make build             # Debug binary + UI bundle
npm run watch          # Watch mode: TS bundle + tsc + cargo check
```

**macOS (Apple Silicon only — default unless specified):**
```sh
npm run release
cargo build --release --target=aarch64-apple-darwin
cp -r assets/Shiba.app Shiba.app
cp target/aarch64-apple-darwin/release/shiba Shiba.app/Contents/MacOS/shiba
```
`make Shiba.app` requires both x86_64 and aarch64 builds — avoid it unless a universal binary is explicitly requested.

**Windows (default: exe only, no MSI):**
```sh
./build_win.sh         # Produces target/release/shiba.exe
ls -lh target/release/shiba.exe  # Verify artifact
```

## Lint & Test

```sh
npm run lint           # All linters: clippy, rustfmt, tsc, prettier, eslint, stylelint
npm run lint:ui        # TS only: tsc + prettier + eslint + stylelint
npm run fix            # Auto-fix Rust and TS lint issues
cargo test             # Rust tests (also covered by npm test)
npm test               # cargo test + tests/ui/
```

Benchmarks live in `bench/benches/`, fuzz targets in `fuzz/`. Run focused tests with `cargo test <test_name>` before PRs touching core parsing or platform logic.

## Architecture

Shiba is a native desktop Markdown previewer. A **Rust binary** embeds a React/TypeScript UI rendered inside a platform WebView (via `wry` + `tao`). There is no Electron.

**Data flow:**
1. Rust watches the filesystem (`notify`) and parses Markdown (`pulldown-cmark`) into a `RenderTreeElem[]` JSON tree.
2. The tree is sent to the WebView via `window.postShibaMessageFromMain(msg)` (`MessageFromMain` discriminated union).
3. The React UI (`ui/`) converts the tree to React nodes and renders it.
4. UI events (search, navigation, dialogs) are sent back with `window.ipc.postMessage(JSON.stringify(msg))` (`MessageToMain` union).

**Asset embedding:** UI assets (HTML, JS bundle, CSS) are compiled into the binary via `include_bytes!` in `src/assets.rs`. The JS bundle is zstd-compressed at build time (`build.rs`) and decompressed at runtime. The build is two-phase: (1) `npm run bundle/release` compiles `ui/index.tsx` → `src/assets/bundle[.min].js`, (2) `cargo build` embeds the assets.

**Key source locations:**
- `src/shiba.rs` — central app struct; owns all state and handles events
- `src/renderer.rs` — `Renderer` trait + IPC message types
- `src/markdown/` — Markdown parser, sanitizer, full-text search
- `src/wry/` — WebView + event loop integration
- `src/config.rs` — YAML config loading and key action definitions
- `ui/ipc.ts` — authoritative IPC type definitions (both directions)
- `ui/dispatcher.ts` — routes `MessageFromMain` events to React state
- `ui/reducer.ts` — `useReducer`-based state management
- `ui/markdown.tsx` — converts `RenderTreeElem[]` to React nodes
- `src/assets/default_config.yml` — default user configuration

**Dependency injection:** `Shiba<R: Rendering, O: Opener, W: Watcher, D: Dialog>` uses generic type parameters so components can be swapped for testing (e.g., `NopWatcher` with `--no-watch`).

## Code Conventions

**TypeScript/React:**
- Named exports only — `import/no-default-export` is enforced.
- Strict TypeScript: `exactOptionalPropertyTypes`, `noUnusedLocals`, `noUnusedParameters`, `noImplicitReturns`.
- Prettier: 4-space indent, single quotes, trailing commas, 120-char line width (200 for CSS).
- Switch exhaustiveness is enforced by ESLint.

**Rust:**
- `dbg!`, `print!`, `println!`, `eprintln!` and undocumented `unsafe` blocks emit warnings (clippy lints in `lib.rs`).
- Format with `cargo fmt`, fix with `cargo clippy --fix`.
- IPC message types in Rust (`MessageToRenderer`) must be kept manually in sync with TypeScript (`MessageFromMain` in `ui/ipc.ts`).

## Commit Style

`type(scope): subject` 形式を優先する（例: `fix(config): ...`, `feat(ui): ...`）。複数ファイルにまたがる変更には短い本文を添える。UI の変更を含む PR にはスクリーンショットまたは GIF を添付する。
