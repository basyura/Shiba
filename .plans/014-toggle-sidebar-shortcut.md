# ctrl+l でサイドバーを開閉する

## 目的

`ctrl+l` のキーボードショートカットでプレビュー右側のサイドバーを開閉できるようにする。

## 修正案

- `src/config.rs` に `ToggleSideBar` アクションを追加し、デフォルトキーマップへ `ctrl+l` を割り当てる。
- `src/assets/default_config.yml` の `keymaps` に `ctrl+l: ToggleSideBar` を追加する。
- `ui/ipc.ts` の `KeyAction` に `ToggleSideBar` を追加する。
- `ui/reducer.ts` にサイドバー表示状態と切り替えアクションを追加する。
- `ui/keymaps.ts` で `ToggleSideBar` を UI 状態変更へ接続する。
- `ui/components/App.tsx` と `ui/components/Preview.tsx` でサイドバー表示状態を受け渡し、非表示時はサイドバーと区切り線を描画しない。

## 確認

- TypeScript の型チェックまたは既存 lint を実行する。
- Rust 側は `cargo test` または設定周辺の既存テストで確認する。

## 実施結果

- `ctrl+l: ToggleSideBar` をデフォルトキーマップに追加した。
- UI 状態としてサイドバーの表示状態を持ち、`ToggleSideBar` で開閉するようにした。
- サイドバー非表示時もリサイズ済み幅を保持できるよう、サイドバー本体はマウントしたまま非表示にした。
- 既存デフォルト設定から新しい `ctrl+l` 割り当てへアップグレードするテストを追加した。
- `npm run lint:tsc`、`cargo test config::tests`、`npm run lint:prettier`、`npm run lint:rustfmt`、`npm run lint:eslint` を実行し、すべて成功した。
