# Windows 版の多重起動抑止とファイル開き直し

## 目的

Windows 版で Shiba を多重起動せず、既存の Shiba ウィンドウで指定された Markdown ファイルを開き直す。

## 方針

- Windows 限定で名前付き mutex を使い、既存インスタンスの有無を起動直後に判定する。
- 既存インスタンスがある場合、新しいウィンドウを作成せず、起動引数で指定されたプレビュー対象ファイルを既存インスタンスへ送信して終了する。
- `watch_paths` は追加監視対象なので、2 回目起動で既存ウィンドウのプレビューを開き直す対象には含めない。
- 既存インスタンスへの送信には `WM_COPYDATA` を使う。
- `WM_COPYDATA` は `with_msg_hook` ではなく、Shiba ウィンドウの window procedure で直接受信する。
- 既存ウィンドウの特定には Windows の window property を使い、ウィンドウタイトル変更に依存しない。
- 受信したパスは既存の `Event::OpenedFiles` に変換し、既存のファイルオープン処理へ流す。
- 既存ウィンドウへ送信したあと、可能ならウィンドウを前面化する。

## 修正予定ファイル

- `src/windows.rs`
- `src/main.rs`
- `src/wry/event_loop.rs`
- `src/wry/webview.rs`
- `Cargo.toml`

## 検証

- `cargo fmt`
- 可能なら Windows 環境で `cargo test` または `go build` 相当ではなく本リポジトリの Rust ビルドを確認する。
- Windows 固有 API を使うため、少なくとも非 Windows ターゲットでコンパイル対象外になることを確認する。
