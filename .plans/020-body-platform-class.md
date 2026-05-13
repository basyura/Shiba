# body タグへの platform class 設定

## 目的

OS ごとの CSS 分岐をしやすくするため、起動時に `body` タグへ
platform を表す class を設定する。

## 方針

- `ui/index.html` に小さなインラインスクリプトを追加する。
- macOS では `body` に `platform-darwin` を付与する。
- Windows では `body` に `platform-windows` を付与する。
- Linux など対象外の platform では class を付与しない。
- ビルド成果物の `src/assets/index.html` は通常のバンドル処理で更新する。
- `ui/index.html` 内のスクリプトを抽出して実行するテストを追加する。
- `navigator.userAgentData.platform` があれば `navigator.platform` より優先されることを確認する。
- `tests` 配下を ESLint / Prettier の対象に含める。

## 修正対象

- `ui/index.html`
- `src/assets/index.html`
- `tests/ui/body-platform-class.test.mjs`
- `package.json`

## 確認

- `npm run bundle` を実行し、生成済み HTML に反映されることを確認する。
- `npm run test:ui` を実行し、platform class の付与条件を確認する。
