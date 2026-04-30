# Apple Silicon 専用 Shiba.app 生成手順の追記

## 1. 課題

ビルド依頼時に universal app を生成しようとすると、x86_64 ターゲットが必要になり失敗する場合がある。
この環境では Apple Silicon 用バイナリのみを `Shiba.app` として生成する運用にしたい。

## 2. 対象箇所の確認

- `AGENTS.md` の `Build, Test, and Development Commands` に macOS app ビルドの説明がある。
- 現状は `make Shiba.app` が両アーキテクチャを要求することのみ説明されている。
- Apple Silicon 専用 app の生成方針は記載されていない。

## 3. 仕様整理

- ユーザーがビルドを依頼したときは、Apple Silicon 用バイナリのみを `Shiba.app` として生成する。
- universal binary が必要な場合以外は `make Shiba.app` を使わない。
- `Shiba.app` には `target/aarch64-apple-darwin/release/shiba` を配置する。

## 4. 実装方針

- `AGENTS.md` の macOS app build note の近くに Apple Silicon 専用ビルドの運用ルールを追記する。
- 既存の universal macOS build の説明は残し、用途を区別する。

具体的な追記案:

```md
- ビルドを依頼された場合は、特に指定がない限り Apple Silicon 用バイナリのみを `Shiba.app` として生成する。`make Shiba.app` は universal app 用で x86_64 も要求するため使わない。
```

## 5. 影響確認

- `AGENTS.md` の指示が既存の universal build 説明と矛盾しないことを確認する。
- 今後のビルド依頼時に Apple Silicon 専用 app 生成方針が明確になることを確認する。

## 6. 動作確認方針

- ドキュメント変更のためビルドは不要。
- Markdown の記載内容を確認する。

## 7. 対応内容 (結果)

- `AGENTS.md` に、ビルド依頼時は特に指定がない限り Apple Silicon 用バイナリのみを `Shiba.app` として生成する方針を追記した。
- `make Shiba.app` は universal app 用で x86_64 も要求するため、通常のビルド依頼では使わないことを明記した。
