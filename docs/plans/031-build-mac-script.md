# macOS 用ビルドスクリプト追加計画

## 目的

macOS Apple Silicon 用のリリースビルドを行い、`Shiba.app` を生成する `build_mac.sh` を追加する。

## 方針

- 既存の `build_win.sh` とプロジェクト指示を確認し、同じ粒度のスクリプトとして追加する。
- `make Shiba.app` は universal app 用で x86_64 も要求するため使わない。
- `npm run release` と `cargo build --release --target=aarch64-apple-darwin` を実行する。
- `assets/Shiba.app` を `Shiba.app` にコピーし、生成した `shiba` バイナリを `Shiba.app/Contents/MacOS/shiba` に配置する。
- 最後に `ls -lh Shiba.app/Contents/MacOS/shiba` などで成果物を確認する。

## 変更予定ファイル

- `build_mac.sh`

## 確認内容

- スクリプトの構文確認を行う。
- 必要に応じて実行権限を付与する。

