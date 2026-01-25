# ファイルオープン時にスクロール位置を先頭へ戻す

## 1. 課題

ファイルパス指定で別のファイルを開いた際に、プレビューのスクロール位置が維持されてしまう。
ファイルを開いた時点で先頭位置にスクロールさせたい。

## 2. 対象箇所の確認

- `ui/dispatcher.ts` の `path_changed` で `pathChanged` を dispatch している。
- `ui/reducer.ts` の `new_path` で `currentPath` を更新している。
- `ui/components/App.tsx` で `currentPath` を `Preview` に渡している。
- `ui/components/Preview.tsx` では `Article` に `tree` と `dispatch` のみ渡している。
- `ui/components/Article.tsx` にスクロール制御があり、ファイル切り替え時の先頭スクロールは未実装。

## 3. 仕様整理

- 新しいファイルを開いた時はプレビューを先頭にスクロールする。
- 同一ファイル内の更新（再レンダリング）では既存の挙動を極力維持する。

## 4. 実装方針

- `Article` でファイル切り替えを検知できるように、`currentPath` を渡す。
- `currentPath` 変更時に `scrollTop = 0` を行う。
- `Preview` 側で `Article` に `key` を付与し、ファイル切り替え時に再マウントさせる。

具体的なコード案:

`ui/components/Article.tsx`
```ts
export interface Props {
    tree: MarkdownReactTree;
    dispatch: Dispatch;
    currentPath: string | null;
}

export const Article: React.FC<Props> = ({ tree, dispatch, currentPath }) => {
    const { root, lastModified } = tree;
    const ref = useRef<HTMLElement>(null);

    useEffect(() => {
        if (ref.current) {
            ref.current.scrollTop = 0;
        }
    }, [currentPath]);
    ...
};
```

`ui/components/Preview.tsx`
```ts
<Article tree={tree} dispatch={dispatch} currentPath={path} key={path ?? 'no-path'} />
```

## 5. 影響確認

- ファイル切り替え時のスクロール位置のみが変わることを確認する。
- 検索・アウトラインなど既存のスクロール操作が影響を受けないことを確認する。

## 6. 動作確認方針

- ファイルパスで別ファイルを開き、スクロール位置が先頭になることを確認する。
- 再読み込みや検索のスクロールが従来通り動作することを確認する。

## 7. 対応内容 (結果)

- `Article` に `currentPath` を追加し、パス変更時に `scrollTop = 0` を実行するようにした。
- `Preview` から `Article` へ `currentPath` を渡すようにした。
  - `Article` 側でファイル切り替えを検知し、先頭スクロールを行うため。
