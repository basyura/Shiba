# 修正計画: Open With からのマークダウン表示

## 目的
OS の「このアプリケーションで開く」から Markdown ファイルが指定された場合に、アプリ内でプレビューを開始する。

## 現状整理
- 起動時の引数は `src/cli.rs` で `init_file` に取り込まれる。
- 起動後のファイル操作は `src/shiba.rs` の `preview_new` に集約されている。
- OS からの「ファイルを開く」イベントは `src/wry/event_loop.rs` で拾われていない。

## 進め方
1. 対象 OS で「Open With」の挙動を確認する。
   - 起動時引数にファイルパスが渡るか
   - 既に起動中のときにイベントが発生するか
2. tao/wry のイベント仕様を確認する。
   - macOS の open file イベント
   - Windows/Linux で相当するイベントや引数受け渡し
3. アプリ内部イベントを追加する。
   - `src/renderer.rs` に新しい `Event` を追加
   - `src/wry/event_loop.rs` で OS イベントを受け取り `EventLoopProxy` へ送る
4. `src/shiba.rs` で新イベントを `preview_new` に接続する。
   - 相対パスの正規化と拡張子チェックは既存の流れに寄せる
5. 必要なら履歴保存やタイトル更新が正しく動くか確認する。

## 変更候補ファイル
- `src/wry/event_loop.rs`
- `src/renderer.rs`
- `src/shiba.rs`
- 必要に応じて `src/cli.rs`（引数の扱い確認のみ）

## テスト方針
- 実機で「Open With」から `.md` を指定して表示されることを確認
- 既にアプリ起動中でも同様に表示できることを確認
- 既存の起動引数 (`shiba README.md`) が動くことを再確認

## リスクと注意
- OS ごとにイベントの型が異なる可能性があるため、条件分岐が増える
- 既に起動中のウィンドウを使うか、新規ウィンドウを出すかの判断が必要
