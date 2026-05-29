# 履歴から開いた Markdown の更新監視修正計画

## 背景

ファイル未指定で Shiba を起動し、履歴から Markdown ファイルを開いた場合、その Markdown ファイルを変更しても描画内容が最新化されない。
Markdown ファイルを直接指定して起動した場合は最新化される。

## 原因

ファイル未指定で起動後に履歴からファイルを開いた場合は、renderer からの `open_file` メッセージ処理が `preview.show()` を直接呼んでおり、監視登録を行う `preview_new()` を経由していない。
そのため、履歴から開いたファイルは変更監視対象にならない。

## 修正方針

- renderer からの `OpenFile` メッセージ処理を `preview_new()` 経由に変更し、履歴から開いたファイルも監視対象に登録する。
- 監視イベント判定や debounce の挙動は変更しない。

## 確認方法

- `cargo fmt --check`
- `cargo test shiba::tests`
- `cargo test watcher::tests`
- `cargo build --release`
