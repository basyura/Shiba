# MacVim.app 単体指定の解決

## 背景

`config.yml` の `editor.path` に `MacVim.app` のみを指定した場合、現在は相対パスとして扱われるため、実際のアプリケーションバンドルを見つけられない。

```yaml
editor:
  path: MacVim.app
```

## 修正案

1. macOS のエディタ起動処理に、`MacVim.app` だけが指定された場合の解決処理を追加する。
2. 解決順は `/Applications/MacVim.app` を優先する。
3. `/Applications/MacVim.app` が存在しない場合は、`/opt/homebrew/Cellar/macvim` 配下から `MacVim.app` を探索する。
4. 見つかった `MacVim.app` は既存の MacVim 専用処理に渡し、`Contents/bin/mvim --remote-silent` で開く。
5. 見つからない場合は、従来通り指定値を使って起動を試みる。

## 変更予定ファイル

- `src/shiba.rs`

## テスト方針

- `MacVim.app` 単体指定で `/Applications/MacVim.app` を優先することをユニットテストする。
- `/Applications/MacVim.app` がない場合に Homebrew Cellar 配下の `MacVim.app` を使用することをユニットテストする。
- 通常の `.app` や実行ファイル指定の既存挙動が変わらないことを既存テストで確認する。
