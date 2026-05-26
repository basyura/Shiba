# IPC とプラットフォーム連携仕様

## IPC の基本

Rust 側と UI 側は WebView の IPC で JSON メッセージを送受信する。

- UI から Rust へは `window.ipc.postMessage(JSON.stringify(message))` を使う。
- Rust から UI へは `window.postShibaMessageFromMain(...)` を評価する。
- Markdown render tree は通常 JSON より大きいため、Rust 側で raw writer を使って直接 script を組み立てる。
- UI 側の IPC 送信失敗は再帰的なエラー送信を避けるため throw しない。

## UI から Rust へのメッセージ

| kind | 仕様 |
| --- | --- |
| `init` | UI 初期化完了を通知する。Rust 側は設定と初期表示内容を送る。 |
| `quit` | アプリを終了する。 |
| `forward` / `back` | プレビュー履歴を前後へ移動する。 |
| `reload` | 現在ファイルを再読み込みする。 |
| `file_dialog` | ファイル選択ダイアログを開く。 |
| `dir_dialog` | 監視ディレクトリ選択ダイアログを開く。 |
| `search` | クエリ、現在 index、matcher を Rust 側へ渡し、検索ハイライトを再生成する。 |
| `open_file` | 指定パスを開く。履歴 UI から使われる。 |
| `zoom_in` / `zoom_out` | WebView のズーム倍率を変更する。 |
| `drag_window` | ウィンドウドラッグを開始する。 |
| `toggle_maximized` | ウィンドウ最大化を切り替える。 |
| `open_menu` | アプリメニューを表示する。任意で表示位置を渡す。 |
| `toggle_menu_bar` | メニューバーを切り替える。 |
| `toggle_always_on_top` | ウィンドウの常に手前表示を切り替える。 |
| `open_devtools` | DevTools を開く。 |
| `error` | UI 側エラーを Rust 側へ通知する。Rust 側ではエラーとして扱う。 |

## Rust から UI へのメッセージ

| kind | 仕様 |
| --- | --- |
| `render_tree` | Markdown の render tree を渡す。 |
| `path_changed` | 現在プレビュー中のパス変更を通知する。 |
| `config` | キーマップ、スクロール設定、検索設定、最近使ったファイル、ホームディレクトリ、ウィンドウ外観を渡す。 |
| `search` | 検索 UI を開く。 |
| `search_next` / `search_previous` | 検索中の現在一致を移動する。 |
| `outline` | Outline palette を開く。 |
| `welcome` | Welcome 画面を表示する。 |
| `history` | History palette を開く。 |
| `help` | Help dialog を開く。 |
| `zoomed` | ズーム変更通知を表示する。 |
| `reload` | 再読み込み通知を表示する。 |
| `reload_style` | Markdown CSS を再読み込みする。 |
| `always_on_top` | ピン留め状態を UI へ反映する。 |
| `debug` | UI 側の debug log を有効化する。 |

## WebView とカスタムプロトコル

- WebView は `shiba://localhost/index.html` を初期 URL として開く。
- カスタムプロトコル `shiba` は、バンドル済みの `index.html`、`bundle.js`、`style.css`、Markdown CSS、highlight.js CSS、ロゴ画像を返す。
- バンドルにないパスはローカルファイルとして動的に読み込む。
- 読み込みに成功した場合は 200、失敗した場合は 404 を返す。
- Windows の favicon は WebView2 側の要求に対して body なしで MIME type を返す。

## ナビゲーション

- `index.html` へのナビゲーションは許可する。
- `index.html#...` の hash link は許可する。
- custom protocol のその他パスはローカルパスとして Rust 側イベントへ変換し、WebView の実ナビゲーションは止める。
- 外部 URL は OS の既定アプリケーションで開き、WebView の実ナビゲーションは止める。
- 新規ウィンドウ要求は拒否する。

ローカルパスイベントでは、相対パスの場合、現在プレビュー中ファイルの親ディレクトリ基準へ解決する。解決後のパスが Markdown 拡張子なら Shiba 内で開き、それ以外は OS の既定アプリケーションで開く。

## ファイルドロップと外部オープン

- WebView へのファイルドロップは、最初の 1 ファイルだけを開く。
- OS から複数ファイルを開くイベントを受けた場合、初期化前なら pending に積み、初期化後なら Markdown として開く。

## ファイル監視

- `--no-watch` が指定されていない場合は OS 依存の watcher を使う。
- Linux では Linux 用実装、それ以外では標準実装を使う。
- 存在しないパスを監視する場合は、存在する祖先ディレクトリを再帰監視対象にする。
- ファイル変更イベントでは、まずユーザー CSS の変更を取り出し、該当する場合は `reload_style` を送る。
- 現在プレビュー中のファイルが変更対象に含まれる場合は、そのファイルを再描画する。
- 現在ファイルが含まれない場合は、変更対象の最後のパスをプレビュー対象として選ぶ。

## メニュー

Rust 側メニューは以下の操作を提供する。

- Quit
- Forward / Back
- Reload
- OpenFile
- WatchDir
- Search / SearchNext / SearchPrevious
- Outline
- Print
- ZoomIn / ZoomOut
- History
- ToggleAlwaysOnTop
- Help
- OpenRepo
- EditConfig
- DeleteCookies
- Linux / Windows では ToggleMenuBar

`EditConfig` は既存設定ファイルを開く。設定ファイルがない場合は既定設定を生成して開く。

## ウィンドウ

- 初期ウィンドウは非表示で作成され、UI 初期化後に表示される。
- 最小 inner size は 100 x 100 である。
- `window.restore` が有効で保存状態がある場合、位置、サイズ、フルスクリーン、最大化、ズーム倍率を復元する。
- 保存状態からのピン留めは復元せず、起動時は未ピン留めになる。
- `window.default_size` は保存状態がない場合に使われる。
- ズーム倍率は Chrome と同じ段階の 25% から 300% までで、既定は 100% である。

## プラットフォーム差分

| プラットフォーム | 仕様 |
| --- | --- |
| macOS | title bar を隠し、full size content view、transparent titlebar、vibrancy を有効にする。標準ウィンドウボタンは右上へ再配置する。メニューは常に設定される。 |
| Windows | リリース時はコンソールウィンドウを出さない。WebView2 の accelerator key を無効化する。タスクバー icon を設定する。custom protocol URL は `http://shiba.localhost/` 形式で扱う。最小化時に WebView memory usage level を low にする。 |
| Linux | GTK window を使って WebView を構築する。 |
| macOS 以外 | 32px の window icon を設定する。 |

## 根拠ソース

- `src/renderer.rs`
- `src/shiba.rs`
- `src/wry/webview.rs`
- `src/wry/event_loop.rs`
- `src/wry/menu.rs`
- `src/watcher/mod.rs`
- `src/watcher/system.rs`
- `src/watcher/system_linux.rs`
- `src/assets.rs`
- `ui/ipc.ts`
- `ui/dispatcher.ts`

