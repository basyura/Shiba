# Windows Alt+Tab アイコン修正計画

## 背景

Windows 版で Alt+Tab に表示されるアイコンが期待した Shiba アイコンではなく、Windows デフォルトのアイコンになっている。

調査の結果、現在は `WindowBuilder::with_window_icon` により小さいウィンドウアイコンだけが設定されている。Tao の Windows 実装ではこれは `ICON_SMALL` に対応する。一方、Alt+Tab やタスクバーでは `ICON_BIG` が使われるため、ウィンドウ作成後に Windows 専用 API で大きいアイコンを明示的に設定する必要がある。

## 修正方針

- `src/wry/webview.rs` で既存の `with_window_icon` 設定は維持する。
- Windows のみ、`tao::platform::windows::WindowExtWindows` を使い、ウィンドウ作成後に `set_taskbar_icon` を呼ぶ。
- `ICON_BIG` 用には既存の `src/assets/icon_256x256.rgba` を使用する。
- macOS や Linux の挙動は変更しない。

## 確認方法

- `cargo build --release` または `make target/release/shiba.exe` でビルドできることを確認する。
- Windows 上で起動し、Alt+Tab のアイコンが Shiba アイコンになることを確認する。
