# Vim 風スクロールキーの追加計画

## 目的

- `G` でプレビューを一番下へスクロールできるようにする。
- `gg` でプレビューを一番上へスクロールできるようにする。
- `ctrl+n` で次のセクションへスクロールできるようにする。
- `ctrl+p` で前のセクションへスクロールできるようにする。
- 一番下にいる状態で `ctrl+n` を押しても先頭へ戻らないようにする。
- スクロール時のサイドバーのアクティブセクション追従を速くする。
- `F12` で DevTools を開けるようにする。

## 修正方針

- 既存の `ScrollBottom` / `ScrollTop` アクションを再利用する。
- 既存の `ScrollNextSection` / `ScrollPrevSection` アクションを再利用する。
- 既定キーマップ定義に `G: ScrollBottom` と `g g: ScrollTop` を追加する。
- 既定キーマップ定義に `ctrl+n: ScrollNextSection` と `ctrl+p: ScrollPrevSection` を追加する。
- `ScrollNextSection` は次の見出しがない場合に何もしないようにする。
- サイドバーのアクティブセクション更新は debounce をやめ、`requestAnimationFrame` で最大1フレームに1回だけ実行する。
- 既定キーマップ定義に `f12: OpenDevTools` を追加し、UI から main へ IPC して WebView の DevTools を開く。
- debug/release や default features の有無に関係なく `F12` が機能するよう、wry 依存で `devtools` feature を常時有効にする。
- `open_devtools` IPC 名を Rust 側で明示的に受け付け、IPC parse 失敗時にも panic しないようにする。
- Mousetrap の連続キー指定は空白区切りのため、`gg` ではなく `g g` を使う。
- 既存のユーザー設定ファイルでも新しい既定キーが使えるよう、欠けている既定キーマップを補完する。
- デフォルト設定ファイルにも同じ割り当てを追加し、生成設定との整合性を保つ。

## 対象ファイル

- `src/config.rs`
- `src/assets/default_config.yml`
- `ui/keymaps.ts`
- `ui/components/Article.tsx`
- `ui/ipc.ts`
- `src/renderer.rs`
- `src/shiba.rs`
- `src/wry/webview.rs`
- `Cargo.toml`

## 確認

- TypeScript 側の追加実装は不要な想定。
- スクロール追従改善は `requestAnimationFrame` ベースの更新頻度制御にする。
- DevTools は常時有効にする。`F12` は常に WebView の DevTools を開く。
- Rust/設定の整合性確認として、可能なら関連テストまたは `cargo test` を実行する。
