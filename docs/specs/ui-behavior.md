# UI 操作仕様

## 全体構成

UI は React で構成される。`App` は reducer 状態を持ち、`Preview`、`Search`、`Outline`、`History`、`Guide`、`Welcome`、`Notification` を条件付きで表示する。

アプリ起動時、`App` は mount 後に `init` IPC メッセージを Rust 側へ送る。

## Preview

- Markdown 本文は `<article class="markdown-body">` に表示される。
- `currentPath` が変わった場合、本文スクロール位置は先頭へ戻る。
- 最終変更位置 marker があり、現在の viewport 外にある場合は、その marker を中央付近へスクロールする。
- 見出し一覧は `article > h1,h2,h3,h4,h5,h6` から収集する。
- 現在見出しは、本文スクロール位置以上にある最初の見出し、または viewport 下端を超える場合は直前の見出しとして判定する。

## サイドバー

- サイドバーは画面右側に表示される。
- 既定幅は `150px`、最小幅も `150px` である。
- サイドバーは左辺をドラッグしてリサイズできる。
- `ToggleSideBar` で表示・非表示を切り替える。
- サイドバーの見出し一覧は h2 以上を表示し、h1 は表示しない。
- 見出しの indent は heading level に応じて増える。
- 現在見出しは selected 状態で表示される。
- 見出しをクリックすると該当見出しへスクロールする。
- 現在見出しがサイドバー内の表示範囲外になった場合は自動的にスクロールする。

## サイドバーヘッダー

- ヘッダーには現在ファイル名を表示する。
- tooltip にはフルパスを表示する。
- Windows の `\\?\` prefix は表示時に除去する。
- ピン留め中は pin アイコン、未ピン留め時はアプリ通常アイコンを表示する。
- ヘッダーをクリックするとファイル選択ダイアログを開く。
- メニューボタンはクリック位置を基準に Rust 側へ `open_menu` を送る。

## 検索 UI

- `Search` アクションまたは Rust 側からの `search` メッセージで検索ボックスを開く。
- Welcome 表示中は検索ボックスを表示しない。
- 検索入力は 100ms debounce 後に Rust 側へ `search` メッセージを送る。
- Enter は次の一致、Shift+Enter は前の一致へ移動する。
- Escape または close ボタンで検索を閉じ、空クエリの `search` メッセージを送ってハイライトを解除する。
- 検索件数表示は `現在位置 / 総数` で、未選択または範囲外の場合は `0 / 総数` になる。
- matcher は `MatcherSelect` で変更できる。

## Outline

- `Outline` は現在表示中の `article > h1,h2,h3,h4,h5,h6` を収集して palette に表示する。
- 表示テキストは heading level に応じた `#` prefix と heading text である。
- 選択すると該当見出しへスクロールし、palette を閉じる。

## History

- `History` は UI 状態の recent files を逆順にして palette 表示する。
- ホームディレクトリ配下のパスは `~` 表示に短縮する。
- Windows の `\\?\` prefix は表示時に除去する。
- 選択すると Rust 側へ `open_file` メッセージを送り、palette を閉じる。

## Help

- `Help` は登録済みキーマップからショートカット一覧を表示する。
- キーマップ登録時、同じ action に複数キーが割り当てられている場合はまとめて扱う。
- ショートカット一覧は action 名でソートされ、各 action のキーは単一キー、`+` を含むキー、その他の順にソートされる。

## キーボード操作

キーマップは `mousetrap` で登録される。登録されたキーイベントは `preventDefault` と `stopPropagation` が実行される。

主な action の UI 動作は以下である。

| Action | UI 動作 |
| --- | --- |
| `ScrollDown` / `ScrollUp` | 設定された step で縦スクロールする。 |
| `ScrollLeft` / `ScrollRight` | 対象要素の幅の半分だけ横スクロールする。 |
| `ScrollPageDown` / `ScrollPageUp` | 設定された page step で縦スクロールする。 |
| `ScrollTop` / `ScrollBottom` | 先頭または末尾へスクロールする。 |
| `ScrollNextSection` / `ScrollPrevSection` | 次または前の見出しへ移動する。 |
| `Forward` / `Back` | Rust 側へ履歴移動を要求する。 |
| `Reload` | Rust 側へ再読み込みを要求する。 |
| `OpenFile` / `OpenDir` | Rust 側へファイルまたはディレクトリ選択を要求する。 |
| `Search` | 検索 UI を開く。 |
| `SearchNext` / `SearchPrev` | 検索 UI が開いている場合に現在一致を移動する。 |
| `Outline` / `History` / `Help` | 対応する palette または dialog を開く。 |
| `ZoomIn` / `ZoomOut` | Rust 側へズーム変更を要求する。 |
| `ShowMenu` | メニューボタンをクリックし、存在しない場合は Rust 側へメニュー表示を要求する。 |
| `ToggleMenuBar` | Rust 側へメニューバー切り替えを要求する。 |
| `ToggleAlwaysOnTop` | Rust 側へピン留め切り替えを要求する。 |
| `ToggleSideBar` | UI 側でサイドバー表示を切り替える。 |
| `OpenDevTools` | Rust 側へ DevTools 表示を要求する。 |
| `Quit` | Rust 側へ終了を要求する。 |

スクロール対象は、dialog が開いている場合は dialog content、なければ article、なければ document root である。

## 根拠ソース

- `ui/components/App.tsx`
- `ui/components/Preview.tsx`
- `ui/components/Article.tsx`
- `ui/components/SideBar.tsx`
- `ui/components/Search.tsx`
- `ui/components/Outline.tsx`
- `ui/components/History.tsx`
- `ui/components/MenuButton.tsx`
- `ui/keymaps.ts`
- `ui/reducer.ts`

