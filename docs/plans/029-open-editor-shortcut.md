# `<C-e>` で現在の Markdown をエディターで開く計画

## 目的

右クリックメニューの `Editor` と同じ動作を、キーボードショートカット
`<C-e>` から実行できるようにする。

## 修正案

- `src/config.rs`
  - `KeyAction` に外部エディター起動用のアクションを追加する。
  - デフォルト keymaps に `ctrl+e` を追加する。
  - 既存のデフォルト keymaps を使用している設定が、新しいデフォルトへ更新されるよう旧デフォルト定数を追加する。
  - 関連テストを更新する。
- `ui/ipc.ts`
  - renderer から main へ送る `open_editor` メッセージを追加する。
- `ui/keymaps.ts`
  - 新しい key action から `open_editor` メッセージを送る。
- `src/shiba.rs`
  - `open_editor` メッセージ受信時に、既存の `open_editor()` を呼ぶ。
  - 開いている Markdown ファイルがない場合は、エラーを出さず何もしない。
  - 履歴上の直近ファイルではなく、実際に Shiba で表示中の Markdown だけを対象にする。
- `src/assets/default_config.yml`
  - サンプル設定の keymaps に `ctrl+e: OpenEditor` を追加する。

## 確認方法

- `cargo test config`
- 必要に応じて TypeScript の型チェックを実行する。

## 実施結果

- `ctrl+e` を `OpenEditor` に割り当てた。
- renderer から main へ `open_editor` IPC を送信し、既存の `open_editor()` を呼ぶようにした。
- 開いている Markdown ファイルがない場合は、何もしないようにした。
- Shiba で Markdown を表示していない状態では、履歴の直近ファイルをエディターで開かないようにした。
- 旧デフォルト keymaps から新デフォルトへアップグレードされるようにした。
- 関連テストを追加した。
