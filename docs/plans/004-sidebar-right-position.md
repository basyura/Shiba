# サイドバーを右側へ配置する

## 1. 課題

UI でヘッダーと見出し一覧を表示しているサイドバーが左側に配置されている。
これを本文の右側に表示する。

## 2. 対象箇所の確認

- `ui/components/Preview.tsx` で `Resizable` な `nav` と `Article` を並べている。
- 現状は `nav`、`Divider`、`Article` の順に描画されているため、サイドバーが左側に表示される。
- `NAV_RESIZE_DIRECTION` は右端ドラッグで幅を変更する設定になっている。

## 3. 仕様整理

- 本文を左側、サイドバーを右側に表示する。
- サイドバーの幅変更は、本文側にある左端のドラッグで行う。
- サイドバー内部のヘッダー、メニュー、見出し一覧の挙動は変えない。

## 4. 実装方針

- `Preview` の JSX の描画順を `Article`、`Divider`、`Resizable nav` に変更する。
- サイドバーが右側に移るため、`NAV_RESIZE_DIRECTION` を `left: true`、`right: false` に変更する。
- CSS は既存の `main` の flex レイアウトを利用し、必要最小限の変更に留める。

具体的なコード案:

```tsx
<Article tree={tree} dispatch={dispatch} currentPath={path} key={path ?? 'no-path'} />
<Divider orientation="vertical" />
<Resizable defaultSize={NAV_DEFAULT_SIZE} minWidth="200px" enable={NAV_RESIZE_DIRECTION} as="nav">
    {titleBar && <WindowBar />}
    <SideBar headings={headings} path={path} />
</Resizable>
```

## 5. 影響確認

- サイドバーの表示位置だけが変わることを確認する。
- 見出しクリック、ファイルメニュー、スクロール追従の既存挙動に影響がないことを確認する。
- サイドバーのリサイズ方向が右側配置に合っていることを確認する。

## 6. 動作確認方針

- TypeScript の型チェックを実行する。
- 可能であれば UI を起動して、本文左・サイドバー右の表示を目視確認する。

## 7. 対応内容 (結果)

- `Preview` の描画順を `Article`、`Divider`、`Resizable nav` に変更し、サイドバーを右側に配置した。
- 右側配置に合わせて、サイドバーのリサイズ方向を左端ドラッグへ変更した。
