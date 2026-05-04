# デフォルトキーマップ更新計画

## 背景

- 現在の `config.yml` にある `keymaps` をアプリケーションのデフォルト設定に反映する。
- `keymaps` をユーザー設定に追加すると既定キーマップが置き換わるため、よく使う設定を初期値として持たせる。

## 修正対象

- `src/assets/default_config.yml`
- `src/config.rs`

## 修正方針

1. `src/assets/default_config.yml` の `keymaps` を指定された内容に合わせる。
2. `src/config.rs` の `DEFAULT_KEY_MAPPINGS` を同じ内容に合わせる。
3. 既存のレガシーキーマップ移行用定数は、必要な範囲で既存テストを壊さないよう調整する。
4. `cargo test config` など、設定周辺のテストを実行して確認する。

## 期待する変更

- `ctrl+r` が既定で `History` に割り当たる。
- `G` と `g g` は新しい既定キーマップから外れる。
- `ctrl+l: ToggleSideBar` は新しい既定キーマップから外れる。
- `f12: OpenDevTools` は既定に残る。

## 実施結果

- `src/assets/default_config.yml` の `keymaps` を指定内容に更新した。
- `src/config.rs` の `DEFAULT_KEY_MAPPINGS` を指定内容に更新した。
- `cargo test config` が成功した。

## 追加対応

- `config.yml` の `keymaps` を既定キーマップ全体の置き換えではなく、既定キーマップへの差分指定として扱う。
- `keymaps:` が空、または省略された場合は既定キーマップを使う。
- `keymaps` に指定されたキーは既定キーマップへ追加または上書きする。
- 設定読み込みテストを追加し、差分指定で `ctrl+r: History` などの既定値が維持されることを確認する。

## 追加対応の実施結果

- `keymaps` 読み込み時に `DEFAULT_KEY_MAPPINGS` を土台として、ユーザー指定を追加・上書きするようにした。
- `keymaps:` が空の場合は既定キーマップへフォールバックするようにした。
- 差分指定と空指定のテストを追加した。
- `cargo test config` が成功した。

## 追加対応 2

- `ctrl+l: ToggleSideBar` をアプリケーションのデフォルトキーマップに追加する。
- `src/assets/default_config.yml` と `src/config.rs` の `DEFAULT_KEY_MAPPINGS` を一致させる。
- 設定テストを実行して確認する。

## 追加対応 2 の実施結果

- `src/assets/default_config.yml` に `ctrl+l: ToggleSideBar` を追加した。
- `src/config.rs` の `DEFAULT_KEY_MAPPINGS` に `("ctrl+l", ToggleSideBar)` を追加した。
- `cargo test config` が成功した。

## 追加対応 3

- `gg: ScrollTop` と `G: ScrollBottom` をアプリケーションのデフォルトキーマップに追加する。
- `src/assets/default_config.yml` と `src/config.rs` の `DEFAULT_KEY_MAPPINGS` を一致させる。
- 設定テストを実行して確認する。

## 追加対応 3 の実施結果

- `src/assets/default_config.yml` に `G: ScrollBottom` と `g g: ScrollTop` を追加した。
- `src/config.rs` の `DEFAULT_KEY_MAPPINGS` に `("G", ScrollBottom)` と `("g g", ScrollTop)` を追加した。
- `cargo test config` が成功した。
