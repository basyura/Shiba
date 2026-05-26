# Markdown プレビュー仕様

## 解析方式

Markdown は Rust 側で `pulldown-cmark` により解析され、Renderer へ JSON ベースの render tree として送られる。UI 側は render tree を React 要素へ変換して表示する。

有効な `pulldown-cmark` オプションは以下である。

- strikethrough
- footnotes
- tables
- task lists
- math
- GFM

## 対応する Markdown 要素

| 要素 | 表示仕様 |
| --- | --- |
| paragraph | `<p>` として表示する。 |
| heading | `<h1>` から `<h6>` として表示し、ID がある場合は保持する。 |
| table | `<table>`、`thead`、`tbody`、`tr`、`th`、`td` として表示する。列 alignment を style に反映する。 |
| blockquote | `<blockquote>` として表示する。 |
| list | `<ol>`、`<ul>`、`<li>` として表示する。 |
| task list | disabled checkbox と task-list 用 class を付けて表示する。 |
| emphasis / strong / strikethrough | `<em>`、`<strong>`、`<del>` として表示する。 |
| link | `<a>` として表示する。通常リンクは title に URL と任意のリンク title を入れる。 |
| autolink | `<a>` として表示するが title は付けない。 |
| image | `<img>` として表示する。alt は子要素の raw text から作る。 |
| code | インライン code または code fence として表示する。 |
| footnote | 参照は本文中に表示し、定義はページ下部の footnotes section にまとめる。 |
| emoji | emoji 名を title と aria-label に持つ `<span>` として表示する。 |
| math | MathJax で SVG 化して表示する。 |
| GitHub alert | 種別ごとのアイコン付き alert block として表示する。 |
| raw HTML | sanitizer を通した HTML を `<span dangerouslySetInnerHTML>` で表示する。 |
| soft break / hard break | soft break は空白、hard break は `<br>` として扱う。 |
| rule | `<hr>` として表示する。 |

## Code fence

code fence の言語に応じて UI 側で追加処理を行う。

| 言語 | 仕様 |
| --- | --- |
| highlight.js が認識する言語 | highlight.js でシンタックスハイライトする。 |
| `mermaid` | mermaid.js で SVG 図として描画する。parse エラー時はエラーテキストを表示する。 |
| `math` | MathJax で数式ブロックとして描画する。 |
| その他または言語なし | 通常の `<code>` として表示する。 |

## Math

- inline math は `math-expr-inline` class を付ける。
- display math は `math-expr-block` class を付ける。
- `math` code fence は `code-fence-math` class を付ける。
- MathJax は TeX input と SVG output を使う。

## Mermaid

- Mermaid は現在のカラースキームに応じて `default` または `dark` テーマで初期化される。
- カラースキームが変わると再初期化される。
- レンダリング ID はプレビューごとにリセットされる。

## HTML sanitizer

raw HTML は `ammonia` で sanitization される。

- `script` やイベントハンドラなどの危険な要素・属性は除去される。
- generic attribute として `name` と `id` が許可される。
- 相対 URL は現在の Markdown ファイルの親ディレクトリ基準へ付け替えられる。
- `#hash`、`http://`、`https://`、`//` で始まる URL は付け替えない。

## ローカルパスとリソース

- MarkdownContent は Markdown ファイルの親ディレクトリを base directory として持つ。
- 画像や raw HTML 内の相対 URL は base directory 基準に変換される。
- WebView のカスタムプロトコルは、バンドル済みアセットに加えて、要求されたローカルパスを動的に読み込む。
- `%20` や `%23` のような percent-encoded path はデコードして読み込む。
- MIME type は拡張子から推定し、不明な場合は `application/octet-stream` にする。

## 最終変更位置へのスクロール

- 同じファイルの再描画時は、旧ソースと新ソースの共通 prefix から UTF-8 境界を考慮した変更位置を算出する。
- render tree には `modified` token が挿入される。
- UI 側は `modified` token の位置が表示範囲外であれば、該当要素を中央付近へスクロールする。
- 別ファイルを開いた場合は最終変更位置ではなくページ先頭へスクロールする。

## 検索ハイライト

- Rust 側は表示テキストとソース位置の対応表を保持する。
- 検索時は一致範囲を Markdown ソース範囲へ戻し、render tree に `match`、`match-current`、`match-start`、`match-current-start` token を挿入する。
- UI 側は `match-start` と `match-current-start` をカウントして検索件数を算出する。
- 検索クエリが空、または matcher の構築に失敗した場合は通常再描画へ戻る。

## 根拠ソース

- `src/markdown/parser.rs`
- `src/markdown/search.rs`
- `src/markdown/sanitizer.rs`
- `src/assets.rs`
- `ui/markdown.tsx`
- `ui/components/Article.tsx`

