# config.yml 指定 CSS のリアルタイム再適用

## 背景

リリースビルド時も、利用者が `config.yml` の `preview.css` で指定した
CSS ファイルを変更した場合に、起動中のプレビューへスタイルを再適用できるようにする。

現状は起動時に `Assets::new` でユーザー CSS を読み込み、`/github-markdown.css`
として WebView に配信しているため、CSS ファイルを編集しても起動中の
WebView には変更が反映されない。

## 方針

1. `config.yml` で指定されたユーザー CSS の実パスを取得できるようにする。
2. ユーザー CSS が指定されている場合、その CSS ファイルを既存のファイル監視に追加する。
3. ユーザー CSS の変更イベントを受け取ったら、WebView 側で CSS を再読み込みする。
4. Markdown 本文の再レンダリングは必要な場合に限定し、CSS 変更ではスタイルの再適用を優先する。
5. ユーザー CSS が未指定、または読み込みに失敗した場合は従来どおり bundled CSS を使う。

## 修正候補

- `src/assets.rs`
  - ユーザー CSS のパス解決処理を再利用しやすい形にする。
  - `/github-markdown.css` のロード時にユーザー CSS を最新内容で読めるよう検討する。
- `src/shiba.rs`
  - 起動時にユーザー CSS のパスを watcher に登録する。
  - CSS 変更イベントを Markdown 変更イベントと区別して扱う。
- `src/renderer.rs`
  - CSS 再適用用の main-to-renderer メッセージを追加する。
- `ui/dispatcher.ts` / `ui/ipc.ts`
  - CSS 再適用メッセージを受け取り、対象 `<link>` の URL に cache buster を付けて差し替える。

## 検証

- `cargo test`
- 可能なら `npm run lint:ui`
- 手動確認:
  - `config.yml` で CSS を指定して Shiba を起動する。
  - CSS ファイルを編集して保存する。
  - アプリ再起動なしでプレビューのスタイルが変わることを確認する。

## 実装結果

- `config.yml` の `preview.css` で指定された CSS パスを起動時に watcher へ追加する。
- CSS 変更イベントは Markdown 変更イベントから分離して扱い、本文の再レンダリングではなく
  stylesheet の再読み込みだけを renderer へ通知する。
- `/github-markdown.css` はユーザー CSS 指定時に都度ファイルから読み直す。
- WebView 側では `reload_style` メッセージを受け取り、`#markdown-css` の URL に
  cache buster を付けて差し替える。

## 実行済み検証

- `cargo test`
- `npm run lint:ui`
- `npm run lint`
