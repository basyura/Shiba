# 設定仕様

## 設定ファイル

- 既定の設定ディレクトリは OS の設定ディレクトリ配下の `Shiba` である。
- `--config-dir PATH` により設定ディレクトリを変更できる。
- 読み込み対象のファイル名は `config.yml`、`config.yaml` の順である。
- 設定ディレクトリまたは設定ファイルが存在しない場合は既定設定を使う。
- 設定ファイルの未知フィールドはエラーになる。
- `--generate-config-file` は `config.yml` を生成し、内部の既定設定 YAML を書き出す。

## CLI による上書き

- `--theme` は `window.theme` より優先される。
- `--no-restore` は `window.restore` を強制的に `false` にする。
- `--debug` は設定ファイルではなく CLI オプションとして保持される。

## watch

| 項目 | 既定値 | 仕様 |
| --- | --- | --- |
| `file_extensions` | `md`, `mkd`, `markdown` | Markdown として扱うファイル拡張子。ドットは含めない。 |
| `debounce_throttle` | `50` | ファイル変更イベントのデバウンス時間。単位はミリ秒。 |

監視対象イベントはファイル作成と内容変更である。対象ファイルは拡張子一致、または内部的に追加許可されたファイルで、実ファイルである必要がある。ユーザー CSS は拡張子に関係なく監視対象へ追加される。

## keymaps

`keymaps` はキーシーケンスから `KeyAction` へのマップである。ユーザー設定は既定キーマップに追加・上書きされる。

主な既定キーマップは以下である。

| キー | アクション |
| --- | --- |
| `j`, `down` | `ScrollDown` |
| `k`, `up` | `ScrollUp` |
| `h`, `left` | `ScrollLeft` |
| `l`, `right` | `ScrollRight` |
| `ctrl+b` | `Back` |
| `ctrl+f` | `Forward` |
| `ctrl+o` | `OpenFile` |
| `ctrl+d`, `pagedown` | `ScrollPageDown` |
| `ctrl+u`, `pageup` | `ScrollPageUp` |
| `ctrl+r` | `History` |
| `ctrl+n`, `ctrl+j` | `ScrollNextSection` |
| `ctrl+p`, `ctrl+k` | `ScrollPrevSection` |
| `/` | `Search` |
| `G`, `ctrl+down` | `ScrollBottom` |
| `g g`, `ctrl+up` | `ScrollTop` |
| `ctrl+l` | `ToggleSideBar` |
| `f12` | `OpenDevTools` |
| `?` | `Help` |

`ToggleAlwaysOnTop` の既定キーはプラットフォームごとに異なる。

| プラットフォーム | キー |
| --- | --- |
| macOS | `ctrl+command+p` |
| Windows | `ctrl+alt+p` |
| その他 | `ctrl+shift+p` |

過去の既定キーマップと完全一致する設定は、現在の既定キーマップへアップグレードされる。`gg` が `ScrollTop` に設定され、`g g` が未設定の場合は `g g` へ移行される。

## scroll

| 項目 | 既定値 | 仕様 |
| --- | --- | --- |
| `step` | `50` | `ScrollDown`、`ScrollUp` の移動量。単位はピクセル。 |
| `pageStep` | `400` | `ScrollPageDown`、`ScrollPageUp` の移動量。単位はピクセル。 |

UI 側で無効な値が渡された場合は `step = 50`、`pageStep = 400` に戻る。

## search

| 項目 | 既定値 | 仕様 |
| --- | --- | --- |
| `matcher` | `SmartCase` | 検索方式。 |

`matcher` の値は以下である。

| 値 | 仕様 |
| --- | --- |
| `SmartCase` | クエリに ASCII 大文字が含まれる場合は大文字小文字を区別し、それ以外は区別しない。 |
| `CaseSensitive` | 常に大文字小文字を区別する通常検索。 |
| `CaseInsensitive` | 常に大文字小文字を区別しない通常検索。 |
| `CaseSensitiveRegex` | 大文字小文字を区別する正規表現検索。 |

## window

| 項目 | 既定値 | 仕様 |
| --- | --- | --- |
| `restore` | `false` | 前回のウィンドウ状態を復元する。 |
| `theme` | `System` | `System`、`Dark`、`Light` のいずれか。 |
| `always_on_top` | `false` | 設定項目として定義されているが、現在の起動処理では初期ピン留め状態としては使われない。 |
| `default_size` | `null` | 起動時の既定ウィンドウサイズ。`restore` で保存状態がある場合は保存状態が優先される。 |
| `menu_bar` | `false` | Linux と Windows でメニューバーを表示する。macOS では常にメニューが設定される。 |

ウィンドウ状態として位置、サイズ、フルスクリーン、ズーム倍率、ピン留め状態、最大化状態が保存される。保存は `window.restore` が有効な場合のみ行われる。保存されたピン留め状態は現在の復元処理では反映されず、起動時は未ピン留めになる。

## preview

| 項目 | 既定値 | 仕様 |
| --- | --- | --- |
| `highlight.light` | `GitHub` | ライトモードの highlight.js テーマ。 |
| `highlight.dark` | `GitHub Dark` | ダークモードの highlight.js テーマ。 |
| `css` | `null` | Markdown 本文に適用するユーザー CSS。設定ディレクトリからの相対パスとして解決される。 |
| `recent_files` | `100` | 永続化する最近使ったファイル数。`0` の場合は履歴保存しない。 |

`highlight.light` と `highlight.dark` が同じ値の場合は単一テーマ CSS が返される。異なる場合は `prefers-color-scheme` のメディアクエリでライト・ダークを切り替える。未知のテーマ名はログに出し、既定テーマへフォールバックする。

`preview.css` が設定され、読み込みに成功した場合は GitHub Markdown CSS の代わりにユーザー CSS を返す。読み込みに失敗した場合は GitHub Markdown CSS を使う。

## dialog

| 項目 | 既定値 | 仕様 |
| --- | --- | --- |
| `default_dir` | `null` | ファイル選択またはディレクトリ選択の初期ディレクトリ。 |

`default_dir` は `~/` をホームディレクトリへ展開する。存在するディレクトリでない場合は `null` として扱う。

ファイル選択ダイアログは、現在プレビュー中のファイルがある場合、その親ディレクトリを初期位置にする。プレビュー中のファイルがない場合は `dialog.default_dir`、それもなければカレントディレクトリを使う。macOS でアプリバンドルから起動され、カレントディレクトリが `/` の場合は Documents ディレクトリを使う。

## 根拠ソース

- `src/config.rs`
- `src/assets/default_config.yml`
- `src/assets.rs`
- `src/shiba.rs`
- `ui/keymaps.ts`
