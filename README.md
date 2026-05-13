![icon](assets/icon.iconset/icon_64x64.png) Shiba
=================================================

**This project is based on [rhysd/Shiba](https://github.com/rhysd/Shiba).**
This is a fork maintained for my personal use.

- [x] スクロール量の設定
- [x] Markdown ファイルの「このアプリで開く」対応
- [x] ファイル切り替え時のスクロール位置リセット
- [x] サイドバーの右側配置
- [x] サイドバー表示の整理
- [x] macOS ウィンドウボタンの右上配置
- [x] Vim 風スクロールキーの追加
- [x] 現在見出しの強調表示
- [x] F12 で DevTools 表示
- [x] `/` で検索ボックス表示
- [x] `ctrl+l` でサイドバー開閉
- [ ] plantuml 対応
- [ ] ピン留めショートカット
- [ ] エディタで開く

## Summary

[Shiba][shiba] is a simple [Markdown][gh-markdown] preview application to be used with your favorite text editor.
It is designed for simplicity, performance, keyboard-friendliness.

![Screenshot of light/dark windows](https://raw.githubusercontent.com/basyura/Shiba/master/images/main.png)

Features:

- [GitHub-flavored Markdown][gfm] support; Emojis, Table, [Alerts][alerts], Math expressions with [Mathjax][mathjax], Diagrams with
  [mermaid.js][mermaid], ...
- Watch the files or directories and automatically update the preview efficiently using OS-specific filesystem events
  (FSEvents, inotify, ...)
- Automatically scroll to the last modified position
- All features can be accessed via keyboard shortcuts (scroll the article, search text, jump to section, go forward/back history...).
  Type `?` to know all shortcuts
- Sections outline in side navigation bar highlighting the current section
- Both CLI and GUI friendly; Available as a single binary executable as well as a desktop application installed to your system
- Performance critical part (parsing Markdown text, searching Markdown AST, calculating the last modified position, ...) and
  core application logic are written in [Rust][rust]. View logic written in [TypeScript][ts] and [React][react] runs on
  platform-specific WebView
- Cross platform; macOS, Windows, Linux are supported
- Customizable with [a YAML config file](./src/assets/default_config.yml) (color theme, keyboard shortcuts, custom CSS, ...)
- Dogs are respected :dog2:

## License

This software is distributed under [the MIT license](./LICENSE).
This project is based on [rhysd/Shiba](https://github.com/rhysd/Shiba).

[ci]: https://github.com/rhysd/Shiba/actions/workflows/watchdogs.yml
[ci-badge]: https://github.com/rhysd/Shiba/actions/workflows/watchdogs.yml/badge.svg
[shiba-badge]: https://img.shields.io/badge/dogs-respected-brightgreen.svg?longCache=true&style=flat
[shiba]: https://github.com/rhysd/Shiba
[gh-markdown]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax
[gfm]: https://github.github.com/gfm/
[mathjax]: https://www.mathjax.org/
[mermaid]: https://mermaid.js.org/
[rust]: https://www.rust-lang.org/ja
[ts]: https://www.typescriptlang.org/
[react]: https://react.dev/
[v1]: https://github.com/rhysd/Shiba/tree/v1
[alerts]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts

## Build for apple silicon

```bash
cargo build --release --target=aarch64-apple-darwin
bash ./scripts/gen_macos_app.bash（または make Shiba.app）
```
