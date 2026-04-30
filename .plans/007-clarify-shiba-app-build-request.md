# ビルド依頼時の Shiba.app 生成手順を明確化する

## 1. 課題

`AGENTS.md` にはビルド依頼時に Apple Silicon 用バイナリのみを `Shiba.app` として生成する方針がある。
しかし、`make build` だけで終わらせないことや具体的な生成手順が明記されていない。

## 2. 対象箇所の確認

- `AGENTS.md` の `Build, Test, and Development Commands` にビルド関連の指示がある。
- 現状の Apple Silicon 専用 app 生成指示は 1 行のみで、手順が不足している。

## 3. 仕様整理

- ユーザーが「ビルド」と依頼した場合は `make build` だけで終わらせない。
- 特に指定がない限り Apple Silicon 専用の `Shiba.app` を生成する。
- universal app 用の `make Shiba.app` は通常使わない。

## 4. 実装方針

- 既存の Apple Silicon 専用 app 生成指示を複数行に分ける。
- `npm run release`、`cargo build --release --target=aarch64-apple-darwin`、`assets/Shiba.app` のコピー、arm64 バイナリ配置まで明記する。

具体的な追記案:

```md
- ビルドを依頼された場合は `make build` だけで終わらせず、特に指定がない限り Apple Silicon 用バイナリのみを `Shiba.app` として生成する。
- Apple Silicon 専用 `Shiba.app` は、`npm run release` と `cargo build --release --target=aarch64-apple-darwin` を実行し、`assets/Shiba.app` を `Shiba.app` にコピーして `target/aarch64-apple-darwin/release/shiba` を `Shiba.app/Contents/MacOS/shiba` に配置して生成する。
- `make Shiba.app` は universal app 用で x86_64 も要求するため、特に指定がない限り使わない。
```

## 5. 影響確認

- 既存の universal app 説明と矛盾しないことを確認する。
- 通常のビルド依頼時の期待成果物が `Shiba.app` だと明確になることを確認する。

## 6. 動作確認方針

- ドキュメント変更のためビルドは不要。
- 差分を確認する。

## 7. 対応内容 (結果)

- `AGENTS.md` に、ビルド依頼時は `make build` だけで終わらせず Apple Silicon 専用 `Shiba.app` を生成することを明記した。
- Apple Silicon 専用 `Shiba.app` の具体的な生成手順を追記した。
- `make Shiba.app` は universal app 用のため、通常のビルド依頼では使わないことを独立した項目として明記した。
