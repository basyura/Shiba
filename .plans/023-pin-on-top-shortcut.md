# Pin/Unpin On Top ショートカット対応計画

## 目的

`Pin/Unpin On Top` をキーボードショートカットから実行できるようにする。
既存のメニュー機能は利用し、設定ファイルの `keymaps` から任意のキーに割り当て可能にする。

## 現状

- `Pin/Unpin On Top` のメニュー項目は `src/wry/menu.rs` に存在する。
- メニュー実行時は `src/shiba.rs` の `toggle_always_on_top()` で window の always-on-top 状態を切り替えている。
- `KeyAction` には always-on-top 切り替え用のアクションがない。
- UI から main process へ always-on-top 切り替えを要求する IPC メッセージがない。
- そのため `keymaps` でショートカットを割り当てることはまだできない。

## 修正案

1. `src/config.rs`
   - `KeyAction` に `ToggleAlwaysOnTop` を追加する。
   - デフォルト keymap に `ctrl+shift+p: ToggleAlwaysOnTop` を追加する。

2. `ui/ipc.ts`
   - `KeyAction` 型に `ToggleAlwaysOnTop` を追加する。
   - UI から main へ送る `toggle_always_on_top` メッセージを追加する。

3. `ui/keymaps.ts`
   - `ToggleAlwaysOnTop` のショートカット処理を追加する。
   - 実行時は `toggle_always_on_top` IPC を送信する。
   - Guide に表示される説明文も追加する。

4. `src/renderer.rs`
   - `MessageFromRenderer` に `ToggleAlwaysOnTop` を追加する。

5. `src/shiba.rs`
   - renderer からの `ToggleAlwaysOnTop` メッセージを受けて、既存の `toggle_always_on_top()` を呼ぶ。

6. `src/assets/default_config.yml`
   - `keymaps` に `ctrl+shift+p: ToggleAlwaysOnTop` を追記する。

7. テストと確認
   - `cargo test config` など、影響範囲の Rust テストを実行する。
   - 可能なら `npm run lint:tsc` で TypeScript 型チェックを確認する。

## 推奨

`ToggleAlwaysOnTop` アクションを追加し、デフォルトで `ctrl+shift+p` に割り当てる。
設定ファイルでは以下のように割り当てられる状態にする。

```yaml
keymaps:
  ctrl+shift+p: ToggleAlwaysOnTop
```
