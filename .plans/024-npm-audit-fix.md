# npm audit fix 対応計画

## 目的

npm 依存関係の脆弱性と npm 汚染リスクを確認しながら、`npm audit fix` を適用する。
本番依存の Mermaid / MathJax 周辺を優先し、更新後の署名検証とビルド確認を行う。

## 現状

- `npm audit` では 19 件の脆弱性が検出されている。
- `npm audit --omit=dev` では 12 件の脆弱性が検出されている。
- `npm audit signatures` は 599 パッケージの registry signature 検証に成功している。
- 最近の npm 汚染で名前が出ている `@antv/*`, `echarts-for-react`, `size-sensor`, `timeago.js`, `@tanstack/*`, `axios`, `mistralai` は lockfile に含まれていない。

## 修正案

1. `npm audit fix` を実行する。
   - `package-lock.json` と必要に応じて `package.json` の差分を確認する。
   - 依存の大きな変更が入った場合は内容をレビューする。

2. 署名と脆弱性を再確認する。
   - `npm audit --omit=dev`
   - `npm audit`
   - `npm audit signatures`

3. UI とバンドルを検証する。
   - `npm run lint:ui`
   - `npm run release`

4. 必要に応じて Rust 側を確認する。
   - UI バンドル差分が発生した場合は、必要な範囲で Rust テストを実行する。

## 重点確認

- Mermaid 更新により Mermaid 図の描画が壊れていないか。
- MathJax / speech-rule-engine / xmldom 更新により数式表示が壊れていないか。
- install script を持つ依存に不審な追加がないか。
- `npm audit signatures` が成功するか。

## 完了条件

- `npm audit fix` 適用後の差分を説明できる。
- `npm audit signatures` が成功する。
- `npm run lint:ui` と `npm run release` が成功する。
- 残る脆弱性がある場合は、本番依存か開発依存かを整理して報告する。

