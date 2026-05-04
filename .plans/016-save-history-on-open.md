# ファイルを開いたタイミングで履歴を保存する

## 背景

- 現状、履歴は `History::push` でメモリ上に追加される。
- 永続化はアプリ終了時の `Shiba::shutdown` で `self.history.save(&self.config)?` が呼ばれたときだけ行われる。
- そのため、アプリが正常終了しなかった場合や終了前に別プロセスから履歴を参照したい場合、直近で開いたファイルが `recent_files.json` に反映されない。

## 目的

- Markdown ファイルを開いて履歴に追加されたタイミングで、`recent_files.json` にも保存する。
- `Cmd+Y` で表示される履歴と永続化済み履歴の差を小さくする。

## 修正案

### 対象ファイル

- `src/history.rs`
- `src/shiba.rs`

### 方針

1. `History::push` の戻り値を変更し、履歴が実際に更新されたかどうかを呼び出し元で判定できるようにする。
   - `max_items == 0` の場合は更新なし。
   - 現在のファイルと同じパスを連続で開いた場合は更新なし。
   - 初回追加、通常追加、分岐履歴の切り詰めを伴う追加は更新あり。

2. `Shiba` 側に履歴追加と保存をまとめる小さなヘルパーを追加する。
   - 例: `push_history(&mut self, path: PathBuf) -> Result<()>`
   - `History::push(path)` が更新ありを返した場合だけ `self.history.save(&self.config)?` を呼ぶ。

3. 既存の `self.history.push(path)` 呼び出しをヘルパー経由に置き換える。
   - `preview_new`
   - renderer からの `OpenFile`
   - `WatchedFilesChanged` で別ファイルを表示した場合

4. アプリ終了時の `self.history.save(&self.config)?` は残す。
   - 終了時保存は最終的な保険として維持する。
   - ただし通常のファイルオープンでは即時保存されるようになる。

## 注意点

- `Forward` / `Back` は履歴の現在位置を移動するだけで、最近開いたファイル一覧自体を増やさないため、今回の即時保存対象には含めない。
- `preview.recent_files: 0` のときはこれまで通り履歴を保存しない。
- `History::save` は重複除去して保存するため、保存形式は既存のまま維持する。
- 起動時に保存済み履歴を復元するとき、現在位置を先頭ではなく最後の要素に合わせる。
  - 先頭を現在位置にすると、起動後に新しいファイルを開いた際に `History::push` が `index + 1` 以降を切り捨て、既存履歴が失われる。
  - 即時保存によりこの切り捨て結果がすぐ永続化されるため、復元時の `index` を修正する。

## 確認方法

1. `cargo test`
2. 必要に応じて `npm run lint`
3. 手動確認
   - Shiba で Markdown ファイルを開く。
   - アプリを終了する前に `~/Library/Application Support/Shiba/recent_files.json` が更新されることを確認する。
   - `Cmd+Y` の履歴表示に開いたファイルが表示されることを確認する。

## 反映内容

- `src/history.rs`
  - `History::push` が履歴を実際に更新したかどうかを `bool` で返すように変更した。
  - 保存済み履歴を復元するとき、現在位置を最後の履歴項目に合わせるように変更した。
- `src/shiba.rs`
  - 履歴追加と保存をまとめる `push_history` を追加した。
  - `preview_new`、renderer からの `OpenFile`、`WatchedFilesChanged` の履歴追加処理を `push_history` 経由に変更した。
  - アプリ終了時の履歴保存は残した。
- 確認
  - `cargo test` 通過。
  - `cargo fmt --check` 通過。
  - `git diff --check` 通過。
  - Apple Silicon 用 `Shiba.app` を再ビルド済み。
