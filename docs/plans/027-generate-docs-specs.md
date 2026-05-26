# docs/specs 仕様ドキュメント生成計画

## 目的

`docs/specs` に、現在のソースコードと整合するアプリケーション仕様ドキュメントを Markdown で生成する。

## 調査対象

- Rust 側の起動処理、設定読み込み、CLI、永続化、履歴、Markdown 処理、WebView 連携
- UI 側の React コンポーネント、Reducer、IPC、キーマップ、検索、Markdown 表示
- 既存 README、既定設定、テストデータ

## 修正案

1. ソースコードから主要機能と設定項目を読み取り、仕様として必要な粒度を整理する。
2. `docs/specs` 配下に領域別の Markdown ファイルを追加する。
   - アプリケーション概要
   - 設定仕様
   - Markdown 表示仕様
   - UI 操作仕様
   - IPC とプラットフォーム連携仕様
3. 各ドキュメントには、根拠となる主なソースファイルを記載する。
4. 生成後、内容が現状コードと矛盾していないか再確認する。

## 検証

- 生成した Markdown の内容をソースコードと照合する。
- 必要に応じて `rg` で関連実装を再検索し、記述漏れや誤記を確認する。

## 実施結果

- `docs/specs/README.md` を追加した。
- `docs/specs/application.md` を追加した。
- `docs/specs/configuration.md` を追加した。
- `docs/specs/markdown-preview.md` を追加した。
- `docs/specs/ui-behavior.md` を追加した。
- `docs/specs/ipc-platform.md` を追加した。
- 主要な根拠ソースを各仕様ファイルに記載した。
- `AGENTS.md` の計画ファイル配置ルールを `docs/plans` に更新した。
