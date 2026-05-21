# 相対パス画像の表示対応計画

## 目的

Markdown 内で相対パス指定された画像を、Markdown ファイルの配置ディレクトリを基準に表示できるようにする。

## 現状の確認

- Markdown の通常画像は `src/markdown/parser.rs` で `base_dir` を付与している。
- HTML 内画像は `src/markdown/sanitizer.rs` の相対 URL 変換で `base_dir` を付与している。
- WebView の `shiba` カスタムプロトコルは `src/assets.rs` でファイル読み込みを行っている。
- パスに空白や URL エンコードが含まれる場合、またはスキーム付き URL として扱われる場合に、ローカル画像を正しく読めない可能性がある。

## 修正案

1. 相対パス画像の URL を WebView から読めるローカルリソース URL として安定して扱えるようにする。
2. `src/assets.rs` の外部リソース読み込みで、カスタムプロトコル経由のパスを安全に復元してから `fs::read` する。
3. 通常 Markdown 画像と HTML 内画像の両方について、既存の相対リンクテストを崩さない形でテストを追加または更新する。
4. `cargo test` で関連テストを確認する。

## 想定編集ファイル

- `src/markdown/parser.rs`
- `src/markdown/sanitizer.rs`
- `src/assets.rs`
- 必要に応じて snapshot または testdata

## 検証

- 相対パス画像が Markdown ファイルのディレクトリ基準で表示できること。
- 外部 URL、ハッシュリンク、通常の相対リンクの挙動が変わらないこと。
- Rust の関連テストが通ること。

## 実施結果

- `src/assets.rs` でカスタムプロトコル経由のパスを percent-decode してから読み込むようにした。
- 空白や `#` を含むローカル画像パスが読めることを確認する回帰テストを追加した。
- `cargo fmt --check` と `cargo test` が通ることを確認した。
