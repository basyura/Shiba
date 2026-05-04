# プレビューの変更位置スクロール実装調査

## 目的

Markdown ファイル更新時に、プレビュー側の表示位置が変更箇所へ移動しているように見える挙動について、該当する実装の有無と処理の流れを確認する。

## 調査結果

該当実装は存在する。

ただし、Markdown ソース上の変更位置からプレビューの `scrollTop` を直接数値計算しているわけではない。実装としては、変更位置に対応するレンダーツリー上へ特別な `modified` トークンを挿入し、React 側で不可視の DOM マーカーに変換したうえで、その要素へ `scrollIntoView` している。

## 実装の流れ

1. `src/shiba.rs` の `PreviewContent::show` で Markdown ファイルを読み直す。
2. 同じファイルの再描画時のみ、前回内容と今回内容を比較して変更開始位置の UTF-8 byte offset を求める。
3. `src/markdown/parser.rs` の `MarkdownContent::modified_utf8_offset` が、前後の Markdown 内容から最初に差分が出る位置を UTF-8 文字境界に補正して返す。
4. `MarkdownParser` に offset を渡し、レンダーツリー生成時に該当する text token の前後へ `{"t":"modified"}` を挿入する。
5. `ui/markdown.tsx` が `modified` token を `span.last-modified-marker` に変換し、その `ref` を `lastModified` として保持する。
6. `ui/components/Article.tsx` が `lastModified` の DOM 要素を確認し、画面外なら `scrollIntoView({ block: 'center', inline: 'center' })` を呼ぶ。

## 主な該当箇所

- `src/shiba.rs`
  - `PreviewContent::show`
  - `prev_content.modified_utf8_offset(&self.content)` で変更 offset を計算
  - `MarkdownParser::new(&self.content, offset, ())` へ offset を渡す

- `src/markdown/parser.rs`
  - `MarkdownContent::modified_utf8_offset`
  - `RenderTreeEncoder::text`
  - `modified` marker を text token 内、または fallback として末尾へ挿入

- `ui/ipc.ts`
  - `t: 'modified'` がレンダーツリー上の特殊 token として定義されている

- `ui/markdown.tsx`
  - `case 'modified'` で `lastModified()` を呼び出す
  - `lastModified()` は `span.last-modified-marker` を生成し、ref を保存する
  - fenced code block など、レンダリング後に token の内部位置を保持しづらいケースでは、ブロック直前へ marker を置く処理がある

- `ui/components/Article.tsx`
  - `lastModified?.current` を取得
  - marker が viewport 外なら `scrollIntoView` で中央付近へスクロールする

- `ui/style.css`
  - `.last-modified-marker` は `display: none` にせず `width: 0; height: 0;` としている
  - コメント上も `scrollIntoView` のための marker であることが示されている

## 挙動上の注意点

- 新しいファイルを開いた場合は `is_new` が true になり、offset は `None` になるため変更位置スクロールは行われない。
- 同じファイルを再読み込みしたときのみ、前回内容との差分から変更位置が検出される。
- 検出しているのは「最初に差分が出た byte offset」であり、エディタのカーソル位置や行番号そのものではない。
- marker がすでに viewport 内にある場合、`Article.tsx` 側でスクロールは行われない。
- text token で消費されない変更位置の場合は、fallback として最後の text の後に `modified` marker が置かれる。

## Markdown ビューアーとしての一般性

この実装方針は、Markdown ビューアーとして実用的な方式の一つではあるが、唯一の標準方式ではない。

Markdown のソース位置とレンダリング後の HTML 表示位置は 1:1 で対応しない。見出し、リスト、コードブロック、画像、数式、HTML、折り返しなどにより表示上の高さが変わるため、行番号や byte offset から `scrollTop` を直接計算する方式は誤差が出やすい。

その点で、Shiba のようにレンダリング結果へ marker を埋め込み、実際の DOM 要素に対して `scrollIntoView` する方式は自然で堅実な実装といえる。

一方で、一般的な Markdown プレビューでは以下のような方式も使われる。

- エディタのカーソル行に対応する heading や block を探して同期スクロールする。
- Markdown AST に source position を保持し、対応する DOM 要素へスクロールする。
- エディタ側のスクロール比率をプレビュー側にも反映する。
- 現在のプレビュー scroll 位置を維持する。
- ファイル更新時は先頭へ戻す、またはスクロールしない。

Shiba の実装は「エディタのカーソル位置同期」ではなく、「ファイル更新時に前回内容との差分位置へ自動スクロールする」方式である。この点はやや独自寄りだが、DOM marker を使って表示位置へ移動する考え方自体は Markdown ビューアーとして不自然ではない。

## 変更する場合の候補

現時点では調査のみで、コード変更は行わない。

今後この挙動を変更する場合は、目的に応じて以下が候補になる。

- 変更位置への自動スクロールを無効化する場合
  - `ui/components/Article.tsx` の `lastModified` に対する `scrollIntoView` を抑止する。

- スクロール位置を中央ではなく上寄せ・下寄せにする場合
  - `scrollIntoView` の `block` オプションを変更する。

- 変更検出そのものを止める場合
  - `src/shiba.rs` で `MarkdownParser::new` に渡す offset を常に `None` にする。

- 変更位置の検出精度を変える場合
  - `src/markdown/parser.rs` の `modified_utf8_offset` または `RenderTreeEncoder::text` の marker 挿入ロジックを見直す。
